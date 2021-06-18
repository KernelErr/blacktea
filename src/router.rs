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

use std::future::Future;
use crate::response::HttpResponse;
use async_trait::async_trait;
use fnv::FnvHashMap;
use hyper::{Method, Response, StatusCode};
use route_recognizer::{Params, Router as InternalRouter};

type HyperResponse = hyper::Response<hyper::Body>;

pub struct Router {
    method_map: FnvHashMap<Method, InternalRouter<Box<dyn Handler>>>,
}

#[async_trait]
pub trait Handler: Send + Sync + 'static {
    async fn invoke(&self) -> HyperResponse;
}

#[async_trait]
impl<F: Send + Sync + 'static, Fut> Handler for F
where
    F: Fn() -> Fut,
    Fut: Future + Send + 'static,
    Fut::Output: IntoResponse,
{
    async fn invoke(&self) -> HyperResponse {
        (self)().await.into_response()
    }
}

pub trait IntoResponse: Send + Sized {
    fn into_response(self) -> HyperResponse;
}

impl IntoResponse for HyperResponse {
    fn into_response(self) -> HyperResponse {
        self
    }
}

impl IntoResponse for HttpResponse {
    fn into_response(self) -> HyperResponse {
        self.res()
    }
}

pub struct RouterMatch<'a> {
    pub handler: &'a dyn Handler,
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
        }
    }

    pub fn add(&mut self, path: &str, method: Method, handler: Box<dyn Handler>) {
        self.method_map
            .entry(method)
            .or_insert_with(InternalRouter::new)
            .add(path, handler);
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
                handler: &***m.handler(),
                params,
            }
        } else {
            RouterMatch {
                handler: &not_found_handler,
                params: Params::new(),
            }
        }
    }
}

async fn not_found_handler() -> HyperResponse {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("NOT FOUND".into())
        .unwrap()
}
