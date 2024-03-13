use serde::Serialize;
use std::{
    cmp::Ordering,
    hash::{DefaultHasher, Hasher},
};

#[derive(Serialize, Debug, Clone, Copy)]
pub struct RgbColor {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

impl RgbColor {
    pub fn new(r: u32, g: u32, b: u32) -> Self {
        Self { r, g, b }
    }

    pub fn from_vec(values: &Vec<u32>) -> Option<Self> {
        match values.len() == Self::dim() {
            true => Some(Self::new(values[0], values[1], values[2])),
            false => None,
        }
    }

    pub fn from_f32_vec(values: &Vec<f32>) -> Result<Self, String> {
        let expected_len = Self::dim();
        let request_len = values.len();

        match request_len.cmp(&expected_len) {
            Ordering::Equal => Ok(Self::new(
                values[0] as u32,
                values[1] as u32,
                values[2] as u32,
            )),
            _ => {
                let error = format!("Unable convert Vec<f32> to RgbColor<u32>, expected number of elements {expected_len} but got {request_len}.");
                Err(error)
            }
        }
    }

    pub fn dim() -> usize {
        3
    }

    pub fn hash_key(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        hasher.write(&self.r.to_ne_bytes());
        hasher.write(&self.g.to_ne_bytes());
        hasher.write(&self.b.to_ne_bytes());

        hasher.finish()
    }

    pub fn to_vec(&self) -> Vec<u32> {
        vec![self.r, self.g, self.b]
    }

    pub fn to_f32_vec(&self) -> Vec<f32> {
        vec![self.r as f32, self.g as f32, self.b as f32]
    }
}
