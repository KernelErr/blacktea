use crate::context::Context;
use crate::response::HttpResponse;
use hyper::Error;
pub type HyperResponse = hyper::Response<hyper::Body>;

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
