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

use crate::extract::FromRequest;
use crate::factory::{factory, BoxServiceFactory, Handler, HandlerService};
use crate::response::HttpResponse;
use hyper::Method;
use std::future::Future;

pub struct App {
    apps: Vec<SubApp>,
}

pub struct SubApp {
    pub path: String,
    pub method: Method,
    pub handler: BoxServiceFactory<HttpResponse>,
}

impl App {
    pub const fn new() -> Self {
        Self { apps: Vec::new() }
    }

    pub fn add<F, T, R>(&mut self, path: &str, method: Method, handler: F)
    where
        F: Handler<T, R>,
        T: FromRequest,
        R: Future<Output = HttpResponse> + Send + Sync + 'static,
    {
        let path = String::from(path);
        let subapp = SubApp::new(path, method, handler);
        self.apps.push(subapp);
    }

    pub fn apps(self) -> Vec<SubApp> {
        self.apps
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl SubApp {
    pub fn new<F, T, R>(path: String, method: Method, handler: F) -> Self
    where
        F: Handler<T, R>,
        T: FromRequest,
        R: Future<Output = HttpResponse> + Send + Sync + 'static,
    {
        Self {
            path,
            method,
            handler: factory(HandlerService::new(handler)),
        }
    }
}
