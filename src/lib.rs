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

// #![deny(warnings)]
#[macro_use]
extern crate log;
mod app;
pub mod context;
mod response;
pub mod router;
mod server;

mod extract;
pub mod factory;

pub use self::app::App;
pub use self::context::Context;
pub use self::extract::HyperResponse;
pub use self::response::HttpResponse;
pub use self::server::Server;
pub use extract::{Header, HttpVersion, PathParams, URLParams, URL};
pub use hyper::Method;
