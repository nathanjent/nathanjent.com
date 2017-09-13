extern crate nathanjent;
extern crate common;
extern crate dotenv;
extern crate http;
#[macro_use] extern crate diesel;

use nathanjent::*;
use self::models::*;
use diesel::prelude::*;
use dotenv::dotenv;
use http::{Method, StatusCode, Request, Response};
use std::io::{self, Write};
use common::Query;

fn main() {
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
    use schema::notes::dsl::*;
    let conn = establish_connection();

    let mut out = String::new();
    if let Ok(ref mut request) = common::build_request_from_env(io::stdin()) {
        common::route(&request, io::stdout(), || {
            let request_triple = (
                request.method(),
                request.uri().path(),
                request.uri().query()
                );
            let response = match request_triple {
                (&Method::GET, "/", None) => {
                    Response::builder()
                        .status(StatusCode::OK)
                        .body(&b"hello"[..])
                }
                (&Method::GET, "/world", None) => {
                    Response::builder()
                        .status(StatusCode::OK)
                        .body(&b"Hello world!"[..])
                }
                (&Method::GET, "/note", Some(query_str)) => {
                    let mut res_builder = Response::builder();
                    res_builder.status(StatusCode::BAD_REQUEST);
                    if let Ok(query) = query_str.parse::<Query>() {
                        let query: Query = query;
                        if let Some(query_str) = query.get_first("query_id") {
                            if let Ok(query_id) = query_str.parse::<i32>() {
                               let result = notes
                                    //.filter(published.eq(true))
                                    .find(query_id)
                                    .first::<Note>(&conn);
                                if let Ok(result) = result {
                                    res_builder.status(StatusCode::OK);
                                    out.push_str(&*result.text.unwrap());
                                } else {
                                    res_builder.status(StatusCode::NOT_FOUND);
                                }
                            }
                        }
                    }
                    res_builder.body(&out.as_bytes()[..])
                }
                (&Method::POST, "/note", Some(query_str)) => {
                    let query = query_str.parse::<Query>();

                    Response::builder()
                        .status(StatusCode::OK)
                        .body(&b"Can't post."[..])
                }
                _ => {
                    Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(&b""[..])
                }
            };
            response.expect("HTTP Routing failed")
        })?;
    }

    Ok(())
}
