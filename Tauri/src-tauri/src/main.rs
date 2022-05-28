#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
async fn get_image_raw_data(frame_number: u32, canvas_w: u32, canvas_h: u32) -> Result<(Vec<u8>, u32, (u32, u32)), String> {
  use image::GenericImageView;

  // Number pf pads (0) for the image number.
  let n_pads = 3;

  println!("{}x{}", canvas_w, canvas_h);

  // TODO: Make sure that this number does not lead to a security issue.
  // Set the image to load
  let img_name = format!("C:/Users/marqu/Resilio Sync/potato/programming/Tests/tauri_test/tauri-canvas/public/jpg-seq/ezgif-frame-{:0n_pads$}.jpg", frame_number, n_pads = n_pads);
  println!("{}", img_name);

  // Open the image
  let img = image::open(img_name);
  let img = match img {
    Ok(tmp_image) => tmp_image,
    Err(_error) => return Err("Error reading file".into()),
  };

  //let img = img.resize(canvas_w, canvas_h, image::imageops::Lanczos3);
  let img = img.thumbnail(canvas_w, canvas_h);
  
  // The dimensions method returns the images width and height.
  println!("dimensions {:?}", img.dimensions());

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
  Ok((img_raw_data, frame_number, img.dimensions()))

}


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

#[tokio::main]
async fn main() {

  tokio::spawn(async {
    // build our application with a route
    let app = Router::new()
    // `GET /` goes to `root`
    .route("/", get(|| async { "ViewFX" }))
    .route("/image_raw_data", get(http_get_image_raw_data))
    .layer(
      // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
      // and https://github.com/tokio-rs/axum/blob/main/examples/cors/src/main.rs
      // for more details
      //
      // pay attention that for some request types like posting content-type: application/json
      // it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
      // or see this issue https://github.com/tokio-rs/axum/issues/849
      CorsLayer::new()
      .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
      .allow_methods([Method::GET]),
    );
  
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    //tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
  });

  // share the current runtime with Tauri
  tauri::async_runtime::set(tokio::runtime::Handle::current());

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_image_raw_data])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

// basic handler that responds with a static string
async fn http_get_image_raw_data(payload: Query<ImageQuery>) -> Result<(StatusCode, Json<ImageResult>), (StatusCode, String)> {

  let frame_number = payload.frame_number;
  let canvas_w = payload.canvas_w;
  let canvas_h = payload.canvas_h;

  // Number pf pads (0) for the image number.
  let n_pads = 3;

  println!("{}x{}", canvas_w, canvas_h);

  // TODO: Make sure that this number does not lead to a security issue.
  // Set the image to load
  let img_name = format!("C:/Users/marqu/Resilio Sync/potato/programming/Tests/tauri_test/tauri-canvas/public/jpg-seq/ezgif-frame-{:0n_pads$}.jpg", frame_number, n_pads = n_pads);
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
  println!("dimensions {:?}", img.dimensions());

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
  //Ok((img_raw_data, frame_number, img.dimensions()))

  let image_r = ImageResult {
    image_raw_data: img_raw_data,
    frame_number: frame_number,
    img_dimensions: img.dimensions(),
  };

  Ok( (StatusCode::OK, Json(image_r)) )
  //Ok(23)
  //(StatusCode::OK, format!("YES!"))
}

#[derive(Deserialize)]
struct ImageQuery {
  frame_number: u32,
  canvas_w: u32,
  canvas_h: u32,
}

// (Vec<u8>, u32, (u32, u32))
// Ok((img_raw_data, frame_number, img.dimensions()))
#[derive(Serialize)]
struct ImageResult {
  image_raw_data: Vec<u8>,
  frame_number: u32,
  img_dimensions: (u32, u32),
}