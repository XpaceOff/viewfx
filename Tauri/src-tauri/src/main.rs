#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use axum::{
  routing::{get},
  http::{StatusCode, Method, HeaderValue},
  Json, Router, 
  extract::Query
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use image::GenericImageView;
use tower_http::cors::CorsLayer;

// Make the main function Async with Tokio
#[tokio::main]
async fn main() {

  // Create a new thread that will run the backend-api/http-bridge
  // to load Images. This solve the bottleneck on Tauri's IPC
  // Check https://github.com/tauri-apps/tauri/discussions/4191
  // for more info about the problem
  tokio::spawn(async {

    // build our application with routes
    let app = Router::new()
    .route("/", get(|| async { "ViewFX" }))
    .route("/image_raw_data", get(http_get_image_raw_data))
    .layer(
      // CORS Middleware that solves the CORS problem
      // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
      // and https://github.com/tokio-rs/axum/blob/main/examples/cors/src/main.rs
      // for more details

      // TODO: You might need a way to get the port that Tauri is running on.
      // If not then the app won't work because of the CORS problem
      CorsLayer::new()
      .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
      //.allow_origin("https://tauri.localhost".parse::<HeaderValue>().unwrap())
      .allow_methods([Method::GET]),
    );
  
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("# http bridge listening on {}", addr);

    axum::Server::bind(&addr)
      .serve(app.into_make_service())
      .await
      .unwrap();
  });

  // share the current runtime with Tauri
  // Here https://github.com/tauri-apps/tauri/discussions/4191#discussioncomment-2833069
  // For more info
  tauri::async_runtime::set(tokio::runtime::Handle::current());

  tauri::Builder::default()
    //.menu(menu)
    //.invoke_handler(tauri::generate_handler![close_splashscreen])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

// Read image and returns it through the http bridge
async fn http_get_image_raw_data(payload: Query<ImageQuery>) -> Result<(StatusCode, Json<ImageResult>), (StatusCode, String)> {

  let img_full_path = payload.img_full_path.clone();
  

  let frame_number = payload.frame_number;
  let canvas_w = payload.canvas_w;
  let canvas_h = payload.canvas_h;

  // Number pf pads (0) for the image number.
  // let n_pads = 3;

  println!("# Canva's size is {}x{}", canvas_w, canvas_h);

  // TODO: Make sure that this number does not lead to a security issue.
  // Set the image to load
  let img_name = img_full_path; //format!("C:/Users/marqu/Resilio Sync/potato/programming/Tests/tauri_test/tauri-canvas/public/jpg-seq/ezgif-frame-{:0n_pads$}.jpg", frame_number, n_pads = n_pads);
  println!("# HTTP-LOG: {}", img_name);

  // Open the image
  let img = image::open(img_name);
  let img = match img {
    Ok(tmp_image) => tmp_image,
    Err(_error) => return Err( (StatusCode::NOT_FOUND, format!("Image not found")) ),
  };

  //let img = img.resize(canvas_w, canvas_h, image::imageops::Lanczos3);
  let img = img.thumbnail(canvas_w, canvas_h);
  
  // The dimensions method returns the images width and height.
  println!("# Returned dimensions are {:?}", img.dimensions());

  // The color method returns the image's `ColorType`.
  //println!("{:?}", img.color());

  let mut img_raw_data = Vec::new();
  for (_x, _y, pixel) in img.pixels() {
    // Do something with pixel.
    //println!("{:?}", pixel.0[0]);
    img_raw_data.push(pixel.0[0]);
    img_raw_data.push(pixel.0[1]);
    img_raw_data.push(pixel.0[2]);
    img_raw_data.push(pixel.0[3]);
  }

  println!("Data transformed!");

  // TODO: Make sure that this is 100% secure
  // Returns:{
  //   1- An array with the raw data of the current image
  //   2- Current frame number / image number
  //   3- The size to be display on the canvas
  // }

  let image_r = ImageResult {
    image_raw_data: img_raw_data,
    frame_number: frame_number,
    img_dimensions: img.dimensions(),
  };

  Ok( (StatusCode::OK, Json(image_r)) )
}

#[derive(Deserialize)]
struct ImageQuery {
  img_full_path: String,
  frame_number: u32,
  canvas_w: u32,
  canvas_h: u32,
}

#[derive(Serialize)]
struct ImageResult {
  image_raw_data: Vec<u8>,
  frame_number: u32,
  img_dimensions: (u32, u32),
}