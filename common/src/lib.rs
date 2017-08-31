extern crate chrono;
extern crate comrak;
extern crate http;

mod errors;
mod request;
mod static_content;

pub use errors::Error;
pub use static_content::get_page;
pub use request::{build_request_from_env, route};

pub type Result<T> = std::result::Result<T, errors::Error>;
