extern crate regex;
extern crate http;

mod router;
mod errors;

pub type Result<T> = std::result::Result<T, errors::Error>;
