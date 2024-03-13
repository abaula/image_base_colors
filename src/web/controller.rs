use crate::img_utils::{base_colors, color_point::ColorPoint};
use crate::web::request_parser::Request;
use axum::{
    body::Bytes,
    extract::{Json, Multipart, Query},
    http::{header, StatusCode},
    response::IntoResponse,
};
use image::{ImageError, ImageOutputFormat};
use std::collections::HashMap;
use std::io::Cursor;

pub async fn hello() -> String {
    format!("Image base colors. Version: {}", env!("CARGO_PKG_VERSION"))
}

pub async fn status_ok() -> StatusCode {
    StatusCode::OK
}

pub async fn info(
    Query(params): Query<HashMap<String, String>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let request = match Request::parse(&params, &mut multipart).await {
        Ok(value) => value,
        Err(err) => return Err((StatusCode::BAD_REQUEST, err)),
    };

    let base_colors = match get_base_colors_info(
        &request.file_buffer,
        request.number_of_clusters,
        request.max_try_count,
    ) {
        Ok(res) => res,
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("File is not a valid image: {}", err),
            ))
        }
    };

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        Json(base_colors),
    ))
}

pub async fn draw(
    Query(params): Query<HashMap<String, String>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let request = match Request::parse(&params, &mut multipart).await {
        Ok(value) => value,
        Err(err) => return Err((StatusCode::BAD_REQUEST, err)),
    };

    let base_colors_image = match get_png_image_with_base_colors(
        &request.file_buffer,
        request.number_of_clusters,
        request.max_try_count,
    ) {
        Ok(res) => res,
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("File is not a valid image: {}", err),
            ))
        }
    };

    let bytes = Bytes::from(base_colors_image);

    Ok((
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, String::from("image/png")),
            (
                header::CONTENT_DISPOSITION,
                format!("inline; filename=\"{}\"", request.file_name),
            ),
        ],
        bytes,
    ))
}

fn get_base_colors_info(
    buffer: &[u8],
    number_of_clusters: u32,
    max_try_count: u32,
) -> Result<Vec<ColorPoint>, ImageError> {
    let image = base_colors::open_image_from_bytes(buffer)?;

    Ok(base_colors::kmeans_calculate(
        &image,
        number_of_clusters,
        max_try_count,
    ))
}

fn get_png_image_with_base_colors(
    buffer: &[u8],
    number_of_clusters: u32,
    max_try_count: u32,
) -> Result<Vec<u8>, ImageError> {
    let source_img = base_colors::open_image_from_bytes(buffer)?;

    let base_colors = base_colors::kmeans_calculate(&source_img, number_of_clusters, max_try_count);
    let result_img = base_colors::draw(&source_img, &base_colors);

    let mut buff = Cursor::new(Vec::new());
    result_img.write_to(&mut buff, ImageOutputFormat::Png)?;

    Ok(buff.into_inner())
}
