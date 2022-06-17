use core::panic;
use std::process::{Command, Stdio};
use std::str;
use regex::Regex;
//use std::io::BufReader;

struct ImgMetadata {
    width: u32,
    height: u32,
    fps: u16,
}

fn main() {

    // Empty image metadata
    let mut img_metadata = ImgMetadata{
        width: 0,
        height: 0,
        fps: 0,
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
        .arg("image2pipe")
        .arg("-vframes")
        .arg("1")
        .arg("-")
        .stdout(Stdio::null())
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
        if n_line.contains("Stream "){
            let tmp_meta = match rx_filter.captures(n_line) {
                Some(r) => {
                    let mut r_meta = ImgMetadata{ width: 0, height: 0, fps: 0 };
                    if r.len() == 5{
                        r_meta = ImgMetadata{
                            width: r.get(1).unwrap().as_str().parse().unwrap(),
                            height:  r.get(2).unwrap().as_str().parse().unwrap(),
                            fps:  r.get(4).unwrap().as_str().parse().unwrap(),
                        };
                    }

                    r_meta
                    
                },
                _ => {
                    let r_meta = ImgMetadata{ width: 0, height: 0, fps: 0 };
                    r_meta
                }
            };

            if tmp_meta.width > 0 && tmp_meta.height > 0 && tmp_meta.fps > 0 {
                img_metadata = tmp_meta;
                
            }
        }
    }

    // Print Metadata
    println!("# Matadata: {:?}x{:?} @{:?}fps", img_metadata.width, img_metadata.height, img_metadata.fps);

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
