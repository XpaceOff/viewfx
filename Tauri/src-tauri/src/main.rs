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

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_image_raw_data])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}