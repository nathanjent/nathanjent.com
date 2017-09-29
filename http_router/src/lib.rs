extern crate http;
extern crate route_recognizer;

mod router;
mod errors;

pub use router::{Handler, RouteBuilder};

pub type Result<T> = std::result::Result<T, errors::Error>;
