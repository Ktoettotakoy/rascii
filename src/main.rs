use clap::{Parser, Subcommand};
use log::{debug};

use rascii::utils::image_ops::image_filters::resize_image_simple;
use rascii::utils::image_to_ascii::image_to_ascii;
use rascii::utils::image_ops::image_rendering::render_ascii_to_image;
use rascii::utils::embedded_font::get_embedded_font;
use rascii::utils::video_ops::video_rendering::process_video_to_ascii;
use rascii::utils::timer::timer_debug;


/// ASCII Art Generator
#[derive(Parser, Debug)]
#[command(name = "ascii-art")]
#[command(author = "Yaroslav", version, about = "Generate ASCII art from images")]
#[command (disable_help_flag = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable debug output (timing info)
    #[arg(long)]
    debug: bool,
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
        #[arg(short = 'w', long, default_value_t = 100)]
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
    Video {
        /// Path to the input video
        #[arg(short, long)]
        input: String,

        /// Output resolution: either named (2k, fhd) or custom (e.g. 1920x1080)
        #[arg(short, long)]
        res: String,

        /// Output file path
        #[arg(short, long, default_value_t = String::from("ascii_video.mp4"))]
        output: String,

        /// ASCII character width per frame
        #[arg(short = 'w', long, default_value_t = 120)]
        char_width: u32,

        /// Style of ASCII art
        #[arg(short, long)]
        style: Option<u8>,

        /// Font size
        #[arg(short = 'f', long, default_value_t = 9.0)]
        f_size: f32,
    },
}

fn main() {
    let cli = Cli::parse();

    // initialize logging
    if cli.debug {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    match &cli.command {
        Commands::Text { input, width, style} => {
            debug!("Input file: {}", input);
            debug!("Width: {}", width);

            println!("Printing ASCII to stdout");

            let ascii = timer_debug("image_to_ascii", || {
                image_to_ascii(resize_image_simple(input, *width), *style)
            });

            println!("{}",ascii);
        },
        Commands::Image { input, res, char_width, f_size, style, output } => {
            let (width_px, height_px) = parse_resolution(res).unwrap_or_else(|| {
                eprintln!("Invalid resolution: '{}'", res);
                std::process::exit(1);
            });

            debug!("Rendering at resolution: {}x{} px", width_px, height_px);
            debug!("Character width: {}", char_width);

            let ascii = image_to_ascii(resize_image_simple(input, *char_width), *style);

            let img = timer_debug("render_ascii_to_image", || {
                render_ascii_to_image(&ascii, &get_embedded_font(), width_px, height_px, *f_size)
            });

            img.save(output).expect("Failed to save image");
            println!("ASCII art saved to: {}", output);
        },
        Commands::Video { input, res, output, char_width, style, f_size } => {
            let (width_px, height_px) = parse_resolution(res).unwrap_or_else(|| {
                eprintln!("Invalid resolution: '{}'", res);
                std::process::exit(1);
            });
            println!("Converting video: {}", input);
            timer_debug("Video to ascii total", || { process_video_to_ascii(input, output, width_px, height_px, *char_width, *style, *f_size)});
            println!("ASCII video saved to: {}", output);
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
