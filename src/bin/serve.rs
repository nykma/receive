use std::net::SocketAddr;

use axum::routing::get_service;
use clap::Parser;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    let root = std::env::current_dir()?;

    println!("Serving files from: {}", root.display());
    println!("Available URLs:");
    for iface in if_addrs::get_if_addrs()? {
        let ip = iface.ip();
        if ip.is_ipv4() && !ip.is_loopback() {
            println!("  http://{}:{}", ip, args.port);
        }
    }

    let app = get_service(ServeDir::new(&root));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{addr}");
    axum::serve(listener, app).await?;

    Ok(())
}
