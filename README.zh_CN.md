# receive

一个用 Rust 编写的极简 HTTP 文件上传服务器。

## 用法

```bash
cargo run                   # 监听 :8080
cargo run -- -p 3000        # 自定义端口
cargo run -- --max-size 1073741824  # 自定义最大上传大小（1 GiB）
```

上传文件：

```bash
curl -F file=@somefile http://localhost:8080/upload
```

## 参数

| 参数 | 说明 | 默认值 |
|------|------|--------|
| `-p`, `--port` | 监听端口 | `8080` |
| `--max-size` | 最大上传大小（字节） | `209715200`（200 MiB） |

## 工作原理

- 在 `0.0.0.0:$PORT` 上启动 HTTP 服务器。
- 接受 `POST /upload` 的 multipart 表单数据。
- 将上传的文件以原始文件名保存到**当前工作目录**。

## Nix flake

直接运行：

```bash
nix run github:nykma/receive
nix run github:nykma/receive -- -p 3000
```

加入你的 flake inputs：

```nix
{
  inputs.receive.url = "github:nykma/receive";
  # ...
}
```

然后使用：`inputs.receive.packages.${system}.receive`。

## 许可证

MIT
