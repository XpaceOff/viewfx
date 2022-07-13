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

use std::process::{Command, Stdio};
//use std::os::windows::process::CommandExt;
use regex::Regex;
use std::path::Path;
pub async fn http_get_vid_metadata(payload: Query<MetadataQuery>) -> Result<(StatusCode, Json<VideoMetadata>), (StatusCode, String)> {
    let mut img_metadata = VideoMetadata {
        width: 0,
        height: 0,
        fps: 0.0,
        timecode: "".to_string(),
        frame_length: 0,
    };

    const CREATE_NO_WINDOW: u32 = 0x08000000;

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

    // Variables that help me get the frame length.
    let mut hrs: f64 = 0.0;
    let mut min: f64 = 0.0;
    let mut sec: f64 = 0.0;
    let mut dec_sec: f64 = 0.0;

    // Regex Filter Object
    let rx_filter = Regex::new(r" (\d+)x(\d+),([[:ascii:]]+) (\d+) fps").unwrap();
    for n_line in err_output {

        // Width, Height, and FPS are stored in a line that contains the word "Stream".
        if n_line.contains("Stream "){
            let tmp_meta = match rx_filter.captures(n_line) {
                Some(r) => {
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
                img_metadata = tmp_meta;
            }
        }

        if n_line.contains("Duration"){
            let rx_timecode = Regex::new(r"Duration: ((\d+):(\d+):(\d+).(\d+)),").unwrap();

            let _tmp_timecode = match rx_timecode.captures(n_line){
                Some(r) => {
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