use http::Error as HttpError;
use std::error::Error as StdError;
use std::io::Error as IoError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    HttpError(HttpError),
    IoError(IoError),
}


impl From<HttpError> for Error {
    fn from(error: HttpError) -> Self {
        Error::HttpError(error)
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        Error::IoError(error)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::HttpError(_) => "http error",
            Error::IoError(_) => "io error",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::HttpError(ref msg) => write!(fmt, "{}", msg),
            Error::IoError(ref msg) => write!(fmt, "{}", msg),
        }
    }
}

