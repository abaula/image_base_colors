use std::hash::{DefaultHasher, Hasher};

#[derive(Debug, Clone, Copy)]
pub struct RgbaColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl RgbaColor {

    pub fn new (r: f32, g: f32, b: f32, a: f32, ) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_vec (values: &Vec<f32>) -> Option<Self> {

        match values.len() == Self::dim() {
            true => {
                Some(Self::new(values[0], values[1], values[2], values[3]))
            },
            false => None
        }
    }

    pub fn dim() -> usize {
        4
    }

    pub fn hash_key(&self) -> u64 {

        let mut hasher = DefaultHasher::new();

        hasher.write(&self.r.to_ne_bytes());
        hasher.write(&self.g.to_ne_bytes());
        hasher.write(&self.b.to_ne_bytes());
        hasher.write(&self.a.to_ne_bytes());

        hasher.finish()
    }

    pub fn to_vec(&self) -> Vec<f32> {
        vec![self.r, self.g, self.b, self.a]
    }
}
