use std::process::{Command, Stdio};
use std::str;
use std::io::BufReader;

fn main() {

    println!("{:?}", std::env::current_exe());

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
    let mut cmd = Command::new("tmp/ffmpeg.exe")
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

    let tmp_pipe = cmd.wait_with_output().expect("Error wjile waiting");
    println!("# Result --> ");
    println!("Status: {:?}", tmp_pipe.status);
    //println!("Stdout: {:?}", tmp_pipe.stdout);
    println!("Stderr: {:?}", str::from_utf8(&tmp_pipe.stderr) );
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
