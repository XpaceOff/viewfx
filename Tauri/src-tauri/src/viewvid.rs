use axum::{
    http::{ StatusCode },
    Json, 
    extract::Query
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MetadataQuery {
    video_full_path: String,
}

#[derive(Serialize)]
pub struct VideoMetadata {
    width: u32,
    height: u32,
    fps: f64,
    timecode: String,
    frame_length: usize,
}

#[cfg(windows)]
fn no_console(cmd: &mut Command) -> &mut Command {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    cmd.creation_flags(CREATE_NO_WINDOW);
    cmd
}

#[cfg(not(windows))]
fn no_console(cmd: &mut Command) -> &mut Command {
    cmd
}

//use std::os::windows::process::CommandExt;
use regex::Regex;
use std::path::Path;
pub async fn http_get_vid_metadata(payload: Query<MetadataQuery>) -> Result<(StatusCode, Json<VideoMetadata>), (StatusCode, String)> {
    
    // Image metadata that will be returned at the end.
    let mut img_metadata = VideoMetadata {
        width: 0,
        height: 0,
        fps: 0.0,
        timecode: "".to_string(),
        frame_length: 0,
    };

    println!("Video Path: {:?}", &payload.video_full_path);

    if !(Path::new(&payload.video_full_path).is_file()){
        return Err( (StatusCode::BAD_REQUEST, format!("Error: wrong path. \n Path: {:?}", &payload.video_full_path)) );
    }

    println!("# Calling CMD TEST!");
    // TODO: get the right directory for ffmpeg. ffmpeg license TBD.
    // Execute ffmpeg
    //let ffmpeg_command = "tmpffmpeg/ffmpeg.exe";
    //let ffmpeg_command = "/Users/spacecomet/Downloads/ffmpeg";
    //let ffmpeg_command = "bin/ffmpeg";
    let cmd_output = match tauri::api::process::Command::new_sidecar("./ffmpeg").unwrap()
        .args([
            "-i",
            &payload.video_full_path,
            "-vf",
            "select=eq(n\\,10)",
            "-f",
            "rawvideo",
            "-pix_fmt",
            "rgba",
            "-vframes",
            "1",
            "-"
        ])
        //.args(["-version"])
        .output(){
            Ok(out) => out,
            Err(err) => return Err( (StatusCode::INTERNAL_SERVER_ERROR, format!("Error executing FFmpeg. \n{:?}", err)) )
        };

    println!("# CMD called.");

    println!("{:?}", cmd_output.stderr);

    let err_output = cmd_output.stderr.lines();

    // Variables to help me get the frame length.
    let mut hrs: f64 = 0.0;
    let mut min: f64 = 0.0;
    let mut sec: f64 = 0.0;
    let mut dec_sec: f64 = 0.0;

    // Regex Filter Object
    let rx_filter = Regex::new(r" (\d+)x(\d+),([[:ascii:]]+) (\d+(?:\.\d+)?) fps").unwrap();
    for n_line in err_output {

        // Width, Height, and FPS are stored in a line that contains the word "Stream".
        if n_line.contains("Stream "){
            //println!("# Stream line: {:?}", n_line);

            let tmp_meta = match rx_filter.captures(n_line) {
                Some(r) => {
                    //println!("# Stream capture: {:#?}", r);
                    let mut r_meta = VideoMetadata { width: 0, height: 0, fps: 0.0, timecode: "".to_string(), frame_length: 0 };
                    if r.len() == 5{
                        r_meta = VideoMetadata{
                            width: r.get(1).unwrap().as_str().parse().unwrap(),
                            height:  r.get(2).unwrap().as_str().parse().unwrap(),
                            fps:  r.get(4).unwrap().as_str().parse().unwrap(),
                            timecode: "".to_string(),
                            frame_length: 0,
                        };

                        let tmp_total_frames: f64 = (hrs * 60.0 * 60.0 * r_meta.fps) + (min * 60.0 * r_meta.fps) + (sec * r_meta.fps) + (dec_sec * 0.01 * r_meta.fps);
                        r_meta.frame_length = tmp_total_frames as usize;
                    }

                    r_meta
                },
                _ => {
                    let r_meta = VideoMetadata{ width: 0, height: 0, fps: 0.0, timecode: "".to_string(), frame_length: 0 };
                    r_meta
                }
            };

            if tmp_meta.width > 0 && tmp_meta.height > 0 && tmp_meta.fps > 0.0 {
                img_metadata = VideoMetadata {
                    timecode: img_metadata.timecode,
                    ..tmp_meta
                };
            }
        }

        if n_line.contains("Duration"){
            //println!("# 'Duration' line: {:?}", n_line);
            let rx_timecode = Regex::new(r"Duration: ((\d+):(\d+):(\d+).(\d+)),").unwrap();

            let _tmp_timecode = match rx_timecode.captures(n_line){
                Some(r) => {
                    //println!("# Duration capture: {:#?}", r);
                    hrs = r.get(2).unwrap().as_str().parse().unwrap();
                    min = r.get(3).unwrap().as_str().parse().unwrap();
                    sec = r.get(4).unwrap().as_str().parse().unwrap();
                    dec_sec = r.get(5).unwrap().as_str().parse().unwrap();

                    img_metadata.timecode = r.get(1).unwrap().as_str().parse().unwrap();

                    ()
                },
                _ => (),
            };
        }
    }

    // Print Metadata
    println!("# Metadata: {:?}x{:?} @{:?}fps, {:?} / {:?} frames", img_metadata.width, img_metadata.height, img_metadata.fps, img_metadata.timecode, img_metadata.frame_length);
    
    Ok( (StatusCode::OK, Json(img_metadata)) )
}

use std::process::{Command, Stdio};

// Read image and returns it through the http bridge
pub async fn http_get_vid_frame(payload: Query<VidFrameQuery>) -> Result<(StatusCode, Json<ImageResult>), (StatusCode, String)> {
    use std::time::Instant;
    let start_time = Instant::now();

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

    let sidecar_ffmpeg = crate::viewos::relative_command_path("./ffmpeg".to_string()).unwrap();
    println!(" sidecar: {:#?}", sidecar_ffmpeg);

    println!("# Calling CMD");

    // TODO: get the right directory for ffmpeg. ffmpeg license TBD.
    // Execute ffmpeg
    let cmd = match no_console(Command::new(sidecar_ffmpeg)
        .args([
            //"-hwaccel",
            //"cuda",
            //"-hwaccel_output_format",
            //"cuda",
            //"-init_hw_device",
            //"vulkan",
            "-i",
            &img_full_path,
            "-vf",
            &format!("select=eq(n\\,{})", frame_number),
            //"-filter_complex",
            // format!("[0:v]select=eq(n\\,{}), scale={}:{}, extractplanes=r", frame_number, canvas_w, canvas_h),
            //"extractplanes=r+g+b+a",
            "-f",
            "rawvideo",
            //"image2pipe",
            "-pix_fmt",
            "rgba",
            "-vframes",
            "1",
            "-"
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped()))
        .spawn(){
            Ok(out) => out,
            Err(err) => return Err( (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", err)) )
        };

    println!("# CMD called. Waiting...");

    // Wait for the command to complete.
    let cmd_result = match cmd.wait_with_output(){
        Ok(out) => out,
        Err(err) => return Err( (StatusCode::INTERNAL_SERVER_ERROR , format!("Error while waiting for ffmpeg. {:?}", err) ))
    };

    // Once it's completed transform the stderr ( where the metadata info is stored in )
    let err_output = match std::str::from_utf8(&cmd_result.stderr) {
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
    
    let image_r = ImageResult {
        image_raw_data: cmd_result.stdout,
        frame_number: frame_number,
        //img_dimensions: (canvas_w, canvas_h),
        img_dimensions: img_dimensions, //img.dimensions(),
    };

    println!("Pixels length: {}", &image_r.image_raw_data.len());

    let end_time = start_time.elapsed();
    println!("{:.3?}", end_time);
    
    // Return the image
    Ok( (StatusCode::OK, Json(image_r)) )

}

#[derive(Deserialize)]
pub struct VidFrameQuery {
    load_full_img: bool,
    img_full_path: String,
    frame_number: u32,
    canvas_w: u32,
    canvas_h: u32,
}

#[derive(Serialize)]
pub struct ImageResult {
    image_raw_data: Vec<u8>,
    frame_number: u32,
    img_dimensions: (u32, u32),
}