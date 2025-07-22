use opencv::{videoio, prelude::*};
use opencv::core::{Mat, CV_8UC3, Size};
use crate::utils::image_to_ascii::image_to_ascii;
use crate::utils::image_ops::image_filters::resize_image_dynamic;
use crate::utils::image_ops::image_rendering::render_ascii_to_image;
use crate::utils::font_utils::get_embedded_font;
use image::{DynamicImage, ImageBuffer, Rgb};

pub fn process_video_to_ascii_opencv(
    input_path: &str,
    output_path: &str,
    width: u32,
    height: u32,
    char_width: u32,
    style: Option<u8>,
    font_size: f32,
) {
    let mut capture = videoio::VideoCapture::from_file(input_path, videoio::CAP_ANY).unwrap();
    assert!(capture.is_opened().unwrap(), "Failed to open video file");

    let fps = capture.get(videoio::CAP_PROP_FPS).unwrap();
    // let target_fps = 10.0;
    // let frame_interval = (fps / target_fps).round() as usize;

    let fourcc = videoio::VideoWriter::fourcc('m', 'p', '4', 'v').unwrap();

    let mut writer = videoio::VideoWriter::new(
        &("opencv_".to_owned() + output_path),
        fourcc,
        fps,
        Size::new(width as i32, height as i32),
        true,
    )
    .unwrap();

    let font = get_embedded_font();
    let mut frame = Mat::default();
    // let mut frame_index = 0;
    while capture.read(&mut frame).unwrap() && !frame.empty() {
        // if frame.empty() {
        //     continue;
        // }

        // if !should_process_frame(frame_index, frame_interval) {
        //     frame_index += 1;
        //     continue;
        // }

        let img = mat_to_rgb_image(&frame);
        let resized = resize_image_dynamic(&DynamicImage::ImageRgb8(img), char_width);
        let gray = resized.to_luma8();
        let ascii = image_to_ascii(gray, style);
        let ascii_img = render_ascii_to_image(&ascii, &font, width, height, font_size);
        let mat_frame = rgb_image_to_mat(&ascii_img);
        writer.write(&mat_frame).unwrap();

        // frame_index += 1;
    }

    writer.release().unwrap();
    capture.release().unwrap();
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
