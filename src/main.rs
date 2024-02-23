pub mod img_utils;
pub mod kmeans;

use crate::img_utils::base_colors;

fn main() {
    let img_name = "Two-horses-from-SS.png";
    //let img_name = "800px-Cat_November_2010-1a.jpg";

    let img_path_in = format!("/home/dev/work/image_base_colors/images/{img_name}");
    let img_path_out = format!("/home/dev/work/image_base_colors/images/__{img_name}.png");
    base_colors::draw_base_colors(&img_path_in, &img_path_out, 4, 30);
}
