# AGENTS.md

## Dev environment
- Nix flake + direnv: the flake provides the Rust toolchain (fenix stable) plus clippy, rustfmt, cargo-deny, cargo-watch.
- If not using Nix, anything with `cargo` and a recent stable Rust works.

## Build & run
```bash
cargo build
cargo run                # listens on :8080
cargo run -- -p 3000     # custom port
```

## Lint & format
```bash
cargo clippy
cargo fmt --check
```

## Architecture
- Single binary crate, `src/main.rs` is the only source file.
- `Cargo.toml` uses edition 2021.
- `axum` requires the `multipart` feature — don't remove it.
- Uploaded files are saved to the **current working directory** with the original filename.