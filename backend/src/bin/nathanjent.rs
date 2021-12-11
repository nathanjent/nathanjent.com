use common::Query;
use ::anyhow::Result;
use ::http::{Method, Request, Response, Version};
use ::matchit::{Match, Node};
use ::outer_cgi::IO;
use http::StatusCode;

use std::borrow::Cow;
use std::collections::HashMap;
use std::io;
use std::str::FromStr;

fn handler(io: &mut dyn IO, env: HashMap<String, String>) -> io::Result<i32> {
    let mut request_body = Vec::new();
    let size = io.read_to_end(&mut request_body)?;

    let response = match handle(&*request_body, size, &env) {
        Ok(response) => response,
        Err(err) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(
                format!(
                    r#"<h1>500 Server Error</h1>
<p>{}</p>
"#,
                    err
                )
                .into(),
            )
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?,
    };

    let body = response.body();
    let header_str: String = response
        .headers()
        .iter()
        .map(|(key, value)| (key.as_str(), value.to_str()))
        .filter(|(_key, value)| value.is_ok())
        .map(|(key, value)| (key, value.unwrap()))
        .map(|(key, value)| {
            format!(
                r#"{}: {}
"#,
                key, value,
            )
        })
        .collect();

    io.write_all(
        format!(
            r#"{}

{}
"#,
            header_str, body
        )
        .as_bytes(),
    )?;
    Ok(0)
}

fn handle<'a>(
    request_body: &'a [u8],
    size: usize,
    env: &'a HashMap<String, String>,
) -> Result<Response<Cow<'a, str>>> {
    if let Some(content_length_str) = env.get("CONTENT_LENGTH") {
        if let Ok(content_length) = usize::from_str(content_length_str) {
            if content_length != size {
                let response = Response::builder().status(StatusCode::BAD_REQUEST).body(
                    format!(
                        r#"<h1>Content Length Unmatched</h1>
<p>The content length, {}, does not match the size of the content, {}.</p>
"#,
                        content_length, size
                    )
                    .into(),
                )?;
                return Ok(response);
            }
        }
    }

    let request = create_request(&request_body, &env)?;
    let response = handle_request(request)?;

    Ok(response)
}

fn create_request<'a>(io: &'a [u8], env: &'a HashMap<String, String>) -> Result<Request<&'a [u8]>> {
    let request = Request::builder()
        .method(match env.get("REQUEST_METHOD") {
            Some(method) => Method::from_str(method)?,
            None => Method::GET,
        })
        .uri(
            &*env
                .get("REQUEST_URI")
                .ok_or(io::Error::new(io::ErrorKind::NotFound, ""))?,
        )
        .version(match &*env["SERVER_PROTOCOL"] {
            "HTTP/0.9" => Version::HTTP_09,
            "HTTP/1.0" | "HTTP/1" => Version::HTTP_10,
            "HTTP/1.1" => Version::HTTP_11,
            "HTTP/2.0" | "HTTP/2" => Version::HTTP_2,
            _ => Version::HTTP_10,
        })
        .body(io)?;
    Ok(request)
}

fn handle_request<'a>(req: Request<&'a [u8]>) -> Result<Response<Cow<'a, str>>> {
    let router = route()?;
    let Match { value, params } = router.at(req.uri().path())?;
    let response = match value {
        0 => Response::builder().body("Welcome".into())?,
        1 => Response::builder().body(format!("id: {}", params.get("id").unwrap_or("0")).into())?,
        2 => Response::builder().body(::std::env::vars()
            .map(|(key, value)| format!("{}: {}\r", key, &value))
            .collect())?,
        3 => Response::builder().body(req
            .headers()
            .iter()
            .map(|(key, value)| (key.as_str(), value.to_str()))
            .filter(|(_key, value)| value.is_ok())
            .map(|(key, value)| format!("{}: {}\r", key, value.unwrap()))
            .collect())?,
        4 => handle_note_request(req)?,
        _ => {
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
            .body("Unknown".into())?  
        },
    };
    Ok(response)
}

fn handle_note_request<'a>(req: Request<&'a [u8]>) -> Result<Response<Cow<'a, str>>> {
    let mut response = Response::builder().body("".into())?;
    if let Some(query_str) = req.uri().query() {
        if let Ok(query) = query_str.parse::<Query>() {
            if let Some(query_str) = query.get_first("query_id") {
                response = Response::builder().body(query_str.to_string().into())?;
            }
        }
    }

    Ok(response)
}

fn route() -> Result<Node<u32>> {
    let mut matcher = Node::new();
    matcher.insert("/", 0)?;
    matcher.insert("/user/:id", 1)?;
    matcher.insert("/env", 2)?;
    matcher.insert("/headers", 3)?;
    matcher.insert("/note", 4)?;

    Ok(matcher)
}

fn init(_max_connections: u32) {
    // TODO Somehow define router matches here
}

fn main() {
    outer_cgi::main(init, handler)
}
