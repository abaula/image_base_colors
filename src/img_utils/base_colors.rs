use std::cmp::min;
use image::{Rgb, RgbImage};

use crate::{
    img_utils::histogram,
    kmeans::histogram_k_means,
};

use super::color_point::ColorPoint;

pub fn draw_base_colors(img_path_in: &str, img_path_out: &str, number_of_clusters: u32, max_try_count: u32) {

    let img = open_image(img_path_in).unwrap();
    let histogram = histogram::from_image(&img);

    let mut centers = histogram_k_means::cluster(&histogram, number_of_clusters, max_try_count);
    centers.sort_by(|a, b| b.weight.total_cmp(&a.weight));
    draw_img_out(&img, &centers, img_path_out);

    dbg!(centers);
}

fn draw_img_out(source_img: &RgbImage, centers: &Vec<ColorPoint>, img_path_out: &str) {

    let (width, height) = source_img.dimensions();

    // expand new image width.
    let out_img_width = (width as f32 * 1.2_f32) as u32;
    let mut out_img= RgbImage::new(out_img_width, height);

    // write source image to new one.
    source_img.enumerate_pixels().for_each(|(x ,y, pixel)| {
        out_img.put_pixel(x, y, pixel.clone());
    });

    let mut sorted_centers = centers.clone();
    sorted_centers.sort_by(|a, b| a.weight.total_cmp(&b.weight));
    draw_dominant_color_area(&mut out_img, &sorted_centers, width, out_img_width, height);

    out_img.save(img_path_out).unwrap();
}

fn draw_dominant_color_area(img: &mut RgbImage, centers: &Vec<ColorPoint>, left: u32, right: u32, height: u32) {

    let mut y_top = 0_u32;
    let mut y_bottom = 0_u32;

    centers.iter().for_each(|point| {
        y_bottom = min(y_top + (height as f32 * point.weight).ceil() as u32, height);

        (left..right).for_each(|x| {
            (y_top..y_bottom).for_each(|y| {
                let rgb = Rgb([point.color.r as u8, point.color.g as u8, point.color.b as u8]);
                img.put_pixel(x, y, rgb);
            });
        });

        y_top = y_bottom;
    });
}

fn open_image(path: &str) -> Option<RgbImage> {
    match image::open(path) {
        Ok(dyn_img) => {
            Some(dyn_img.to_rgb8())
        },
        Err(err) => panic!("error loading image: {err}"),
    }
}