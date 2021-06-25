use blacktea::{App, HttpResponse, Method, Server};

async fn url_echo(method: Method) -> HttpResponse {
    HttpResponse::Ok().text(method.as_str())
}

/* async fn path_echo(cxt: Context) -> HttpResponse {
    let params = cxt.path_params("msg");
    if let Some(msg) = params {
        HttpResponse::Ok().text(&msg)
    } else {
        HttpResponse::Ok().text("Echo!")
    }
} */

#[tokio::main]
async fn main() {
    // Enable logging, set RUST_LOG=info
    // pretty_env_logger::init();
    let mut server = Server::new("127.0.0.1:8080");
    let mut app = App::new();
    // echo?msg=hello
    app.add("/echo", Method::GET, url_echo);
    // echo/hello
    // app.add("/echo/:msg", Method::GET, Box::new(path_echo));
    server.mount("/v1", app);
    server.run().await
}
