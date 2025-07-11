use image::{RgbImage, Rgb};
use rusttype::{Font, Scale};

pub fn render_ascii_to_image(ascii: &str, font: &Font, img_width: u32, img_height: u32, font_size: f32) -> RgbImage {
    let scale = Scale::uniform(font_size);
    let v_metrics = font.v_metrics(scale);

    let mut image = RgbImage::new(img_width, img_height);
    let _white = Rgb([255, 255, 255]);
    // let blueish = Rgb([32, 50, 60]);
    let lighter_black = Rgb([8, 8, 8]);
    let _black = Rgb([0, 0, 0]);

    // Fill background
    fill_background(&mut image, lighter_black);

    let start_x = 10;
    let mut y = 10 + v_metrics.ascent as u32;

    for line in ascii.lines() {
        let mut x = start_x;

        for ch in line.chars() {
            if let Some(_glyph) = font.glyph(ch).scaled(scale).positioned(rusttype::point(x as f32, y as f32)).pixel_bounding_box() {
                font.glyph(ch)
                    .scaled(scale)
                    .positioned(rusttype::point(x as f32, y as f32))
                    .draw(|gx, gy, v| {
                        let px = x + gx;
                        let py = y + gy;
                        if px < img_width && py < img_height {
                            let value = (v * 255.0) as u8;
                            image.put_pixel(px, py, Rgb([value, value, value]));
                        }
                    });
            }
            x += (font_size * 0.6) as u32;
        }

        y += (font_size * 1.2) as u32;
    }

    image
}

fn fill_background(image: &mut RgbImage, color: Rgb<u8>) {
    for pixel in image.pixels_mut() {
        *pixel = color;
    }
}
