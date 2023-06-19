use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use std::convert::Infallible;

use crate::{config, print::print_http};

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

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

pub async fn start(cfg: &config::Config) {
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(echo)) });
    let server = Server::bind(&cfg.socket_addr).serve(make_svc);

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    println!("Echo started!\nWaiting for requests...");

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}
