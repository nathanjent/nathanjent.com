extern crate futures;
extern crate http;
extern crate route_recognizer;

mod router;
mod handler;
mod errors;

pub use router::RouteBuilder;
pub use handler::Handler;

pub type Result<T> = std::result::Result<T, errors::Error>;
