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
#[command (disable_help_flag = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Convert an image to ASCII
    Text {
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
    },
    Image {
        /// Path to the input image
        #[arg(short, long)]
        input: String,

        /// Output resolution: either named (2k, fhd) or custom (e.g. 1920x1080)
        #[arg(short, long)]
        res: String,

        /// Width in characters (columns) for ASCII rendering
        #[arg(short = 'c', long, default_value_t = 100)]
        char_width: u32,

        /// Font size
        #[arg(short = 'f', long, default_value_t = 9.0)]
        f_size: f32,

        /// ASCII art style
        #[arg(short, long)]
        style: Option<u8>,

        /// Output file
        #[arg(short, long, default_value_t = String::from("output.png"))]
        output: String,
    },
}

fn main() {
 let cli = Cli::parse();

    match &cli.command {
        Commands::Text { input, width, style} => {
            println!("Input file: {}", input);
            println!("Width: {}", width);

            println!("Printing ASCII to stdout");
            println!("{}",image_to_ascii(resize_image(input, *width), *style));
        },
         Commands::Image { input, res, char_width, f_size, style, output } => {
            let (width_px, height_px) = parse_resolution(res).unwrap_or_else(|| {
                eprintln!("Invalid resolution: '{}'", res);
                std::process::exit(1);
            });

            println!("Rendering at resolution: {}x{} px", width_px, height_px);
            println!("Character width: {}", char_width);

            let ascii = image_to_ascii(resize_image(input, *char_width), *style);
            let img = render_ascii_to_image(
                &ascii,
                &get_embedded_font(),
                width_px,
                height_px,
                *f_size,
            );
            img.save(output).expect("Failed to save image");
            println!("ASCII art saved to: {}", output);
        }
    }
}

fn parse_resolution(res_str: &str) -> Option<(u32, u32)> {
    match res_str.to_lowercase().as_str() {
        "2k" => Some((2560, 1440)),
        "fhd" => Some((1920, 1080)),
        "wxga" => Some((1440, 900)),
        _ => {
            // Try parsing "WIDTHxHEIGHT"
            let parts: Vec<&str> = res_str.split('x').collect();
            if parts.len() == 2 {
                if let (Ok(w), Ok(h)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                    return Some((w, h));
                }
            }
            None
        }
    }
}

// Convert an image to ASCII art
// It returns a string into the stdout containing the ASCII art representation of the image.
// The image is resized to the specified width while maintaining the aspect ratio.
fn image_to_ascii(img: image::GrayImage, style: Option<u8>) -> String {
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
