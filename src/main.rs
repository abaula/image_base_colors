pub mod img_utils;
pub mod kmeans;
pub mod web;

use axum::{
    extract::DefaultBodyLimit,
    http::StatusCode,
    routing::{ get, post, },
    Router,
};
use tokio::signal;
use crate::web::controller;

#[tokio::main]
async fn main() {

    const IMAGE_LIMIT_10MB: usize = 1024 * 1024 * 10;

    // build our application with a single route
    let app = Router::new()
        .route("/", get(hello))
        .route("/alive", get(status_ok))
        .route("/ready", get(status_ok))
        .route("/info", post(controller::info))
            .layer(DefaultBodyLimit::max(IMAGE_LIMIT_10MB))
        .route("/draw", post(controller::draw))
            .layer(DefaultBodyLimit::max(IMAGE_LIMIT_10MB));

    // run our app with hyper, listening globally on port 8080
    let listener = match tokio::net::TcpListener::bind("0.0.0.0:8080").await {
        Ok(lsn) => lsn,
        Err(err) => panic!("Bind error: {err}"),
    };

    let socket_addr = listener.local_addr().unwrap();
    println!("Server is listening on {}:{}", &socket_addr.ip(), &socket_addr.port());

    match axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await {
            Ok(_) => (),
            Err(err) => panic!("Server error: {err}")
        };
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

async fn hello() -> String {
    format!("Image base colors. Version: {}", env!("CARGO_PKG_VERSION"))
}

async fn status_ok() -> StatusCode {
    StatusCode::OK
}