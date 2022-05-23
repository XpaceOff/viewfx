#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
async fn get_image_raw_data(frame_number: u32, canvas_w: u32, canvas_h: u32) -> Result<(Vec<u8>, u32, (u32, u32)), String> {
  use image::GenericImageView;

  println!("{}x{}", canvas_w, canvas_h);

  // TODO: Make sure that this number does not lead to a security issue.
  // Set the image to load
  let img_name = format!("C:/Users/marqu/Resilio Sync/potato/programming/Tests/tauri_test/tauri-canvas/public/jpg-seq/ezgif-frame-00{}.jpg", frame_number);
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

#[tauri::command]
fn my_custom_command() -> Result<Vec<Vec<u8>>, String> {
  use image::GenericImageView;

  let image_name_list: Vec<&str> = vec![
    "C:/Users/marqu/Resilio Sync/potato/programming/Tests/tauri_test/tauri-canvas/public/jpg-seq/ezgif-frame-001.jpg",
    "C:/Users/marqu/Resilio Sync/potato/programming/Tests/tauri_test/tauri-canvas/public/jpg-seq/ezgif-frame-002.jpg",
    "C:/Users/marqu/Resilio Sync/potato/programming/Tests/tauri_test/tauri-canvas/public/jpg-seq/ezgif-frame-003.jpg",
    "C:/Users/marqu/Resilio Sync/potato/programming/Tests/tauri_test/tauri-canvas/public/jpg-seq/ezgif-frame-004.jpg",
    "C:/Users/marqu/Resilio Sync/potato/programming/Tests/tauri_test/tauri-canvas/public/jpg-seq/ezgif-frame-005.jpg"
  ];

  let mut r_frames: Vec<Vec<u8>> = Vec::new();

  for img_name in 0..image_name_list.len(){
    println!("{}", image_name_list[img_name]);

    let img = image::open(image_name_list[img_name]);
    let img = match img {
      Ok(tmp_image) => tmp_image,
      Err(_error) => return Err("Error reading file".into()),
    };

    //let img = img.resize(800, 400, image::imageops::Lanczos3);
    let img = img.thumbnail(800, 400);
  
    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());
  
    // The color method returns the image's `ColorType`.
    //println!("{:?}", img.color());
  
    //let tmp_vector: Vec<i32> = vec![32, 2, 3];
    //let payload = BootPayload { drives: tmp_vector };
  
    let mut img_raw_data = Vec::new();
    for (_x, _y, pixel) in img.pixels() {
      // Do something with pixel.
      //println!("{:?}", pixel.0[0]);
      img_raw_data.push(pixel.0[0]);
      img_raw_data.push(pixel.0[1]);
      img_raw_data.push(pixel.0[2]);
      img_raw_data.push(pixel.0[3]);
    }

    r_frames.push(img_raw_data);
    
  }
  
  println!("Data transformed!");
  //Ok("Hello from Rust!".into())
  Ok(r_frames)
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![my_custom_command, get_image_raw_data])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}