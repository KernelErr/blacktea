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
use crate::router::{Handler, Router};
use hyper::server::conn::AddrStream;
use hyper::service::make_service_fn;
use hyper::service::service_fn;
use hyper::Body;
use hyper::Method;
use hyper::Request;
use hyper::Server as HyperServer;
use core::str;
use std::net::SocketAddr;
use std::sync::Arc;

type HttpResponse = hyper::Response<hyper::Body>;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub struct Server {
    addr: SocketAddr,
    router: Router,
}

impl Server {
    #[inline]
    pub fn new(addr: String) -> Self {
        let addr: SocketAddr = addr.parse().expect("Failed to parse server address.");
        Self {
            addr,
            router: Router::new(),
        }
    }

    pub fn service(&mut self, path: &str, method: Method, handler: Box<dyn Handler>) {
        self.router.add(path, method, handler);
    }

    pub fn mount(&mut self, mount_point: &str, app: App) {
        for x in app.apps() {
            let subapp = *x;
            let path = String::from(subapp.path);
            let path = format!("{}{}", mount_point, path);
            let method = subapp.method;
            let handler = subapp.handler;
            self.router.add(&path, method, handler);
        }
    }

    pub async fn run(self) {
        let shared_router = Arc::new(self.router);
        let service = make_service_fn(move |conn: &AddrStream| {
            let addr: String = conn.remote_addr().to_string();
            let router_capture = shared_router.clone();
            async {
                Ok::<_, Error>(service_fn(move |req| route(router_capture.clone(), req, addr.clone())))
            }
        });
        let server = HyperServer::bind(&self.addr).serve(service);
        info!("Listening on http://{}", self.addr);
        let _ = server.await;
    }
}

async fn route(router: Arc<Router>, req: Request<Body>, addr: String) -> Result<HttpResponse, Error> {
    info!("{} {} {}", req.method(), req.uri(), addr);
    let found_handler = router.route(req.uri().path(), req.method());
    let res = found_handler.handler.invoke().await;
    Ok(res)
}
