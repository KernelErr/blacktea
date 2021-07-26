// Copyright 2021 Black Tea Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::app::App;
use crate::context::Context;
use crate::extract::{FromRequest, IntoResponse};
use crate::factory::{Handler, ServiceFactory};
use crate::response::HttpResponse;
use crate::router::Router;
use core::str;
use hyper::server::conn::AddrStream;
use hyper::service::make_service_fn;
use hyper::service::service_fn;
use hyper::Body;
use hyper::Method;
use hyper::Request;
use hyper::Server as HyperServer;
use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;

type Response = hyper::Response<hyper::Body>;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

/// https://github.com/hyperium/hyper/blob/19f38b3e7febadedbfc558d17fa41baff73c6ecc/src/common/exec.rs#L28-L57
pub enum Executor {
    Tokio,
    #[cfg(feature = "tokio_io_uring")]
    TokioUring,
}

impl Default for Executor {
    fn default() -> Self {
        Self::Tokio
    }
}

#[cfg(feature = "tokio_io_uring")]
#[derive(Clone)]
struct TokioUringExecutor;

#[cfg(feature = "tokio_io_uring")]
impl<F> hyper::rt::Executor<F> for TokioUringExecutor
where
    F: Future + 'static,
    F::Output: 'static,
{
    fn execute(&self, fut: F) {
        tokio_uring::spawn(fut);
    }
}

pub struct Server {
    addr: SocketAddr,
    router: Router,
    executor: Executor,
}

impl Server {
    #[inline]
    pub fn new(addr: &str) -> Self {
        let addr = String::from(addr);
        let addr: SocketAddr = addr.parse().expect("Failed to parse server address.");
        Self {
            addr,
            router: Router::new(),
            executor: Executor::default(),
        }
    }

    pub fn set_executor(&mut self, executor: Executor) {
        self.executor = executor;
    }

    pub fn service<F, T, R>(&mut self, path: &str, method: Method, handler: F)
    where
        F: Handler<T, R> + Send + Sync + 'static,
        T: FromRequest + Send + Sync + 'static,
        R: Future<Output = HttpResponse> + Send + Sync + 'static,
    {
        self.router.add(path, method, handler);
    }

    pub fn mount(&mut self, mount_point: &str, app: App) {
        for subapp in app.apps() {
            self.router.mount(mount_point, subapp);
        }
    }

    pub async fn run(self) {
        let shared_router = Arc::new(self.router);
        match self.executor {
            Executor::Tokio => {
                let service = make_service_fn(move |conn: &AddrStream| {
                    let addr: String = conn.remote_addr().to_string();
                    let router_capture = shared_router.clone();
                    async {
                        Ok::<_, Error>(service_fn(move |req| {
                            route(router_capture.clone(), addr.clone(), req)
                        }))
                    }
                });
                let server = HyperServer::bind(&self.addr).serve(service);
                info!("Listening on http://{}", self.addr);
                let _ = server.await;
            }
            #[cfg(feature = "tokio_io_uring")]
            Executor::TokioUring => {
                let listener = tokio::net::TcpListener::bind(self.addr)
                    .await
                    .expect("bind failed");
                let server = hyper::server::conn::Http::new().with_executor(TokioUringExecutor);
                while let Ok((stream, addr)) = listener.accept().await {
                    let router_capture = shared_router.clone();
                    server
                        .serve_connection(
                            stream,
                            service_fn(move |req| {
                                route(router_capture.clone(), addr.to_string(), req)
                            }),
                        )
                        .await
                        .expect("error in serve_connection");
                }
            }
        }
    }
}

async fn route(router: Arc<Router>, addr: String, req: Request<Body>) -> Result<Response, Error> {
    info!("{} {} {}", req.method(), req.uri(), addr);
    let found_handler = router.route(req.uri().path(), req.method());
    let res = found_handler
        .handler
        .new_service()
        .await
        .call(Context::new(req, found_handler.params))
        .await
        .into_response();
    Ok(res)
}
