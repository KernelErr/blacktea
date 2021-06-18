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
blacktea = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

## 最小样例代码

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

## 项目贡献

目前Black Tea非常欢迎大家的贡献，为了帮助您能够快速加入我们的开发，您可以直接联系[KernelErr](https://github.com/KernelErr)了解本项目。

## 开源协议

Black Tea以[Apache License 2.0](https://opensource.org/licenses/Apache-2.0)开源，同时您也受制于所有依赖的开源协议。
