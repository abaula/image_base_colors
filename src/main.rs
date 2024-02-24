pub mod img_utils;
pub mod kmeans;

use crate::img_utils::base_colors;

use axum::{
    body::Body,
    routing::get,
    response::Json,
    Router,
};

use rand::prelude::*;

#[tokio::main]
async fn main() {

    // build our application with a single route
    let app = Router::new()
        .route("/", get(hello));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    //run_local_base_colors();
}

async fn hello() -> String {
    let mut rng = rand::thread_rng();
    let msg = format!("Hello, World of {}!", rng.gen_range(0..10));
    msg
}

fn run_local_base_colors() {
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
