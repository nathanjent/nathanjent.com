use chrono;
use http::{StatusCode, Request, Response};
use std::time::{Duration, Instant};
use std::panic;
use std::io::{self, Read, Write};

use super::Result;

pub fn send_response<'r, W, F, REQ, RES>(req: &Request<REQ>, mut out: W, f: F) -> Result<()>
    where W: Write,
          REQ: Read,
          RES: Read,
          F: FnOnce() -> Response<RES>
{
    let start_instant = Instant::now();
    let rq_line = format!("{} UTC - {} {}",
                          chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.6f"),
                          req.method(),
                          req.uri());

    // Calling the handler and catching potential panics.
    // Note that this we always resume unwinding afterwards,
    // we can ignore the small panic-safety mecanism of `catch_unwind`.
    let mut response = panic::catch_unwind(panic::AssertUnwindSafe(f));

    let elapsed_time = format_time(start_instant.elapsed());

    match response {
        Ok(mut response) => {
            match response.status() {
                StatusCode::ACCEPTED => {
                },
                s @ StatusCode::BAD_REQUEST | s @ StatusCode::NOT_FOUND => {
                    writeln!(out, "Status: {}",  s)?;
                },
                _ => {},
            }

            for (ref k, ref v) in response.headers() {
                writeln!(out, "{:?}: {:?}", k, v)?;
            }

            let mut body = response.body_mut();
            let mut buff = Vec::new();
            let content_length = io::copy(&mut body, &mut buff)?;
            if content_length > 0 {
                writeln!(out, "Content-Length: {}",  content_length)?;
            }
            writeln!(out, "")?;
            io::copy(&mut &buff[..], &mut out)?;
            writeln!(out, "")?;
        }
        Err(payload) => {
            // There is probably no point in printing the payload,
            // as this is done by the panic handler.
            let _ = writeln!(out, "{} - {} - PANIC!", rq_line, elapsed_time);
            panic::resume_unwind(payload);
        }
    }
    Ok(())
}

// copied from the rouille log function
fn format_time(duration: Duration) -> String {
    let secs_part = match duration.as_secs().checked_mul(1_000_000_000) {
        Some(v) => v,
        None => return format!("{}s", duration.as_secs() as f64),
    };

    let duration_in_ns = secs_part + duration.subsec_nanos() as u64;

    if duration_in_ns < 1_000 {
        format!("{}ns", duration_in_ns)
    } else if duration_in_ns < 1_000_000 {
        format!("{:.1}us", duration_in_ns as f64 / 1_000.0)
    } else if duration_in_ns < 1_000_000_000 {
        format!("{:.1}ms", duration_in_ns as f64 / 1_000_000.0)
    } else {
        format!("{:.1}s", duration_in_ns as f64 / 1_000_000_000.0)
    }
}
