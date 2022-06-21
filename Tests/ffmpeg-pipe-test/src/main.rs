use core::panic;
use std::process::{Command, Stdio};
use std::str;
use regex::Regex;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
//use std::io::BufReader;

struct ImgMetadata {
    width: u32,
    height: u32,
    fps: u16,
    timecode: String,
    t_frames: usize,
}

fn main() {

    // Empty image metadata
    let mut img_metadata = ImgMetadata {
        width: 0,
        height: 0,
        fps: 0,
        timecode: "".to_string(),
        t_frames: 0,
    };

    println!("# Current exe: {:?}", std::env::current_exe());

    // Command
    // $ .\ffmpeg.exe -i .\SC_07.mov -vf "select=eq(n\,10)" -f image2pipe -vframes 1 -

    /*let img_output = Command::new("tmp/ffmpeg.exe")
        .arg("-i")
        .arg("tmp/SC_07.mov")
        .arg("-vf")
        .arg("select=eq(n\\,10)")
        .arg("-f")
        .arg("image2pipe")
        .arg("-vframes")
        .arg("1")
        .arg("-")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed");*/

    println!("# Calling CMD");
    let cmd = Command::new("tmp/ffmpeg.exe")
        .arg("-i")
        .arg("tmp/SC_07.mov")
        .arg("-vf")
        .arg("select=eq(n\\,10)")
        .arg("-f")
        .arg("rawvideo")
        .arg("-pix_fmt")
        .arg("argb")
        .arg("-vframes")
        .arg("1")
        .arg("-")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("errorr");

    println!("# CMD called. Waiting...");

    let tmp_pipe = cmd.wait_with_output().expect("Error while waiting");

    let err_out = match str::from_utf8(&tmp_pipe.stderr) {
        Ok(out) => {out},
        Err(err) => panic!("Error: {:?}", err),
    };

    let err_out = err_out.lines();

    println!("# Result --> ");
    //println!("Status: {:?}", tmp_pipe.status);
    //println!("Stdout: {:?}", tmp_pipe.stdout);
    //println!("Stderr: {:?}", err_out);

    // Regex Filter Object
    let rx_filter = Regex::new(r" (\d+)x(\d+),([[:ascii:]]+) (\d+) fps").unwrap();
    for n_line in err_out {
        //println!("{:?}", n_line);
        if n_line.contains("Stream "){
            //println!("----------------------------------------- HERE --------");
            let tmp_meta = match rx_filter.captures(n_line) {
                Some(r) => {
                    let mut r_meta = ImgMetadata { width: 0, height: 0, fps: 0, timecode: "".to_string(), t_frames: 0 };
                    if r.len() == 5{
                        r_meta = ImgMetadata{
                            width: r.get(1).unwrap().as_str().parse().unwrap(),
                            height:  r.get(2).unwrap().as_str().parse().unwrap(),
                            fps:  r.get(4).unwrap().as_str().parse().unwrap(),
                            timecode: "".to_string(),
                            t_frames: 0,
                        };
                    }

                    r_meta
                    
                },
                _ => {
                    let r_meta = ImgMetadata{ width: 0, height: 0, fps: 0, timecode: "".to_string(), t_frames: 0 };
                    r_meta
                }
            };

            if tmp_meta.width > 0 && tmp_meta.height > 0 && tmp_meta.fps > 0 {
                img_metadata = tmp_meta;
                
            }
        }

        if n_line.contains("timecode"){
            // Get the timecode only after I get the fps
            if img_metadata.width > 0 && img_metadata.height > 0 && img_metadata.fps > 0{
                let rx_timecode = Regex::new(r"(\d+):(\d+):(\d+):(\d+)").unwrap();

                let _tmp_timecode = match rx_timecode.captures(n_line){
                    Some(r) => {
                        let hrs: usize = r.get(1).unwrap().as_str().parse().unwrap();
                        let min: usize = r.get(2).unwrap().as_str().parse().unwrap();
                        let sec: usize = r.get(3).unwrap().as_str().parse().unwrap();
                        let fra: usize = r.get(4).unwrap().as_str().parse().unwrap();
                        let tmp_fps = img_metadata.fps as usize;

                        let tmp_total_frames = (hrs * 60 * 60 * tmp_fps) + (min * 60 * tmp_fps) + (sec * tmp_fps) + fra;

                        img_metadata.timecode = r.get(0).unwrap().as_str().parse().unwrap();
                        img_metadata.t_frames = tmp_total_frames;

                        //println!("{:?}", tmp_total_frames);
                        //println!("{:?}", r.get(0).unwrap().as_str())
                        ()
                    },
                    _ => (),
                };
                //println!("{:?}", n_line);

            }
        }
    }

    // Print Metadata
    println!("# Matadata: {:?}x{:?} @{:?}fps, {:?} / {:?} frames", img_metadata.width, img_metadata.height, img_metadata.fps, img_metadata.timecode, img_metadata.t_frames);

    // Construct a new RGB ImageBuffer with the specified width and height.
    let mut img: RgbImage = ImageBuffer::new(img_metadata.width, img_metadata.height);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let xx = x as usize;
        let pixel_pos = ((img_metadata.width*y*4) + (x*4)) as usize;
        let r = *tmp_pipe.stdout.get(pixel_pos+1).unwrap() as u8;
        let g = *tmp_pipe.stdout.get(pixel_pos+2).unwrap() as u8;
        let b = *tmp_pipe.stdout.get(pixel_pos+3).unwrap() as u8;
        *pixel = image::Rgb([r, g, b]);
    }

    img.save("test.png").unwrap();

    //println!("All: {:?}", tmp_pipe);

    //let cmd_stdout = BufReader::new(cmd.stdout.unwrap());

    //println!("{:?}", img_output);
    //println!("{:?}", img_output.stdout.as_slice());
    //println!("{:?}", img_output.stderr.as_slice());

    //let mut tmp = std::str::from_utf8(&img_output.stderr);
    //let mut tmp = tmp.split(' ');
    //println!("{:?}", tmp);


    //println!("{:?}", img_output.stdout);
    //println!("{:#?}", cmd_stdout);
    //println!("Length: {:?}", img_output.stdout.len()/4);
}
