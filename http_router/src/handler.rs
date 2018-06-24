use std::error::Error;

use std::io;
use std::rc::Rc;
use std::sync::Arc;

use futures::Future;
use http::{Request, Response};

/// A Handler takes a request and returns a response or an error.
/// By default, a bare function implements `Handler`.
pub trait Handler<I, O>: Sync + Send + 'static
{
    fn call(&self, request: &mut Request<I>) -> Result<Response<O>, Box<Error+Send>>;
}

impl<I, O, F, E> Handler<I, O> for F
where F: Fn(&mut Request<I>) -> Result<Response<O>, E> + Sync + Send + 'static,
      E: Error + Send + 'static
{
    fn call(&self, request: &mut Request<I>) -> Result<Response<O>, Box<Error+Send>> {
        (*self)(request).map_err(|e| Box::new(e) as Box<Error+Send>)
    }
}

pub trait Service {

    /// Requests handled by the service.
    type Request;

    /// Responses given by the service.
    type Response;

    /// Errors produced by the service.
    type Error;

    /// The future response value.
    type Future: Future<Item = Self::Response, Error = Self::Error>;

    /// Process the request and return the response asynchronously.
    fn call(&self, req: Self::Request) -> Self::Future;
}

/// Creates new `Service` values.
pub trait NewService {
    /// Requests handled by the service
    type Request;

    /// Responses given by the service
    type Response;

    /// Errors produced by the service
    type Error;

    /// The `Service` value created by this factory
    type Instance: Service<Request = Self::Request, Response = Self::Response, Error = Self::Error>;

    /// Create and return a new service value.
    fn new_service(&self) -> io::Result<Self::Instance>;
}

impl<F, R> NewService for F
    where F: Fn() -> io::Result<R>,
          R: Service,
{
    type Request = R::Request;
    type Response = R::Response;
    type Error = R::Error;
    type Instance = R;

    fn new_service(&self) -> io::Result<R> {
        (*self)()
    }
}

impl<S: NewService + ?Sized> NewService for Arc<S> {
    type Request = S::Request;
    type Response = S::Response;
    type Error = S::Error;
    type Instance = S::Instance;

    fn new_service(&self) -> io::Result<S::Instance> {
        (**self).new_service()
    }
}

impl<S: NewService + ?Sized> NewService for Rc<S> {
    type Request = S::Request;
    type Response = S::Response;
    type Error = S::Error;
    type Instance = S::Instance;

    fn new_service(&self) -> io::Result<S::Instance> {
        (**self).new_service()
    }
}

impl<S: Service + ?Sized> Service for Box<S> {
    type Request = S::Request;
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn call(&self, request: S::Request) -> S::Future {
        (**self).call(request)
    }
}

impl<S: Service + ?Sized> Service for Rc<S> {
    type Request = S::Request;
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn call(&self, request: S::Request) -> S::Future {
        (**self).call(request)
    }
}

impl<S: Service + ?Sized> Service for Arc<S> {
    type Request = S::Request;
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn call(&self, request: S::Request) -> S::Future {
        (**self).call(request)
    }
}
