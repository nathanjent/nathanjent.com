extern crate chrono;
extern crate comrak;
extern crate diesel;
extern crate http;

mod errors;
mod request;
mod query;
mod static_content;

pub use errors::Error;
pub use static_content::get_page;
pub use request::{build_request_from_env, route};
pub use query::{Query, QueryValue};

pub type Result<T> = std::result::Result<T, errors::Error>;
