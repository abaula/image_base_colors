use super::color_point::ColorPoint;
use crate::{img_utils::histogram, kmeans::histogram_k_means};
use image::{ImageError, Rgb, RgbImage};
use std::cmp::min;

pub fn open_image(path: &str) -> Result<RgbImage, ImageError> {
    let image = image::open(path)?.to_rgb8();

    Ok(image)
}

pub fn open_image_from_bytes(data: &[u8]) -> Result<RgbImage, ImageError> {
    let image = image::load_from_memory(data)?.to_rgb8();

    Ok(image)
}

pub fn kmeans_calculate(
    source_img: &RgbImage,
    number_of_clusters: u32,
    max_try_count: u32,
) -> Vec<ColorPoint> {
    let histogram = histogram::from_image(source_img);
    let mut centers = histogram_k_means::cluster(&histogram, number_of_clusters, max_try_count);
    centers.sort_by(|a, b| b.weight.total_cmp(&a.weight));

    centers
}

pub fn draw(source_img: &RgbImage, base_colors: &[ColorPoint]) -> RgbImage {
    let (width, height) = source_img.dimensions();

    // expand new image width.
    let expand_ratio = 1.2_f32;
    let out_img_width = (width as f32 * expand_ratio) as u32;
    let mut out_img = RgbImage::new(out_img_width, height);

    // write source image to new one.
    source_img.enumerate_pixels().for_each(|(x, y, pixel)| {
        out_img.put_pixel(x, y, *pixel);
    });

    // sort base colors.
    let mut sorted_base_colors = base_colors.to_vec();
    sorted_base_colors.sort_by(|a, b| a.weight.total_cmp(&b.weight));
    draw_base_colors_area(
        &mut out_img,
        &sorted_base_colors,
        width,
        out_img_width,
        height,
    );

    out_img
}

fn draw_base_colors_area(
    img: &mut RgbImage,
    centers: &[ColorPoint],
    left: u32,
    right: u32,
    height: u32,
) {
    let mut y_top = 0_u32;
    let mut y_bottom = 0_u32;

    centers.iter().for_each(|point| {
        y_bottom = min(y_top + (height as f32 * point.weight).ceil() as u32, height);

        (left..right).for_each(|x| {
            (y_top..y_bottom).for_each(|y| {
                let rgb = Rgb([
                    point.color.r as u8,
                    point.color.g as u8,
                    point.color.b as u8,
                ]);
                img.put_pixel(x, y, rgb);
            });
        });

        y_top = y_bottom;
    });
}
