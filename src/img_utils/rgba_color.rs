use std::hash::Hasher;
use siphasher::sip::SipHasher;

const HASHER_INIT_KEY: &[u8; 16] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

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

    pub fn get_size() -> u32 {
        4
    }

    pub fn get_hash_key(&self) -> u64 {
        let mut hasher = SipHasher::new_with_key(HASHER_INIT_KEY);

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
