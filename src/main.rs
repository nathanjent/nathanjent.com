extern crate common;
extern crate dotenv;
extern crate http;
extern crate serde;

use dotenv::dotenv;
use http::Response;
use std::io::{self, Write};

fn main() {
    dotenv().ok();
    //println!("{:?}", ::std::env::vars().collect::<Vec<_>>());

    let status = match handle() {
        Ok(_) => 0,
        Err(e) => {
            writeln!(io::stdout(),
                     "Status: 500\r\n\r\n
                     <h1>500 Internal Server \
                      Error</h1>
                     <p>{}</p>",
                     e)
                .expect("Panic writing Server Error to STDOUT!");
            1
        }
    };
    ::std::process::exit(status);
}

fn handle() -> common::Result<()> {
    if let Ok(request) = common::build_request_from_env(io::stdin()) {
        //println!("{:?}", request);
        common::route(&request, io::stdout(), || {
            let default_response = Response::builder()
                .status(http::status::NOT_FOUND)
                .body(&b""[..])
                .unwrap();
            match (request.method(), request.uri().path(), request.uri().query()) {
                (&http::method::GET, "/", None) => {
                    Response::builder()
                        .status(http::status::OK)
                        .body(&b"hello"[..])
                        .unwrap()
                }
                (&http::method::GET, "/world", None) => {
                    Response::builder()
                        .status(http::status::OK)
                        .body(&b"Hello world!"[..])
                        .unwrap()
                }
                (&http::method::GET, "/world", Some(query)) => {
                    Response::builder()
                        .status(http::status::OK)
                        .body(&query.as_bytes()[..])
                        .unwrap()
                }
                _ => default_response,
            }
        })?;
    }

    Ok(())
}
