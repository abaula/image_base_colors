pub mod img_utils;
pub mod kmeans;

use crate::{
    img_utils::histogram,
    kmeans::histogram_k_means,
};

fn main() {
    let img_path = "/home/dev/work/image_base_colors/images/800px-Cat_November_2010-1a.jpg";
    let histogram = histogram::from_image_path(img_path);
    let mut centers = histogram_k_means::cluster(&histogram, 4, 30);

    centers.sort_by(|a, b| b.weight.total_cmp(&a.weight));

    dbg!(centers);
}
