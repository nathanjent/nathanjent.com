extern crate chrono;
extern crate comrak;
extern crate http;
extern crate query_parse;

mod errors;
mod static_content;

pub use errors::Error;
pub use static_content::get_page;
pub use query_parse::{Query, QueryValue};

pub type Result<T> = std::result::Result<T, errors::Error>;
