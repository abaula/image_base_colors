pub mod img_utils;
pub mod kmeans;
pub mod web;

use std::collections::HashMap;
use crate::web::{ controller, request_parser::Request };
use axum::{
    extract::{ DefaultBodyLimit, Json, Multipart, Query, },
    http::{header, StatusCode, },
    response::IntoResponse,
    routing::{ get, post, },
    Router,
    body::Bytes,
};
use tokio::signal;

#[tokio::main]
async fn main() {

    const IMAGE_LIMIT_10MB: usize = 1024 * 1024 * 10;

    // build our application with a single route
    let app = Router::new()
        .route("/", get(hello))
        .route("/alive", get(status_ok))
        .route("/ready", get(status_ok))
        .route("/info", post(info))
            .layer(DefaultBodyLimit::max(IMAGE_LIMIT_10MB))
        .route("/draw", post(draw))
            .layer(DefaultBodyLimit::max(IMAGE_LIMIT_10MB));

    // run our app with hyper, listening globally on port 3000
    let listener = match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
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

async fn info(Query(params): Query<HashMap<String, String>>, mut multipart: Multipart) -> impl IntoResponse {

    let request = match Request::parse(&params, &mut multipart).await {
        Ok(value) => value,
        Err(err) => return Err((StatusCode::BAD_REQUEST, err)),
    };

    let base_colors = match controller::get_base_colors_info(
        &request.file_buffer,
        request.number_of_clusters,
        request.max_try_count) {
        Ok(res) => res,
        Err(err) => return Err((StatusCode::BAD_REQUEST, format!("File is not a valid image: {}", err))),
    };

    Ok(
        (StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        Json(base_colors),)
    )
}

async fn draw(Query(params): Query<HashMap<String, String>>, mut multipart: Multipart) -> impl IntoResponse {

    let request = match Request::parse(&params, &mut multipart).await {
        Ok(value) => value,
        Err(err) => return Err((StatusCode::BAD_REQUEST, err)),
    };

    let base_colors_image = match controller::get_png_image_with_base_colors(
        &request.file_buffer,
        request.number_of_clusters,
        request.max_try_count) {
        Ok(res) => res,
        Err(err) => return Err((StatusCode::BAD_REQUEST, format!("File is not a valid image: {}", err))),
    };

    let bytes = Bytes::from(base_colors_image);

    Ok(
        (StatusCode::OK,
        [(header::CONTENT_TYPE, String::from("image/png")),
        (header::CONTENT_DISPOSITION, format!("inline; filename=\"{}\"", request.file_name))],
        bytes,)
    )
}
