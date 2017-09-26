//! Router logic borrowed from Conduit-Router crate.
use std::collections::hash_map::{HashMap, Entry};
use std::error::Error;
use std::fmt;
use std::io::{Read, Write};

use route_recognizer::{Router, Match, Params};
use http::{Method, Request, Response};

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

pub struct RouteBuilder<I, O> {
    routers: HashMap<Method, Router<Box<Handler<I, O>>>>,
}

#[derive(Debug)]
pub struct RouterError(String);

impl<I, O> RouteBuilder<I, O>
{
    pub fn new() -> RouteBuilder<I,O> {
        RouteBuilder { routers: HashMap::new() }
    }

    pub fn recognize<'a>(&'a self, method: &Method, path: &str)
                         -> Result<Match<&'a Box<Handler<I, O>>>,
                                   RouterError>
    {
        match self.routers.get(method) {
            Some(router) => router.recognize(path),
            None => Err(format!("No router found for {:?}", method)),
        }.map_err(RouterError)
    }

    pub fn map<'a, H: Handler<I, O>>(&'a mut self, method: Method, pattern: &str,
                               handler: H) -> &'a mut RouteBuilder<I, O> {
        {
            let router = match self.routers.entry(method) {
                Entry::Occupied(e) => e.into_mut(),
                Entry::Vacant(e) => e.insert(Router::new()),
            };
            router.add(pattern, Box::new(handler));
        }
        self
    }

    pub fn get<'a, H: Handler<I, O>>(&'a mut self, pattern: &str, handler: H)
                               -> &'a mut RouteBuilder<I, O> {
        self.map(Method::GET, pattern, handler)
    }

    pub fn post<'a, H: Handler<I, O>>(&'a mut self, pattern: &str, handler: H)
                                -> &'a mut RouteBuilder<I, O> {
        self.map(Method::POST, pattern, handler)
    }

    pub fn put<'a, H: Handler<I, O>>(&'a mut self, pattern: &str, handler: H)
                               -> &'a mut RouteBuilder<I, O> {
        self.map(Method::PUT, pattern, handler)
    }

    pub fn delete<'a, H: Handler<I, O>>(&'a mut self, pattern: &str, handler: H)
                                  -> &'a mut RouteBuilder<I, O> {
        self.map(Method::DELETE, pattern, handler)
    }

    pub fn head<'a, H: Handler<I, O>>(&'a mut self, pattern: &str, handler: H)
                                -> &'a mut RouteBuilder<I, O> {
        self.map(Method::HEAD, pattern, handler)
    }
}

impl<I,O> Handler<I,O> for RouteBuilder<I,O>
where I: 'static,
      O: 'static
{
    fn call(&self, request: &mut Request<I>) -> Result<Response<O>, Box<Error+Send>> {
        let m = {
            let method = request.method();
            let path = request.uri().path_and_query().unwrap().path();

            match self.recognize(&method, path) {
                Ok(m) => m,
                Err(e) => return Err(Box::new(e) as Box<Error+Send>)
            }
        };

        {
            let extensions = request.extensions_mut();
            extensions.insert(m.params.clone());
        }

        (*m.handler).call(request)
    }
}

impl Error for RouterError {
    fn description(&self) -> &str { &self.0 }
}

impl fmt::Display for RouterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

pub trait RequestParams<'a> {
    fn params(self) -> &'a Params;
}

pub fn params<'a, I>(req: &'a Request<I>) -> &'a Params
{
    req.extensions().get::<Params>()
        .expect("Missing params")
}

impl<'a, I> RequestParams<'a> for &'a Request<I>
{
    fn params(self) -> &'a Params {
        params(self)
    }
}
