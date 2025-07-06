use clap::{Parser, Subcommand};
// use image::GenericImageView;
use image::imageops::FilterType;

const ASCII_CHARS: &[u8] = b"@%#*+=-:. ";
const ASCII_CHARS_EXTENDED: &[u8] = b"@%#*+=-:.              ";

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

        /// Output file (optional)
        #[arg(short, long)]
        output: Option<String>,
    },
}

fn main() {
 let cli = Cli::parse();

    match &cli.command {
        Commands::Convert { input, width, output } => {
            println!("Input file: {}", input);
            println!("Width: {}", width);

            if let Some(out) = output {
                println!("Saving output to: {}", out);
                // println!({},image_to_ascii(resize_image(input, *width)));
                // Call your image-to-ASCII function here and write to file
            } else {
                println!("Printing ASCII to stdout");
                // Call your image-to-ASCII function and print to terminal
                println!("{}",image_to_ascii(resize_image(input, *width)));
            }
        }
    }
}

fn resize_image(path: &str, width: u32) -> image::GrayImage {
    let img = image::open(path).expect("Failed to open image").to_luma8();
    let (orig_width, orig_height) = img.dimensions();
    let aspect_ratio = orig_height as f32 / orig_width as f32;
    let height = (width as f32 * aspect_ratio * 0.55) as u32; // Adjust for terminal font ratio
    image::imageops::resize(&img, width, height, FilterType::Nearest)
}



fn pixel_to_ascii(pixel: u8) -> char {
    let index = (pixel as f32 / 255.0 * (ASCII_CHARS_EXTENDED.len() - 1) as f32) as usize;
    ASCII_CHARS_EXTENDED[index] as char
}

fn image_to_ascii(img: image::GrayImage) -> String {
    let mut ascii = String::new();
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y)[0];
            ascii.push(pixel_to_ascii(pixel));
        }
        ascii.push('\n');
    }
    ascii
}
