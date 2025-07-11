use std::process::Command;
use std::fs;

use crate::utils::image_to_ascii::image_to_ascii;
use crate::utils::image_ops::image_filters::resize_image_simple;
use crate::utils::image_ops::image_rendering::render_ascii_to_image;
use crate::utils::embedded_font::get_embedded_font;

pub fn process_video_to_ascii(
    input: &str,
    output: &str,
    char_width: u32,
    style: Option<u8>,
    f_size: f32,
) {
    let frames_dir = "frames";
    fs::create_dir_all(frames_dir).unwrap();

    println!("Extracting frames from video: {}", input);
    Command::new("ffmpeg")
        .args(["-i", input, "-vf", "fps=10", &format!("{}/frame_%04d.png", frames_dir)])
        .status()
        .expect("Failed to extract video frames");

    println!("Generating ASCII frames to: {}", frames_dir);
    let paths = fs::read_dir(frames_dir).unwrap();
    let mut frame_paths: Vec<_> = paths.map(|p| p.unwrap().path()).collect();
    frame_paths.sort();

    let ascii_frames_dir = "ascii_frames";
    fs::create_dir_all(ascii_frames_dir).unwrap();

    for (i, path) in frame_paths.iter().enumerate() {
        let ascii = image_to_ascii(resize_image_simple(path.to_str().unwrap(), char_width), style);
        let ascii_img = render_ascii_to_image(&ascii, &get_embedded_font(), 1920, 1080, f_size);
        let out_frame = format!("{}/ascii_{:04}.png", ascii_frames_dir, i);
        ascii_img.save(&out_frame).expect("Failed to save ASCII frame");
    }

    println!("ASCII frames converting back to the video in: {}", ascii_frames_dir);
    Command::new("ffmpeg")
        .args([
            "-framerate", "10",
            "-i", &format!("{}/ascii_%04d.png", ascii_frames_dir),
            "-c:v", "libx264",
            "-pix_fmt", "yuv420p",
            output
        ])
        .status()
        .expect("Failed to encode ASCII video");

    println!("ASCII video saved to: {}", output);
}
