# Black Tea

**Under heavy development, can not use in production environment.**

Language: [English](./README.md) | [简体中文](./README_cn.md)

> Would you like to have one cup of warm black tea?

Homepage & Document: [Black Tea(Under Construction)](https://blacktea.lirui.tech/)	Discord: [Black Tea](https://discord.gg/tfE8RMx8Dr)

Black Tea is a new Rust back end framework based on hyper. We are enthusiastic to provide developers some enhanced features and comfortable coding experience.

## Quick Start

Add dependencies in `Cargo.toml`:

```toml
[dependencies]
blacktea = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

## Minimal Example Code

```rust
use blacktea::{Server, HttpResponse, Response, Method};

async fn hello() -> Response {
    HttpResponse::Ok().text("Hello, world!".into())
}

#[tokio::main]
async fn main() {
    let mut server = Server::new("127.0.0.1:8080".into());
    server.service("/hello", Method::GET, Box::new(hello));
    server.run().await
}
```

## Contribution

Currently Black Tea needs your contribution! To be one of us quickly, you can contact with [KernelErr](https://github.com/KernelErr) directly to get a brief view of this project.

## License

Black Tea is available under  [Apache License 2.0](https://opensource.org/licenses/Apache-2.0), you are also subjected to all dependencies' licenses.
