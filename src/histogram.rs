use std::collections::HashMap;
use crate::rgba_color::RgbaColor;

#[derive(Debug, Clone, Copy)]
pub struct HistogramEntry {
    pub color: RgbaColor,
    pub weight: u32,
}

impl HistogramEntry {
    pub fn new (color: RgbaColor, weight: u32) -> Self {
        Self { color, weight, }
    }
}

#[derive(Debug)]
pub struct Histogram {
    map: HashMap<u64, HistogramEntry>,
}

impl Histogram {
    pub fn new () -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn from_image(path: &str) -> Self {

        let img = match image::open(path) {
            Ok(dyn_img) => dyn_img.to_rgba32f(),
            Err(err) => panic!("error loading image: {err}"),
        };

        let mut histogram = Self::new();

        for pixel in img.pixels() {
            let color = RgbaColor::new(pixel[0], pixel[1], pixel[2], pixel[3]);
            histogram.append_color(&color);
        }

        histogram
    }

    pub fn append_color(&mut self, color: &RgbaColor) {
        let entry = self.map
            .entry(color.get_hash_key())
            .or_insert(HistogramEntry::new(color.clone(), 0));

        entry.weight += 1;
    }

    pub fn get_vec(&self) -> Vec<HistogramEntry> {
        let map = &self.map;
        map
            .into_iter()
            .map(|(_key, entry)| entry.clone())
            .collect()
    }
}
