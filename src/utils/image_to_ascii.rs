const ASCII_CHARS: &[u8] = b"@%#*+=-:. ";
const ASCII_CHARS_EXTENDED: &[u8] = b"@%#*+=-:.      ";
const ASCII_CHARS_INVERTED: &[u8] = b" .:-=+*#%@";
const ASCII_CHARS_MODERATE_DETAIL: &[u8] = b"   .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
const ASCII_CHARS_MODERATE_DETAIL_INVERTED: &[u8] = b"$@%B&8W#*oahkbdpqmwZO0QLCJUYXzcvunxrjft\\/|)(1}{][?-_+~><i!lI;:,\"^`\'. ";


// Convert an image to ASCII art
// It returns a string into the stdout containing the ASCII art representation of the image.
// The image is resized to the specified width while maintaining the aspect ratio.
pub fn image_to_ascii(img: image::GrayImage, style: Option<u8>) -> String {
    let mut ascii = String::new();

    let ascii_set: &[u8] = match style.unwrap_or(0) {
        1 => ASCII_CHARS_EXTENDED,
        2 => ASCII_CHARS_INVERTED,
        3 => ASCII_CHARS_MODERATE_DETAIL,
        4 => ASCII_CHARS_MODERATE_DETAIL_INVERTED,
        _ => ASCII_CHARS,
    };

    // println!("Using ASCII set: {:?}", std::str::from_utf8(ascii_set).expect("Invalid UTF-8 data").chars());
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y)[0];
            ascii.push(pixel_to_ascii(pixel, ascii_set));
        }
        ascii.push('\n');
    }
    ascii
}

// Convert a pixel value to an ASCII character based on the provided ASCII set.
fn pixel_to_ascii(pixel: u8, ascii_set: &[u8]) -> char {
    let index = (pixel as f32 / 255.0 * (ascii_set.len() - 1) as f32) as usize;
    ascii_set[index] as char
}
