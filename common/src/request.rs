use http::{Request, Version};
use std::env;
use std::io;

use super::Result;

pub fn build_request_from_env() -> Result<Request<io::Stdin>> {
    let mut request_builder = Request::builder();
    let mut content_length = 0;

    for (k, v) in  env::vars() {
        match &*k {
            "CONTENT_LENGTH" => {
                if let Ok(var) = v.parse::<u64>() {
                    content_length = var;
                }
            },
            "REQUEST_METHOD" => {
                request_builder.method(&*v);
            },
            "SERVER_PROTOCOL" => {
                let version = match &*v {
                    "HTTP/0.9" => Version::HTTP_09,
                    "HTTP/1.0" | "HTTP/1" => Version::HTTP_10,
                    "HTTP/1.1" => Version::HTTP_11,
                    "HTTP/2.0" | "HTTP/2" => Version::HTTP_2,
                    _ => Version::HTTP_10,
                };
                request_builder.version(version);
            },
            "REQUEST_URI" => {
                request_builder.uri(&*v);
            },
            _ => {
                if let Some(k) = k.split("HTTP_").nth(1) {
                    request_builder.header(k, &*v);
                }
            },
        }
    }
    
    let request: Request<io::Stdin> = request_builder.body(io::stdin())?;
    Ok(request)
}
