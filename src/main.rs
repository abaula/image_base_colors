pub mod img_utils;
pub mod kmeans;
pub mod web;

use std::collections::HashMap;

use crate::{
    img_utils::base_colors,
    web::{ controller, request_parser::Request }
};

use axum::{
    extract::{ DefaultBodyLimit, Json, Multipart, Query, },
    http::{header, StatusCode, },
    response::IntoResponse,
    routing::{ get, post, },
    Router,
    body::Bytes,
};

#[tokio::main]
async fn main() {

    const IMAGE_LIMIT_10MB: usize = 1024 * 1024 * 10;

    // build our application with a single route
    let app = Router::new()
        .route("/", get(hello))
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

    match axum::serve(listener, app).await {
        Ok(_) => (),
        Err(err) => panic!("Server error: {err}")
    };

    //run_local_base_colors();
}

async fn hello() -> String {
    format!("Image base colors. Version: {}", env!("CARGO_PKG_VERSION"))
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

fn _run_local_base_colors() {
    //let img_name = "Two-horses-from-SS.png";
    let img_name = "800px-Cat_November_2010-1a.jpg";

    let number_of_clusters = 4;
    let max_try_count = 30;

    let img_path_in = format!("/home/dev/work/image_base_colors/images/{img_name}");
    let img_path_out = format!("/home/dev/work/image_base_colors/images/__{img_name}.png");

    let source_img = match base_colors::open_image(&img_path_in) {
        Ok(image) => image,
        Err(err) => panic!("Error: {err}"),
    };

    let base_colors = base_colors::kmeans_calculate(&source_img, number_of_clusters, max_try_count);
    dbg!(&base_colors);
    let result_img = base_colors::draw(&source_img, &base_colors);

    match result_img.save(&img_path_out) {
        Ok(_) => println!("New image saved to {img_path_out}"),
        Err(err) => panic!("Error: {err}"),
    };
}
