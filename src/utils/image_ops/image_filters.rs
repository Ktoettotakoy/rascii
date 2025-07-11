use image::imageops::FilterType;

pub fn resize_image_simple(path: &str, width: u32) -> image::GrayImage {
    let img = image::open(path).expect("Failed to open image").to_luma8();
    let (orig_width, orig_height) = img.dimensions();
    let aspect_ratio = orig_height as f32 / orig_width as f32;
    let height = (width as f32 * aspect_ratio * 0.55) as u32; // Adjust for terminal font ratio
    image::imageops::resize(&img, width, height, FilterType::Nearest)
}
