use chrono::Utc;
use colored::{Color, Colorize};
use hyper::body::Bytes;
use hyper::http::request::Parts;
use hyper::Method;
use termsize::Size;

const DIVIDER_CHAR: &str = "─";

/// Formats and the HTTP request to stdout with a timestamp header.
pub fn print_http(parts: &Parts, body_bytes: &Bytes) {
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

    let date_iso = Utc::now().to_rfc2822();
    let term_size = termsize::get().unwrap_or(Size { cols: 0, rows: 0 });
    let term_width: usize = term_size.cols.into();
    let date_len: usize = date_iso.len();
    let divider_chunk_len = (term_width - date_len - 1) / 2;
    let divider_chunk = DIVIDER_CHAR.repeat(divider_chunk_len);
    let divider = format!("{divider_chunk}{divider_chunk} {date_iso}").bright_black();
    let footer = DIVIDER_CHAR.repeat(term_width).bright_black();

    let version_str = format!("{:?}", parts.version);

    let mut headers_str = String::new();

    for (key, value) in parts.headers.clone() {
        match key {
            None => continue,
            Some(key) => {
                if let Ok(value_str) = value.to_str() {
                    let key_str: &str = key.as_str();
                    let formatted = format!("{}: {}\n", key_str.bright_black(), value_str);
                    headers_str.push_str(&formatted)
                }
            }
        }
    }

    let method_color: Color = match parts.method {
        Method::GET => Color::Magenta,
        Method::POST => Color::Green,
        Method::PUT => Color::Yellow,
        Method::PATCH => Color::BrightYellow,
        Method::DELETE => Color::BrightRed,
        _ => Color::White,
    };

    let colored_method = parts.method.as_str().color(method_color);
    let colored_uri = parts.uri.to_string().color(method_color);
    let colored_version = version_str.bright_black();
    let colored_body = body_str
        .replace("  ", " ")
        .replace('\t', "  ")
        .bright_black();

    println!("{}", divider);
    println!("{} {} {}", colored_method, colored_uri, colored_version);
    println!("{}", headers_str);
    if colored_body.len() > 0 {
        println!("{}", colored_body);
    }
    println!("{}", footer);
}
