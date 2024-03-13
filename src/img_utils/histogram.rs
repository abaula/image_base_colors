use image::RgbImage;
use std::collections::HashMap;

use crate::{img_utils::color_point::ColorPoint, img_utils::rgb_color::RgbColor};

#[derive(Debug)]
pub struct Histogram {
    map: HashMap<u64, ColorPoint>,
}

impl Default for Histogram {
    fn default() -> Self {
        Self::new()
    }
}

impl Histogram {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn push_color(&mut self, color: &RgbColor) {
        let entry = self
            .map
            .entry(color.hash_key())
            .or_insert(ColorPoint::new(*color, 0_f32));

        entry.weight += 1_f32;
    }

    pub fn to_vec(&self) -> Vec<ColorPoint> {
        let map = &self.map;

        map.iter().map(|(_key, entry)| *entry).collect()
    }
}

pub fn from_image(img: &RgbImage) -> Histogram {
    let mut histogram = Histogram::new();

    img.pixels().for_each(|pixel| {
        let color = RgbColor::new(pixel[0] as u32, pixel[1] as u32, pixel[2] as u32);
        histogram.push_color(&color);
    });

    histogram
}
