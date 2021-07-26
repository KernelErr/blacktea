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
use crate::extract::{FromRequest, IntoResponse};
use crate::response::HttpResponse;
use std::boxed::Box;
use std::future::Future;
use std::future::{ready, Ready};
use std::marker::PhantomData;
use std::pin::Pin;

pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + Sync>>;

pub trait Service: Send + Sync {
    type Response;
    type Future: Future<Output = Self::Response> + Send + Sync;

    fn call(&self, c: Context) -> Self::Future;
}

pub trait ServiceFactory: Send + Sync {
    type Response;

    type Future: Future<Output = Self::Service>;

    type Service: Service<Response = Self::Response>;

    fn new_service(&self) -> Self::Future;
}

pub type BoxService<Res> = Box<dyn Service<Response = Res, Future = BoxFuture<Res>>>;

impl<Res> Service for BoxService<Res> {
    type Response = Res;
    type Future = BoxFuture<Res>;

    #[inline]
    fn call(&self, c: Context) -> Self::Future {
        (**self).call(c)
    }
}

pub fn service<S>(service: S) -> BoxService<S::Response>
where
    S: Service + 'static,
    S::Future: 'static,
{
    Box::new(ServiceWrapper::new(service))
}

struct ServiceWrapper<S> {
    inner: S,
}

impl<S> ServiceWrapper<S> {
    const fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, Res> Service for ServiceWrapper<S>
where
    S: Service<Response = Res>,
    S::Future: 'static,
{
    type Response = Res;
    type Future = BoxFuture<Res>;

    // crate::forward_ready!(inner);

    fn call(&self, c: Context) -> Self::Future {
        Box::pin(self.inner.call(c))
    }
}

struct FactoryWrapper<SF>(SF);

impl<SF, Res> ServiceFactory for FactoryWrapper<SF>
where
    Res: 'static,
    SF: ServiceFactory<Response = Res>,
    SF::Future: Send + Sync + 'static,
    SF::Service: 'static,
    <SF::Service as Service>::Future: 'static,
{
    type Response = Res;
    type Service = BoxService<Res>;
    type Future = BoxFuture<Self::Service>;

    fn new_service(&self) -> Self::Future {
        let f = self.0.new_service();
        Box::pin(async { Box::new(ServiceWrapper::new(f.await)) as _ })
    }
}

pub struct BoxServiceFactory<Res>(
    Box<
        dyn ServiceFactory<
            Response = Res,
            Service = BoxService<Res>,
            Future = BoxFuture<BoxService<Res>>,
        >,
    >,
);

impl<Res> ServiceFactory for BoxServiceFactory<Res> {
    type Response = Res;
    type Service = BoxService<Res>;
    type Future = BoxFuture<Self::Service>;

    fn new_service(&self) -> Self::Future {
        self.0.new_service()
    }
}

pub fn factory<SF, Res>(factory: SF) -> BoxServiceFactory<Res>
where
    SF: ServiceFactory<Response = Res> + 'static,
    Res: 'static,
    SF::Response: 'static,
    SF::Service: 'static,
    SF::Future: 'static + Send + Sync,
{
    BoxServiceFactory(Box::new(FactoryWrapper(factory)))
}

pub trait Handler<T, R>: Send + Sync + Clone + 'static
where
    R: Future,
    R::Output: IntoResponse,
{
    fn call(&self, param: T) -> R;
}

pub struct HandlerService<F, T, R>
where
    F: Handler<T, R>,
    T: FromRequest,
    R: Future,
    R::Output: IntoResponse,
{
    hnd: F,
    _phantom: PhantomData<(T, R)>,
}

impl<F, T, R> HandlerService<F, T, R>
where
    F: Handler<T, R>,
    T: FromRequest,
    R: Future,
    R::Output: IntoResponse,
{
    pub fn new(hnd: F) -> Self {
        Self {
            hnd,
            _phantom: PhantomData,
        }
    }
}

impl<F, T, R> Clone for HandlerService<F, T, R>
where
    F: Handler<T, R>,
    T: FromRequest,
    R: Future,
    R::Output: IntoResponse,
{
    fn clone(&self) -> Self {
        Self {
            hnd: self.hnd.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<F, T, R> ServiceFactory for HandlerService<F, T, R>
where
    F: Handler<T, R>,
    T: FromRequest,
    R: Future<Output = HttpResponse> + Send + Sync + 'static,
{
    type Response = HttpResponse;
    type Service = Self;
    type Future = Ready<Self::Service>;

    fn new_service(&self) -> Self::Future {
        ready(self.clone())
    }
}

/// HandlerService is both it's ServiceFactory and Service Type.
impl<F, T, R> Service for HandlerService<F, T, R>
where
    F: Handler<T, R>,
    T: FromRequest,
    R: Future<Output = HttpResponse> + Send + Sync + 'static,
{
    type Response = HttpResponse;
    type Future = BoxFuture<HttpResponse>;

    fn call(&self, c: Context) -> Self::Future {
        let data = T::from_request(&c).ok().unwrap();
        Box::pin(self.hnd.call(data))
    }
}

impl<F, R> Handler<(), R> for F
where
    F: Fn() -> R + Send + Sync + Clone + 'static,

    R: Future,
    R::Output: IntoResponse,
{
    fn call(&self, _: ()) -> R {
        (self)()
    }
}
macro_rules! factory_tuple({ $(($n: tt, $param:ident)),+} => {
    impl<F, $($param: ,)+ R> Handler<($($param,)+), R> for F
    where
        F: Fn($($param,)+) -> R + Send + Sync + Clone + 'static,

        R: Future,
        R::Output: IntoResponse,
    {
        fn call(&self, param: ($($param,)+)) -> R {
            (self)($(param.$n,)+)
        }
    }
});

factory_tuple!((0, P1));
factory_tuple!((0, P1), (1, P2));
factory_tuple!((0, P1), (1, P2), (2, P3));
factory_tuple!((0, P1), (1, P2), (2, P3), (3, P4));
factory_tuple!((0, P1), (1, P2), (2, P3), (3, P4), (4, P5));
factory_tuple!((0, P1), (1, P2), (2, P3), (3, P4), (4, P5), (5, P6));
