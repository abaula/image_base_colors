use crate::rgba_color::RgbaColor;

#[derive(Debug)]
pub struct Point {
    pub color: RgbaColor,
    pub center: f32,
}

impl Point {
    pub fn new (color: RgbaColor, center: f32) -> Self {
        Self { color, center }
    }
}