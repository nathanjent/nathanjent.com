extern crate chrono;
extern crate comrak;
extern crate diesel;
extern crate http;
extern crate query_parse;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_file_unix;

mod errors;
mod request;
mod response;
mod static_content;

pub use errors::Error;
pub use static_content::get_page;
pub use request::{build_request_from_env};
pub use response::{send_response};
pub use query_parse::{Query, QueryValue};

pub type Result<T> = std::result::Result<T, errors::Error>;
