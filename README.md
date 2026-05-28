# receive

A minimal HTTP file transfer toolkit written in Rust.

## Commands

### `receive` — file upload server

```bash
cargo run --bin receive                 # listen on :8080
cargo run --bin receive -- -p 3000      # custom port
cargo run --bin receive -- --max-size 1073741824  # custom max upload size (1 GiB)
```

Upload a file:

```bash
curl -F file=@somefile http://localhost:8080/upload
```

| Flag | Description | Default |
|------|-------------|---------|
| `-p`, `--port` | Port to listen on | `8080` |
| `--max-size` | Max upload size in bytes | `209715200` (200 MiB) |

### `serve` — static file serving

Exposes the current working directory as a static file tree.

```bash
cargo run --bin serve              # serve CWD on :8080
cargo run --bin serve -- -p 3000   # custom port
```

| Flag | Description | Default |
|------|-------------|---------|
| `-p`, `--port` | Port to listen on | `8080` |

On startup, all available IP addresses and URLs are printed to the console.

## Nix flake

Run directly:

```bash
nix run github:nykma/receive         # receive (file upload)
nix run github:nykma/receive#serve   # serve (static files)
nix run github:nykma/receive -- -p 3000
```

Add to your flake inputs:

```nix
{
  inputs.receive.url = "github:nykma/receive";
  # ...
}
```

Then use: `inputs.receive.packages.${system}.receive` or `inputs.receive.packages.${system}.serve`.

## License

MIT
