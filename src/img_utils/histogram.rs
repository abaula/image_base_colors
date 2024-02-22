use std::collections::HashMap;
use image::Rgba32FImage;

use crate::{
    img_utils::color_point::ColorPoint,
    img_utils::rgba_color::RgbaColor
};

#[derive(Debug)]
pub struct Histogram {
    map: HashMap<u64, ColorPoint>,
}

impl Histogram {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn push_color(&mut self, color: &RgbaColor) {

        let entry = self
            .map
            .entry(color.get_hash_key())
            .or_insert(ColorPoint::new(color.clone(), 0_f32));

        entry.weight += 1_f32;
    }

    pub fn get_vec(&self) -> Vec<ColorPoint> {

        let map = &self.map;

        map.iter()
            .map(|(_key, entry)| entry.clone())
            .collect()
    }
}

pub fn from_image_path(path: &str) -> Histogram {

    let img = match image::open(path) {
        Ok(dyn_img) => dyn_img.to_rgba32f(),
        Err(err) => panic!("error loading image: {err}"),
    };

    from_image(&img)
}

pub fn from_image(img: &Rgba32FImage) -> Histogram {
    let mut histogram = Histogram::new();

    for pixel in img.pixels() {
        let color = RgbaColor::new(pixel[0], pixel[1], pixel[2], pixel[3]);
        histogram.push_color(&color);
    }

    histogram
}
