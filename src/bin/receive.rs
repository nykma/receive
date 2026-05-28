use std::net::SocketAddr;
use std::path::PathBuf;

use axum::extract::{DefaultBodyLimit, Multipart};
use axum::response::Html;
use axum::routing::post;
use clap::Parser;
use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    #[arg(long, default_value_t = 200 * 1024 * 1024)]
    max_size: u64,
}

fn format_size(bytes: u64) -> String {
    const KIB: u64 = 1024;
    const MIB: u64 = KIB * 1024;
    const GIB: u64 = MIB * 1024;
    if bytes >= GIB {
        format!("{:.1} GiB", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{} MiB", bytes / MIB)
    } else if bytes >= KIB {
        format!("{} KiB", bytes / KIB)
    } else {
        format!("{} bytes", bytes)
    }
}

async fn upload(mut multipart: Multipart) -> Html<&'static str> {
    while let Ok(Some(mut field)) = multipart.next_field().await {
        let name = field.file_name().unwrap_or("unknown").to_string();
        let path = PathBuf::from(&name);

        let mut file = match tokio::fs::File::create(&path).await {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to create {name}: {e}");
                return Html("<p>Save failed</p>");
            }
        };

        let mut total: u64 = 0;
        while let Some(chunk) = field.next().await {
            match chunk {
                Ok(bytes) => {
                    if let Err(e) = file.write_all(&bytes).await {
                        eprintln!("Failed to write {name}: {e}");
                        return Html("<p>Save failed</p>");
                    }
                    total += bytes.len() as u64;
                }
                Err(e) => {
                    eprintln!("Failed to read field data for {name}: {e}");
                    return Html("<p>Save failed</p>");
                }
            }
        }

        if let Err(e) = file.sync_all().await {
            eprintln!("Failed to sync {name}: {e}");
        }

        println!("Saved: {} ({} bytes)", path.display(), total);
    }
    Html("<p>OK</p>")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));

    println!("Listening on http://{addr}");
    println!("Max upload size: {}", format_size(args.max_size));
    println!("Local IPs:");
    for iface in if_addrs::get_if_addrs()? {
        let ip = iface.ip();
        if ip.is_ipv4() && !ip.is_loopback() {
            println!("  http://{}:{}", ip, args.port);
            println!(
                "  curl -F file=@somefile http://{}:{}/upload",
                ip, args.port
            );
        }
    }

    let app = axum::Router::new()
        .route("/upload", post(upload))
        .layer(DefaultBodyLimit::max(args.max_size as usize));

    let listener = TcpListener::bind(addr).await?;
    println!("Ready.");
    axum::serve(listener, app).await?;

    Ok(())
}
