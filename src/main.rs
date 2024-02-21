pub mod rgba_color;
pub mod point;
pub mod histogram;
pub mod histogram_k_means;
use crate::{histogram::Histogram, histogram_k_means::HistogramKMeans};


fn main() {

    let img_path = "/home/dev/work/image_base_colors/images/800px-Cat_November_2010-1a.jpg";
    let histogram = Histogram::from_image(img_path);
    let kmeans = HistogramKMeans::new();
    let mut centers = kmeans.cluster(&histogram, 4, 20);
    centers.sort_by(|a, b| b.1.total_cmp(&a.1));
    dbg!(centers);


    /*
    let img = match image::open(img_path) {
        Ok(dyn_img) => dyn_img.to_rgba32f(),
        Err(err) => panic!("error loading image: err"),
    };

    let pixels = img.pixels();

    for pixel in pixels {
        dbg!(pixel);
        return;
    }


    //dbg!(img);

    let color1 = Color::new(100, 101, 102, 100);
    let color2 = Color::new(200, 201, 202, 200);
    let color3 = Color::new(300, 301, 302, 300);

    let mut histogram = Histogram::new();
    histogram.append_color(&color1);
    histogram.append_color(&color2);
    histogram.append_color(&color3);
    histogram.append_color(&color2);
    histogram.append_color(&color3);
    histogram.append_color(&color3);
    histogram.append_color(&color3);

    let centers = HistogramKMeans::cluster(&histogram, 4, 100);
    dbg!(centers);
    */
}
