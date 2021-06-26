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
