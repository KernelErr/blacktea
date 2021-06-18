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
use hyper::Method;

#[derive(Debug)]
pub struct Service<F>
where
    F: Future
{
    path: String,
    method: Method,
    handler: dyn Fn() -> F,
}

impl<F> Service<F> {
    pub fn new(path: String, method: Method, handler: dyn Fn() -> F) -> Self {
        Self {
            path,
            method,
            handler,
        }
    }

    pub fn path(self) -> String {
        self.path
    }

    pub fn method(self) -> Method {
        self.method
    }

    pub fn handler(self) -> fn() -> HttpResponse {
        self.handler
    }
}
