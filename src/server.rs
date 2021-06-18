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
use crate::router::Router;
use std::net::SocketAddr;

pub struct Server{
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

    #[inline]
    pub fn mount(&mut self, app: App) {
        for i in app.get_service() {
            self.router.add(i);
        }
    }

    #[inline]
    pub fn run(self) {
        
    }
}