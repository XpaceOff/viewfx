#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Use Tauri States to set and then get the address:port
use tauri::State;
struct BgAddrState(String);
struct BgHashState(String);

// Function that return the backend address and port.
#[tauri::command]
fn get_bg_addr(state: State<BgAddrState>) -> String {
    println!("# ip:port: {}", state.0);
    state.0.clone()
}

// Function that return the backend hash.
#[tauri::command]
fn get_bg_hash(hash_state: State<BgHashState>) -> String {
    //println!("hash: {}", hash_state.0);
    hash_state.0.clone()
}

use axum::{
  routing::{ get },
  http::{StatusCode, Method, HeaderValue},
  Json, Router, 
  extract::Query
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use image::GenericImageView;
use tower_http::cors::CorsLayer;
use tower_http::auth::RequireAuthorizationLayer;
use tower::ServiceBuilder;
use std::sync::{Arc, Mutex};
use http::header::{AUTHORIZATION};

mod viewvid;
mod viewos;
mod viewhash;

// Make the main function Async with Tokio
#[tokio::main]
async fn main() {
    // variable containing addr:port of backend.
    let addr_data = Arc::new(Mutex::new(String::from("")));
    let addr_data_clone= Arc::clone(&addr_data); // clone it to be able to access it on different threads.

    // Auth token.
    let bridge_hash = viewhash::generate_hash();
    let bridge_hash02 = bridge_hash.clone();

    // Create a new thread that will run the backend-api/http-bridge
    // to load Images. This solve the bottleneck on Tauri's IPC
    // Check https://github.com/tauri-apps/tauri/discussions/4191
    // for more info about the problem
    tokio::spawn(async move {
        // build our application with routes
        let app = Router::new()
        .route("/", get(|| async { "ViewFX" }))
        .route("/image_raw_data", get(http_get_image_raw_data))
        //.route("/video_metadata", get(http_get_video_metadata))
        .route("/video_metadata", get(viewvid::http_get_vid_metadata))
        .route("/video_frame", get(viewvid::http_get_vid_frame))
        .layer(
            ServiceBuilder::new()
            .layer(
                // CORS Middleware that solves the CORS problem
                // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
                // and https://github.com/tokio-rs/axum/blob/main/examples/cors/src/main.rs
                // for more details

                // TODO: You might need a way to get the port that Tauri is running on.
                // If not then the app won't work because of the CORS problem
                CorsLayer::new()
                .allow_origin(vec![
                    "http://localhost:8080".parse::<HeaderValue>().unwrap(),    // Dev Mode
                    "https://tauri.localhost".parse::<HeaderValue>().unwrap(),  // Win build mode
                    "tauri://localhost".parse::<HeaderValue>().unwrap()         // Macos built mode
                ])
                .allow_methods([Method::GET])
                .allow_headers([AUTHORIZATION]),
            )
            .layer(RequireAuthorizationLayer::bearer(&bridge_hash02))
        );
  
        // Set the port to `0` to get a random unused port.
        let addr = SocketAddr::from(([127, 0, 0, 1], 0));
        
        // run our app with hyper
        // `axum::Server` is a re-export of `hyper::Server`
        let bg_server = axum::Server::bind(&addr)
        .serve(app.into_make_service());
        //.await.unwrap();

        let bg_addr = bg_server.local_addr().to_string().clone();

        // Make a new scope to lock `addr_data_clone`; get the value, and then drop it.
        // If these {} are removed then the code won't compile because `addr_data_clone` won't be unlocked.
        // therefore you won't be able to read it later.
        // To force the unlock make a new scope.
        {
            let mut tmp_locked_addr = addr_data_clone.lock().unwrap();
            *tmp_locked_addr = bg_addr.clone();
            println!("# http bridge listening on {}", bg_addr.clone());
        }

        // Start the bridge backend server.
        bg_server.await.unwrap();
    });

    // share the current runtime with Tauri
    // Here https://github.com/tauri-apps/tauri/discussions/4191#discussioncomment-2833069
    // For more info
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    // Wait for backend bridge to have a port and be ready so I can send it to the frontend
    // with Tauri's States.
    let addr_data_clone = Arc::clone(&addr_data);
    let final_bg_addr;
    loop {
        let addr_data_clone = addr_data_clone.lock().unwrap();
        let addr_obtained = &*addr_data_clone;
        
        // Once it changes (gets a port) then build the Tauri app.
        if !(addr_obtained.eq("")) { 
            final_bg_addr = addr_data_clone.clone();
            break;
        }
    }

    tauri::Builder::default()
        //.menu(menu)
        .manage(BgAddrState(final_bg_addr.into())) // Sets the bridge port with Tauri's States
        .manage(BgHashState(bridge_hash.into())) // Sets the hash code with Tauri's States
        .invoke_handler(tauri::generate_handler![get_bg_addr, get_bg_hash])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Read image and returns it through the http bridge
async fn http_get_image_raw_data(payload: Query<ImageQuery>) -> Result<(StatusCode, Json<ImageResult>), (StatusCode, String)> {

    //const CREATE_NO_WINDOW: u32 = 0x08000000;

    use std::time::Instant;
    let start_time = Instant::now();

    let src_img_type = payload.src_img_type.clone();
    let img_full_path = payload.img_full_path.clone();
    
    let load_full_img = payload.load_full_img;
    let frame_number = payload.frame_number;
    let canvas_w = payload.canvas_w;
    let canvas_h = payload.canvas_h;

    println!("# Canva's size is {}x{}", canvas_w, canvas_h);

    // TODO: Make sure that this number does not lead to a security issue.
    // Set the image to load
    println!("# Image full path: {}", img_full_path);

    // Make sure that the img to be load is actually a file.
    if !(Path::new(&img_full_path).is_file()){
        return Err( (StatusCode::BAD_REQUEST, format!("Error: wrong path.")) );
    }

    let image_result = match src_img_type.as_str(){
        "FROM_VIDEO" => { // When the img is loaded from a video.
            println!("# image loaded from a video");

            let sidecar_ffmpeg = viewos::relative_command_path("./ffmpeg".to_string()).unwrap();
            println!(" sidecar: {:#?}", sidecar_ffmpeg);

            println!("# Calling CMD");
            // TODO: get the right directory for ffmpeg. ffmpeg license TBD.
            // Execute ffmpeg
            //let ffmpeg_command = "tmpffmpeg/ffmpeg.exe";
            //let ffmpeg_command = "/Users/spacecomet/Downloads/ffmpeg";
            //let ffmpeg_command = "/usr/local/bin/ffmpeg";
            let ffmpeg_command = "/Applications/viewfx.app/Contents/MacOS/ffmpeg";
            println!("command: {}", ffmpeg_command);
            //let ffmpeg_command = "./ffmpeg";
            let cmd = match Command::new(sidecar_ffmpeg)
                //.arg("-hwaccel")
                //.arg("cuda")
                //.arg("-hwaccel_output_format")
                //.arg("cuda")
                //.arg("-init_hw_device")
                //.arg("vulkan")
                .arg("-i")
                .arg(&img_full_path)
                .arg("-vf")
                .arg( format!("select=eq(n\\,{})", frame_number) )
                //.arg("-filter_complex")
                //.arg( format!("[0:v]select=eq(n\\,{}), scale={}:{}, extractplanes=r", frame_number, canvas_w, canvas_h) )
                //.arg("extractplanes=r+g+b+a")
                .arg("-f")
                .arg("rawvideo")
                //.arg("image2pipe")
                .arg("-pix_fmt")
                .arg("rgba")
                .arg("-vframes")
                .arg("1")
                .arg("-")
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                //.creation_flags(CREATE_NO_WINDOW) // Un-comment this for a Windows build.
                .spawn(){
                    Ok(out) => out,
                    Err(err) => return Err( (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", err)) )
                };

            println!("# CMD called. Waiting...");

            // Wait for the command to complete.
            let tmp_pipe = match cmd.wait_with_output(){
                Ok(out) => out,
                Err(err) => return Err( (StatusCode::INTERNAL_SERVER_ERROR , format!("Error while waiting for ffmpeg. {:?}", err) ))
            };

            println!("# Length: {:?}", tmp_pipe.stdout.len());

            // Once it's completed transform the stderr ( where the metadata info is stored in )
            let err_output = match std::str::from_utf8(&tmp_pipe.stderr) {
                Ok(out) => {out},
                Err(err) => return Err( (StatusCode::INTERNAL_SERVER_ERROR , format!("Error getting metadata. {:?}", err)) )
            };

            // Regex Filter Object - Get image dimensions.
            let rx_filter = Regex::new(r" (\d+)x(\d+),([[:ascii:]]+) (\d+) fps").unwrap();
            let img_dimensions: (u32, u32) = match rx_filter.captures(err_output){
                Some(rx_result) => {
                    if rx_result.len() == 5{
                        (rx_result.get(1).unwrap().as_str().parse().unwrap(), rx_result.get(2).unwrap().as_str().parse().unwrap())
                    }
                    else{
                        return Err( (StatusCode::INTERNAL_SERVER_ERROR , format!("Error getting image resolution.")) )
                    }
                },
                _ => return Err( (StatusCode::INTERNAL_SERVER_ERROR, format!("Error getting image dimensions.")) )
            };

            //println!("# Image dimensions: {:?}", img_dimensions);

            /*let err_output = err_output.lines();
            for i in err_output {
                println!("{}", i);
            }*/
            //println!("{:?}", err_output);

            // TODO: Make sure that this is 100% secure
            // Returns:{
            //   1- An array with the raw data of the current image
            //   2- Current frame number / image number
            //   3- The size to be display on the canvas
            // }
    
            let image_r = ImageResult {
                image_raw_data: tmp_pipe.stdout,
                frame_number: frame_number,
                //img_dimensions: (canvas_w, canvas_h),
                img_dimensions: img_dimensions, //img.dimensions(),
            };
            
            // Return the image
            Ok( (StatusCode::OK, Json(image_r)) )
            //Err( (StatusCode::BAD_REQUEST, format!("Bad request")) )
        },
        "FROM_IMG" => { // When the img is loaded from a image sequence.
            println!("Its loaded from an image");
            
            // Open the image
            let img = image::open(img_full_path);
            let mut img = match img {
                Ok(tmp_image) => tmp_image,
                Err(_error) => return Err( (StatusCode::NOT_FOUND, format!("Image not found")) ),
            };
    
            println!("# Original dimensions {:?}", img.dimensions());
            
            if !load_full_img {
                //let img = img.resize(canvas_w, canvas_h, image::imageops::Lanczos3);
                img = img.thumbnail(canvas_w, canvas_h);
            }
            
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
    
            // Return the image
            Ok( (StatusCode::OK, Json(image_r)) )
        }
        _ => {
            Err( (StatusCode::BAD_REQUEST, format!("Bad request")) )
        }
    };

    let end_time = start_time.elapsed();
    println!("{:.3?}", end_time);

    image_result

}

use std::process::{Command, Stdio};
//use std::os::windows::process::CommandExt;
use regex::Regex;
use std::path::Path;

#[derive(Deserialize)]
struct ImageQuery {
    src_img_type: String,
    load_full_img: bool,
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