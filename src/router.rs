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

use crate::app::SubApp;
use crate::extract::FromRequest;
use crate::factory::{factory, BoxServiceFactory, Handler, HandlerService};
use crate::response::HttpResponse;
use fnv::FnvHashMap;
use hyper::{Method, Response, StatusCode};
use route_recognizer::{Params, Router as InternalRouter};
use std::future::Future;

pub struct Router {
    method_map: FnvHashMap<Method, InternalRouter<BoxServiceFactory<HttpResponse>>>,

    not_found_handler: BoxServiceFactory<HttpResponse>,
}

pub struct RouterMatch<'a> {
    pub handler: &'a BoxServiceFactory<HttpResponse>,
    pub params: Params,
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

impl Router {
    pub fn new() -> Self {
        Self {
            method_map: FnvHashMap::default(),
            not_found_handler: factory(HandlerService::new(not_found_handler)),
        }
    }

    pub fn add<F, T, R>(&mut self, path: &str, method: Method, handler: F)
    where
        F: Handler<T, R>,
        T: FromRequest,
        R: Future<Output = HttpResponse> + Send + Sync + 'static,
    {
        self.method_map
            .entry(method)
            .or_insert_with(InternalRouter::new)
            .add(path, factory(HandlerService::new(handler)));
    }

    pub fn mount(&mut self, path: &str, sub_app: SubApp) {
        self.method_map
            .entry(sub_app.method)
            .or_insert_with(InternalRouter::new)
            .add(&format!("{}{}", path, sub_app.path), sub_app.handler);
    }

    pub fn route(&self, path: &str, method: &Method) -> RouterMatch {
        if let Some(m) = self
            .method_map
            .get(method)
            .and_then(|r| r.recognize(path).ok())
        {
            let mut params = Params::new();
            params.clone_from(m.params());
            RouterMatch {
                handler: m.handler(),
                params,
            }
        } else {
            RouterMatch {
                handler: &self.not_found_handler,
                params: Params::new(),
            }
        }
    }
}

async fn not_found_handler() -> HttpResponse {
    HttpResponse::from_builder(
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("NOT FOUND".into())
            .unwrap(),
    )
}
