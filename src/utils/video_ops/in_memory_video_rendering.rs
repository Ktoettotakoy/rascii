use opencv::{videoio, prelude::*};
use opencv::core::{Mat, CV_8UC3, Size};
use crate::utils::image_to_ascii::image_to_ascii;
use crate::utils::image_ops::image_filters::resize_image_dynamic;
use crate::utils::image_ops::image_rendering::render_ascii_to_image;
use crate::utils::font_utils::get_embedded_font;
use image::{DynamicImage, ImageBuffer, Rgb};
use log::{debug, info};
use rayon::prelude::*;
use std::sync::Arc;

// Struct to hold frame data in order
#[derive(Clone)]
struct FrameData {
    index: usize,
    mat: Mat,
}

// Struct to hold processed frame in order
struct ProcessedFrame {
    index: usize,
    mat: Mat,
}

pub fn process_video_to_ascii_opencv(
    input_path: &str,
    output_path: &str,
    width: u32,
    height: u32,
    char_width: u32,
    style: Option<u8>,
    fps: Option<f64>,
    font_size: f32,
) {
    let mut capture = videoio::VideoCapture::from_file(input_path, videoio::CAP_ANY).unwrap();
    assert!(capture.is_opened().unwrap(), "Failed to open video file");

    let input_fps = capture.get(videoio::CAP_PROP_FPS).unwrap_or(30.0);
    let target_fps = fps.unwrap_or(input_fps);
    let frame_interval = (input_fps / target_fps).round() as usize;

    debug!(
        "Input FPS: {:.2}, Target FPS: {:.2}, Frame interval: {}",
        input_fps, target_fps, frame_interval
    );

    // Read all frames that need processing
    info!("Reading frames from video...");
    let frames_to_process = read_frames_for_processing(&mut capture, frame_interval);
    let total_frames = frames_to_process.len();
    info!("Read {} frames for processing", total_frames);

    // Process frames in parallel using Rayon
    info!("Processing frames...");
    let font = Arc::new(get_embedded_font());

    let processed_frames: Vec<ProcessedFrame> = frames_to_process
        .into_par_iter()
        .map(|frame_data| {
            let font_clone = Arc::clone(&font);
            process_single_frame(frame_data, char_width, style, font_clone, width, height, font_size)
        })
        .collect();

    // Sort processed frames by original index to maintain order
    let mut sorted_frames = processed_frames;
    sorted_frames.sort_by_key(|f| f.index);

    // Write frames to output video
    info!("Writing processed frames to video...");
    write_processed_frames_to_video(sorted_frames, output_path, target_fps, width, height);

    capture.release().unwrap();
    info!("Video processing completed successfully");
}

fn read_frames_for_processing(
    capture: &mut videoio::VideoCapture,
    frame_interval: usize
) -> Vec<FrameData> {
    let mut frames = Vec::new();
    let mut frame = Mat::default();
    let mut frame_index = 0;

    while capture.read(&mut frame).unwrap() && !frame.empty() {
        if should_process_frame(frame_index, frame_interval) {
            // Clone the frame data to store it
            let frame_clone = frame.clone();
            frames.push(FrameData {
                index: frame_index,
                mat: frame_clone,
            });
        }
        frame_index += 1;
    }

    frames
}

fn process_single_frame(
    frame_data: FrameData,
    char_width: u32,
    style: Option<u8>,
    font: Arc<rusttype::Font<'static>>, // Adjust type based on your font type
    width: u32,
    height: u32,
    font_size: f32,
) -> ProcessedFrame {
    // Convert OpenCV Mat to RGB image
    let img = mat_to_rgb_image(&frame_data.mat);

    // Resize image
    let resized = resize_image_dynamic(&DynamicImage::ImageRgb8(img), char_width);

    // Convert to grayscale
    let gray = resized.to_luma8();

    // Generate ASCII representation
    let ascii = image_to_ascii(gray, style);

    // Render ASCII back to image
    let ascii_img = render_ascii_to_image(&ascii, &font, width, height, font_size);

    // Convert back to OpenCV Mat
    let mat_frame = rgb_image_to_mat(&ascii_img);

    ProcessedFrame {
        index: frame_data.index,
        mat: mat_frame,
    }
}

fn write_processed_frames_to_video(
    frames: Vec<ProcessedFrame>,
    output_path: &str,
    fps: f64,
    width: u32,
    height: u32,
) {
    let fourcc = videoio::VideoWriter::fourcc('m', 'p', '4', 'v').unwrap();
    let mut writer = videoio::VideoWriter::new(
        &("opencv_".to_owned() + output_path),
        fourcc,
        fps,
        Size::new(width as i32, height as i32),
        true,
    ).unwrap();

    for processed_frame in frames {
        writer.write(&processed_frame.mat).unwrap();
    }

    writer.release().unwrap();
}


fn mat_to_rgb_image(mat: &opencv::core::Mat) -> image::RgbImage {
    let size = mat.size().unwrap();
    let rows = size.height;
    let cols = size.width;
    let mut img_buf = ImageBuffer::new(cols as u32, rows as u32);

    for y in 0..rows {
        for x in 0..cols {
            let pixel = mat.at_2d::<opencv::core::Vec3b>(y, x).unwrap();
            img_buf.put_pixel(x as u32, y as u32, Rgb([pixel[2], pixel[1], pixel[0]])); // BGR to RGB
        }
    }

    img_buf
}

fn rgb_image_to_mat(img: &image::RgbImage) -> opencv::core::Mat {
    let (width, height) = img.dimensions();
    let mat_expr = Mat::zeros(height as i32, width as i32, CV_8UC3).unwrap();
    let mut mat = mat_expr.to_mat().unwrap();

    for (y, row) in img.rows().enumerate() {
        for (x, pixel) in row.enumerate() {
            let bgr = opencv::core::Vec3b::from([pixel[2], pixel[1], pixel[0]]);
            *mat.at_2d_mut::<opencv::core::Vec3b>(y as i32, x as i32).unwrap() = bgr;
        }
    }

    mat
}

fn should_process_frame(frame_index: usize, frame_interval: usize) -> bool {
    frame_index % frame_interval == 0
}
