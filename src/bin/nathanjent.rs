extern crate nathanjent;
extern crate common;
extern crate http_router;
extern crate http;
#[macro_use] extern crate diesel;

use nathanjent::*;
use self::models::*;
use diesel::prelude::*;
use http::{Method, StatusCode, Request, Response};
use std::io::{self, Write};
use common::Query;
use http_router::{Handler, RouteBuilder as Router};

fn main() {
    let mut router = Router::new();
    router.get("/id", handle_id);

    let status = match start(router) {
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

fn handle_id<'r>(req: &mut Request<&'r [u8]>) -> Result<Response<&'r [u8]>, http::Error> {
    Response::builder()
        .status(StatusCode::OK)
        .body(&b""[..])
}

fn start<'r, H>(handler: H) -> common::Result<()>
    where H: Handler<&'r [u8], &'r [u8]> + 'static + Sync
{
    let conn = establish_connection();

    let mut out = String::new();
    if let Ok(ref mut request) = common::build_request_from_env() {
        common::send_response(&request, io::stdout(), || {
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
                (&Method::GET, "/env", None) => {
                    out = ::std::env::vars()
                        .map(|(k, v)| k + ":" + &v + "\r")
                        .collect();
                    Response::builder()
                        .status(StatusCode::OK)
                        .body(&out.as_bytes()[..])
                }
                (&Method::GET, "/headers", None) => {
                    out = request.headers()
                        .iter()
                        .map(|(k, ref v)| {
                            format!("{}:{}\r", k.as_str(), v.to_str().unwrap())
                        })
                        .collect();
                    Response::builder()
                        .status(StatusCode::OK)
                        .body(&out.as_bytes()[..])
                }
                (&Method::GET, "/world", None) => {
                    Response::builder()
                        .status(StatusCode::OK)
                        .body(&b"Hello world!"[..])
                }
                (&Method::GET, "/note", Some(query_str)) => {
                    use schema::notes::dsl::*;
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
                    use schema::notes::dsl::notes;
                    let mut res_builder = Response::builder();
                    res_builder.status(StatusCode::BAD_REQUEST);
                    if let Ok(query) = query_str.parse::<Query>() {
                        let query: Query = query;
                        if let Some(title) = query.get_first("title") {
                            if let Some(text) = query.get_first("text") {
                                let new_note = NewNote {
                                    title,
                                    text,
                                };

                                let result = diesel::insert(&new_note)
                                    .into(notes)
                                    .execute(&conn);
                                    
                                if let Ok(result) = result {
                                    res_builder.status(StatusCode::OK);
                                } else {
                                    res_builder.status(StatusCode::NOT_FOUND);
                                }
                            }
                        }
                    }
                    res_builder.body(&b""[..])
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
