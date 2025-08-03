use image::{RgbImage, Rgb};
use rusttype::{Font, Scale};
use log::{debug};


pub fn render_ascii_to_image(
    ascii: &str,
    font: &Font,
    img_width: u32,
    img_height: u32,
    font_size: f32
) -> RgbImage {
    // debug!("Rendering at resolution: {}x{} px", img_width, img_height);
    // debug!("Character width: {}", font_size);

    let scale = Scale::uniform(font_size);
    let v_metrics = font.v_metrics(scale);


    let lighter_black = Rgb([8, 8, 8]);
    let mut image = RgbImage::from_pixel(img_width, img_height, lighter_black);

    let start_x = 10u32;
    let mut y = 10u32 + v_metrics.ascent as u32;
    let char_width = (font_size * 0.6) as u32;
    let line_height = (font_size * 1.2) as u32;

    for line in ascii.lines() {
        if y >= img_height { break; }

        let mut x = start_x;

        for ch in line.chars() {
            if x + char_width > img_width { break; }

            let positioned_glyph = font.glyph(ch)
                .scaled(scale)
                .positioned(rusttype::point(x as f32, y as f32));

            // Only draw if glyph has pixels
            if positioned_glyph.pixel_bounding_box().is_some() {
                positioned_glyph.draw(|gx, gy, v| {
                    let px = x + gx;
                    let py = y + gy;

                    if px < img_width && py < img_height && v > 0.0 {
                        let value = (v * 255.0) as u8;
                        image.put_pixel(px, py, Rgb([value, value, value]));
                    }
                });
            }

            x += char_width;
        }

        y += line_height;
    }

    image
}
