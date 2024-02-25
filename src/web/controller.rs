use std::io::Cursor;

use image::{ ImageError, ImageOutputFormat };

use crate::img_utils::{
    base_colors,
    color_point::ColorPoint
};

pub fn get_base_colors_info(buffer: &Vec<u8>, number_of_clusters: u32, max_try_count: u32) -> Result<Vec<ColorPoint>, ImageError> {

    let image = match base_colors::open_image_from_bytes(&buffer) {
        Ok(img) => img,
        Err(err) => return Err(err),
    };

    Ok(base_colors::kmeans_calculate(&image, number_of_clusters, max_try_count))
}

pub fn get_png_image_with_base_colors(buffer: &Vec<u8>, number_of_clusters: u32, max_try_count: u32) -> Result<Vec<u8>, ImageError> {

    let source_img = match base_colors::open_image_from_bytes(&buffer) {
        Ok(img) => img,
        Err(err) => return Err(err),
    };

    let base_colors = base_colors::kmeans_calculate(&source_img, number_of_clusters, max_try_count);
    let result_img = base_colors::draw(&source_img, &base_colors);

    let mut buff = Cursor::new(Vec::new());
    result_img.write_to(&mut buff, ImageOutputFormat::Png).unwrap();

    Ok(buff.into_inner())
}