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

use fnv::FnvHashMap;
use hyper::{Body, Request};
use route_recognizer::Params;
use url::Url;

#[derive(Debug)]
pub struct Context {
    pub req: Request<Body>,
    pub path_params: Params,
    pub url_params: FnvHashMap<String, String>,
}

impl Context {
    pub fn new(req: Request<Body>, path_params: Params) -> Self {
        let mut hashmap = FnvHashMap::default();
        let url = format!("http://localhost{}", req.uri().to_string());
        let url = Url::parse(&url).unwrap();
        let params = url.query_pairs();
        for pair in params {
            let k = String::from(pair.0.as_ref());
            let v = String::from(pair.1.as_ref());
            hashmap.insert(k, v);
        }
        Self {
            req,
            path_params,
            url_params: hashmap,
        }
    }

    pub fn url_params(self, key: &str) -> Option<String> {
        let params = self.url_params.get(key);
        if let Some(params) = params {
            let params = String::from(params);
            return Some(params);
        }
        None
    }

    pub fn path_params(self, key: &str) -> Option<String> {
        let params = self.path_params.find(key);
        if let Some(params) = params {
            let params = String::from(params);
            return Some(params);
        }
        None
    }
}
