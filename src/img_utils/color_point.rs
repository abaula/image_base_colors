use crate::img_utils::rgb_color::RgbColor;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, Copy)]
pub struct ColorPoint {
    pub color: RgbColor,
    pub weight: f32,
}

impl ColorPoint {
    pub fn new(color: RgbColor, weight: f32) -> Self {
        Self { color, weight }
    }

    pub fn color_dim() -> usize {
        RgbColor::dim()
    }
}
