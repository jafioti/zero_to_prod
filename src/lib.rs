use axum::{routing::get, Router};
use std::{net::SocketAddr};

pub fn run() -> axum::Server<hyper::server::conn::AddrIncoming, axum::routing::IntoMakeService<Router<(), axum::body::Body>>> {
    let app = Router::new()
        .route("/", get(root))
        .route("/health_check", get(health_check));
    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 8000)))
        .serve(app.into_make_service())
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn health_check() {}