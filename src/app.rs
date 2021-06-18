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

use crate::response::HttpResponse;
use crate::service::Service;
use hyper::Method;

#[derive(Debug)]
pub struct App{
    mount: String,
    services: Vec<Service>,
}

impl App {
    pub fn new(mount: String) -> Self {
        Self {
            mount,
            services: Vec::new(),
        }
    }

    pub fn service(&mut self, path: String, method: Method, handler: fn() -> HttpResponse) {
        let service = Service::new(path, method, handler);
        self.services.push(service);
    }

    pub fn get_service(self) -> Vec<Service> {
        self.services
    }
}
