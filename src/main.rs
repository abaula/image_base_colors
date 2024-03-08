pub mod img_utils;
pub mod kmeans;
pub mod web;

use std::env;
use axum::{
    extract::DefaultBodyLimit,
    routing::{ get, post, },
    Router,
};
use tokio::signal;
use crate::web::controller;

#[tokio::main]
async fn main() {

    let app_port = env::var("APP_PORT")
        .unwrap_or(String::from("80"));

    const IMAGE_LIMIT_10MB: usize = 1024 * 1024 * 10;

    // build our application with a single route
    let app = Router::new()
        .route("/", get(controller::hello))
        .route("/alive", get(controller::status_ok))
        .route("/ready", get(controller::status_ok))
        .route("/info", post(controller::info))
            .layer(DefaultBodyLimit::max(IMAGE_LIMIT_10MB))
        .route("/draw", post(controller::draw))
            .layer(DefaultBodyLimit::max(IMAGE_LIMIT_10MB));

    let bind_addr = format!("0.0.0.0:{app_port}");
    let listener = match tokio::net::TcpListener::bind(bind_addr).await {
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