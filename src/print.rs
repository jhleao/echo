use chrono::Utc;
use hyper::body::Bytes;
use hyper::http::request::Parts;
use termsize::Size;

const DIVIDER_CHAR: &str = "=";

/// Formats and the HTTP request to stdout with a timestamp header.
pub fn print_http(parts: &Parts, body_bytes: &Bytes) {
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

    let date_iso = Utc::now().to_rfc2822();
    let term_size = termsize::get().unwrap_or(Size { cols: 0, rows: 0 });
    let term_width: usize = term_size.cols.into();
    let date_len: usize = date_iso.len();
    let divider_chunk = DIVIDER_CHAR.repeat((term_width - date_len - 2) / 2);
    let divider = format!("\n{divider_chunk}[{date_iso}]{divider_chunk}");

    let mut headers_str = String::from("\n");

    for (key, value) in parts.headers.clone() {
        match key {
            None => continue,
            Some(key) => {
                if let Ok(value_str) = value.to_str() {
                    let formatted = format!("{}: {}", key, value_str);
                    headers_str.push_str(&formatted)
                }
            }
        }
    }

    println!("{divider}",);
    println!("{} {} {:?}", parts.method, parts.uri, parts.version);
    println!("{}", headers_str);
    println!("\n{}", body_str);
}
