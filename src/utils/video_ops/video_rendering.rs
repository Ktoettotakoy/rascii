use std::process::Command;
use std::fs;
use log::{debug};

use crate::utils::image_to_ascii::image_to_ascii;
use crate::utils::image_ops::image_filters::resize_image_simple;
use crate::utils::image_ops::image_rendering::render_ascii_to_image;
use crate::utils::font_utils::get_embedded_font;
use crate::utils::timer::timer_debug;

/// old
pub fn process_video_to_ascii(
    input: &str,
    output: &str,
    width: u32,
    height: u32,
    char_width: u32,
    style: Option<u8>,
    f_size: f32,
) {
    // To ensure a clean slate for each run.
    remove_folder_if_exists("frames");
    remove_folder_if_exists("ascii_frames");

    // Create frames directory
    let frames_dir = "frames";
    fs::create_dir_all(frames_dir).unwrap();

    debug!("Extracting frames from video: {}", input);

    timer_debug("Extracting frames", || {
        Command::new("ffmpeg")
            .args(["-i", input, "-vf", "fps=10", &format!("{}/frame_%04d.png", frames_dir)])
            .status()
            .expect("Failed to extract video frames");
    });

    debug!("Generating ASCII frames to: {}", frames_dir);
    let paths = fs::read_dir(frames_dir).unwrap();
    let mut frame_paths: Vec<_> = paths.map(|p| p.unwrap().path()).collect();
    frame_paths.sort();


    let ascii_frames_dir = "ascii_frames";
    fs::create_dir_all(ascii_frames_dir).unwrap();

    timer_debug("Generating ASCII frames", || {
        for (i, path) in frame_paths.iter().enumerate() {
            let ascii = image_to_ascii(resize_image_simple(path.to_str().unwrap(), char_width), style);
            let ascii_img = render_ascii_to_image(&ascii, &get_embedded_font(), width, height, f_size);
            let out_frame = format!("{}/ascii_{:04}.png", ascii_frames_dir, i);
            ascii_img.save(&out_frame).expect("Failed to save ASCII frame");
        }
    });

    debug!("ASCII frames converting back to the video in: {}", ascii_frames_dir);
    timer_debug("Converting ASCII frames to video", || {
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
    });
}

fn remove_folder_if_exists(path: &str) {
    if fs::metadata(path).is_ok() {
        fs::remove_dir_all(path).expect("Failed to remove existing frames directory");
    }
}
