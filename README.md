# receive

A minimal HTTP file upload server written in Rust.

## Usage

```bash
cargo run                 # listen on :8080
cargo run -- -p 3000      # custom port
cargo run -- --max-size 1073741824  # custom max upload size (1 GiB)
```

Upload a file:

```bash
curl -F file=@somefile http://localhost:8080/upload
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-p`, `--port` | Port to listen on | `8080` |
| `--max-size` | Max upload size in bytes | `209715200` (200 MiB) |

## How it works

- Starts an HTTP server on `0.0.0.0:$PORT`.
- Accepts `POST /upload` with multipart form data.
- Saves uploaded files to the **current working directory** using their original filenames.

## Nix flake

Run directly:

```bash
nix run github:nykma/receive
nix run github:nykma/receive -- -p 3000
```

Add to your flake inputs:

```nix
{
  inputs.receive.url = "github:nykma/receive";
  # ...
}
```

Then use it: `inputs.receive.packages.${system}.receive`.

## License

MIT
