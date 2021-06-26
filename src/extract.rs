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

use crate::context::Context;
use crate::response::HttpResponse;
use hyper::header::HeaderValue;
// use hyper::http::Extensions;
use hyper::{Error, HeaderMap, Version};

pub type HyperResponse = hyper::Response<hyper::Body>;
pub type Header = HeaderMap<HeaderValue>;
pub type URL = hyper::Uri;
pub type HttpVersion = Version;
// pub type ProtocolExtensions = Extensions;
pub type PathParams = route_recognizer::Params;
pub type URLParams = fnv::FnvHashMap<String, String>;

pub trait IntoResponse: Send + Sync {
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

pub trait FromRequest: Sized + Send + Sync + 'static {
    type Error: Into<Error>;

    fn from_request(c: &Context) -> Result<Self, Error>;
}

impl FromRequest for hyper::Method {
    type Error = Error;

    fn from_request(c: &Context) -> Result<Self, Error> {
        Ok(c.req.method().clone())
    }
}

impl FromRequest for URL {
    type Error = Error;

    fn from_request(c: &Context) -> Result<Self, Error> {
        Ok(c.req.uri().clone())
    }
}

impl FromRequest for HttpVersion {
    type Error = Error;

    fn from_request(c: &Context) -> Result<Self, Error> {
        Ok(c.req.version())
    }
}

impl FromRequest for Header {
    type Error = Error;

    fn from_request(c: &Context) -> Result<Self, Error> {
        Ok(c.req.headers().clone())
    }
}

impl FromRequest for PathParams {
    type Error = Error;

    fn from_request(c: &Context) -> Result<Self, Error> {
        Ok(c.path_params.clone())
    }
}

impl FromRequest for URLParams {
    type Error = Error;

    fn from_request(c: &Context) -> Result<Self, Error> {
        Ok(c.url_params.clone())
    }
}

impl FromRequest for () {
    type Error = Error;

    #[inline]
    fn from_request(_: &Context) -> Result<Self, Error> {
        Ok(())
    }
}

macro_rules! from_request_tuple({ $($param:ident),+} => {
    impl< $($param:  FromRequest + 'static,)+ > FromRequest for ($($param,)+) {
        type Error = Error;

        #[inline]
        fn from_request(c: &Context) -> Result<Self, Error> {
            Ok(($($param::from_request(c)?,)+))
        }
    }
});

from_request_tuple!(A);
from_request_tuple!(A, B);
from_request_tuple!(A, B, C);
from_request_tuple!(A, B, C, D);
from_request_tuple!(A, B, C, D, E);
from_request_tuple!(A, B, C, D, E, F);
from_request_tuple!(A, B, C, D, E, F, G);
