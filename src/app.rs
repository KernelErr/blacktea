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

use hyper::Method;
use crate::router::Handler;

pub struct App {
    apps: Vec<Box<SubApp>>,
}

pub struct SubApp {
    pub path: String,
    pub method: Method,
    pub handler: Box<dyn Handler>,
}

impl App {
    pub fn new() -> Self {
        Self {
            apps: Vec::new(),
        }
    }

    pub fn add(&mut self, path: &str, method: Method, handler: Box<dyn Handler>) {
        let path = String::from(path);
        let subapp = SubApp::new(path, method, handler);
        self.apps.push(Box::new(subapp));
    }

    pub fn apps(self) -> Vec<Box<SubApp>> {
        self.apps
    }
}

impl SubApp {
    pub fn new(path: String, method: Method, handler: Box<dyn Handler>) -> Self {
        Self {
            path,
            method,
            handler
        }
    }

    pub fn path(self) -> String {
        self.path
    }

    pub fn method(self) -> Method {
        self.method
    }

    pub fn handler(self) -> Box<dyn Handler> {
        self.handler
    }
}