use crate::img_utils::rgba_color::RgbaColor;

#[derive(Debug, Clone, Copy)]
pub struct ColorPoint {
    pub color: RgbaColor,
    pub weight: f32,
}

impl ColorPoint {
    pub fn new (color: RgbaColor, weight: f32) -> Self {
        Self { color, weight }
    }

    pub fn color_dim() -> usize {
        RgbaColor::dim()
    }
}