pub mod img_utils;
pub mod kmeans;

use crate::img_utils::base_colors;

fn main() {
    //let img_name = "Two-horses-from-SS.png";
    let img_name = "800px-Cat_November_2010-1a.jpg";

    let number_of_clusters = 4;
    let max_try_count = 30;

    let img_path_in = format!("/home/dev/work/image_base_colors/images/{img_name}");
    let img_path_out = format!("/home/dev/work/image_base_colors/images/__{img_name}.png");

    let source_img = match base_colors::open_image(&img_path_in) {
        Ok(image) => image,
        Err(err) => panic!("Error: {err}"),
    };

    let base_colors = base_colors::kmeans_calculate(&source_img, number_of_clusters, max_try_count);
    dbg!(&base_colors);
    let result_img = base_colors::draw(&source_img, &base_colors);

    match result_img.save(&img_path_out) {
        Ok(_) => println!("New image saved to {img_path_out}"),
        Err(err) => panic!("Error: {err}"),
    };
}
