# receive

一个用 Rust 编写的极简 HTTP 文件传输工具集。

## 命令

### `receive` — 文件上传服务器

```bash
cargo run --bin receive                 # 监听 :8080
cargo run --bin receive -- -p 3000      # 自定义端口
cargo run --bin receive -- --max-size 1073741824  # 自定义最大上传大小（1 GiB）
```

上传文件：

```bash
curl -F file=@somefile http://localhost:8080/upload
```

| 参数 | 说明 | 默认值 |
|------|------|--------|
| `-p`, `--port` | 监听端口 | `8080` |
| `--max-size` | 最大上传大小（字节） | `209715200`（200 MiB） |

### `serve` — 静态文件托管

将当前工作目录作为静态文件树暴露。

```bash
cargo run --bin serve              # 托管当前目录，监听 :8080
cargo run --bin serve -- -p 3000   # 自定义端口
```

| 参数 | 说明 | 默认值 |
|------|------|--------|
| `-p`, `--port` | 监听端口 | `8080` |

启动时会在控制台打印所有可用的 IP 地址及 URL。

## Nix flake

直接运行：

```bash
nix run github:nykma/receive         # receive（文件上传）
nix run github:nykma/receive#serve   # serve（静态文件托管）
nix run github:nykma/receive -- -p 3000
```

加入你的 flake inputs：

```nix
{
  inputs.receive.url = "github:nykma/receive";
  # ...
}
```

然后使用：`inputs.receive.packages.${system}.receive` 或 `inputs.receive.packages.${system}.serve`。

## 许可证

MIT
