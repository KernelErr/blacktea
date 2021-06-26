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
blacktea = "0.1.1"
tokio = { version = "1", features = ["full"] }
# Enable logging
# log = "0.4"
# pretty_env_logger = "0.4"
```

## Example Code

> Code below only suits with version on GitHub, for published version, please refer to Crates.

```rust
use blacktea::{App, HttpResponse, Method, PathParams, Server, URLParams};

async fn url_echo(params: URLParams) -> HttpResponse {
    let params = params.get("msg");
    if let Some(msg) = params {
        HttpResponse::Ok().text(&msg)
    } else {
        HttpResponse::Ok().text("Echo!")
    }
}

async fn path_echo(params: PathParams) -> HttpResponse {
    let params = params.find("msg");
    if let Some(msg) = params {
        HttpResponse::Ok().text(&msg)
    } else {
        HttpResponse::Ok().text("Echo!")
    }
}

#[tokio::main]
async fn main() {
    // Enable logging, set RUST_LOG=info
    // pretty_env_logger::init();
    let mut server = Server::new("127.0.0.1:8080");
    let mut app = App::new();
    // echo?msg=hello
    app.add("/echo", Method::GET, url_echo);
    // echo/hello
    app.add("/echo/:msg", Method::GET, path_echo);
    server.mount("/v1", app);
    server.run().await
}
```

## License

Black Tea is available under  [Apache License 2.0](https://opensource.org/licenses/Apache-2.0), you are also subjected to all dependencies' licenses.
