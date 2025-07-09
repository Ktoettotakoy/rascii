use clap::{Parser, Subcommand};
// use image::GenericImageView;
use image::imageops::FilterType;

use rascii::image_rendering::render_ascii_to_image;
use rascii::utils::embedded_font::get_embedded_font;

const ASCII_CHARS: &[u8] = b"@%#*+=-:. ";
const ASCII_CHARS_EXTENDED: &[u8] = b"@%#*+=-:.      ";
const ASCII_CHARS_INVERTED: &[u8] = b" .:-=+*#%@";
const ASCII_CHARS_MODERATE_DETAIL: &[u8] = b"   .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
const ASCII_CHARS_MODERATE_DETAIL_INVERTED: &[u8] = b"$@%B&8W#*oahkbdpqmwZO0QLCJUYXzcvunxrjft\\/|)(1}{][?-_+~><i!lI;:,\"^`\'. ";

/// ASCII Art Generator
#[derive(Parser, Debug)]
#[command(name = "ascii-art")]
#[command(author = "Yaroslav", version, about = "Generate ASCII art from images")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Convert an image to ASCII
    Convert {
        /// Path to the input image
        #[arg(short, long)]
        input: String,

        /// Output width
        #[arg(short, long, default_value_t = 100)]
        width: u32,

        /// Style of ASCII art (0 for basic, 1 for extended, 2 for inverted...)
        /// Default is 0 (basic)
        #[arg(short, long)]
        style: Option<u8>,

        /// Output file (optional)
        #[arg(short, long)]
        output: Option<String>,
    },
}

fn main() {
 let cli = Cli::parse();

    match &cli.command {
        Commands::Convert { input, width, style, output } => {
            println!("Input file: {}", input);
            println!("Width: {}", width);


            if let Some(out) = output {
                println!("Saving output to: {}", out);
                let ascii = image_to_ascii(resize_image(input, *width), *style);
                let img = render_ascii_to_image(
                    &ascii,
                    &get_embedded_font(),
                    2560,
                    1440,
                    9.0,
                );
                img.save(out).expect("Failed to save image");
            } else {
                println!("Printing ASCII to stdout");
                println!("{}",image_to_ascii(resize_image(input, *width), *style));
            }
        }
    }
}

// Convert an image to ASCII art
// It returns a string into the stdout containing the ASCII art representation of the image.
// The image is resized to the specified width while maintaining the aspect ratio.
fn image_to_ascii(img: image::GrayImage, style: Option<u8>) -> String {
    let mut ascii = String::new();

    let ascii_set: &[u8] = if style.is_some() {
        match style.unwrap() {
            1 => ASCII_CHARS_EXTENDED,
            2 => ASCII_CHARS_INVERTED,
            3 => ASCII_CHARS_MODERATE_DETAIL,
            4 => ASCII_CHARS_MODERATE_DETAIL_INVERTED,
            _ => ASCII_CHARS,
        }
    } else {
        ASCII_CHARS
    };

    println!("Using ASCII set: {:?}", std::str::from_utf8(ascii_set).expect("Invalid UTF-8 data").chars());
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y)[0];
            ascii.push(pixel_to_ascii(pixel, ascii_set));
        }
        ascii.push('\n');
    }
    ascii
}

fn resize_image(path: &str, width: u32) -> image::GrayImage {
    let img = image::open(path).expect("Failed to open image").to_luma8();
    let (orig_width, orig_height) = img.dimensions();
    let aspect_ratio = orig_height as f32 / orig_width as f32;
    let height = (width as f32 * aspect_ratio * 0.55) as u32; // Adjust for terminal font ratio
    image::imageops::resize(&img, width, height, FilterType::Nearest)
}

fn pixel_to_ascii(pixel: u8, ascii_set: &[u8]) -> char {
    let index = (pixel as f32 / 255.0 * (ascii_set.len() - 1) as f32) as usize;
    ascii_set[index] as char
}
