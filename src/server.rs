use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use std::{convert::Infallible, net::SocketAddr};

use crate::print::print_http;

/// Echoes HTTP back to the sender and prints the request to stdout.
async fn echo(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    let (parts, body) = req.into_parts();

    // Consume the body stream...
    let body_bytes = hyper::body::to_bytes(body).await.unwrap();

    print_http(&parts, &body_bytes);

    *response.status_mut() = hyper::StatusCode::OK;
    *response.version_mut() = parts.version;
    *response.headers_mut() = parts.headers;
    *response.body_mut() = Body::from(body_bytes);

    Ok(response)
}

pub async fn start() {
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(echo)) });
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let server = Server::bind(&addr).serve(make_svc);

    println!("Echo started!\nWaiting for requests...");

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
