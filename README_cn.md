# Black Tea

**积极开发中，请勿用于生产环境。**

Language: [English](./README.md) | [简体中文](./README_cn.md)

> 想来一杯温暖的红茶么？

主页及文档: [Black Tea(搭建中)](https://blacktea.lirui.tech/)	Discord: [Black Tea](https://discord.gg/tfE8RMx8Dr)

Black Tea是一款新兴的Rust后端框架，基于hyper开发。我们致力于为开发者提供增强特性和舒适的开发体验。

## 快速开始

在`Cargo.toml`的依赖中添加：

```toml
[dependencies]
blacktea = "0.1.1"
tokio = { version = "1", features = ["full"] }
# 启用日志
# log = "0.4"
# pretty_env_logger = "0.4"
```

## 样例代码

> 以下代码仅适用于GitHub版本，当前版本代码请参考Crates。

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
    // 启用日志，设置环境变量：RUST_LOG=info
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

## 开源协议

Black Tea以[Apache License 2.0](https://opensource.org/licenses/Apache-2.0)开源，同时您也受制于所有依赖的开源协议。
