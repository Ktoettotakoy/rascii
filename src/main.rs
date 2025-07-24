use clap::{Parser, Subcommand};
use log::{debug, info};
use colored::*;


use rascii::utils::image_ops::image_filters::resize_image_simple;
use rascii::utils::image_to_ascii::image_to_ascii;
use rascii::utils::image_ops::image_rendering::render_ascii_to_image;
use rascii::utils::font_utils::{get_embedded_font,get_larry3d_font};
use rascii::utils::video_ops::in_memory_video_rendering::process_video_to_ascii_opencv;
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
    /// Converts an image to ASCII into stdout
    Console {
        /// Path to the input image
        #[arg(short, long)]
        input: String,

        /// Output width
        #[arg(short, long, default_value_t = 100)]
        width: u32,

        /// ASCII art style [default: 0]
        #[arg(short, long)]
        style: Option<u8>,
    },
    /// Converts an image to ASCII and save as an image file
    Image {
        /// Path to the input image
        #[arg(short, long)]
        input: String,

        /// Output resolution
        #[arg(short, long, default_value_t = String::from("fhd"))]
        res: String,

        /// Width in characters (columns) for ASCII rendering
        #[arg(short = 'w', long, default_value_t = 100)]
        char_width: u32,

        /// Font size
        #[arg(short = 'f', long, default_value_t = 9.0)]
        f_size: f32,

        /// ASCII art style [default: 0]
        #[arg(short, long)]
        style: Option<u8>,

        /// Output file
        #[arg(short, long, default_value_t = String::from("res.png"))]
        output: String,
    },
    /// Converts a video to ASCII and save as a video file
    Video {
        /// Path to the input video
        #[arg(short, long)]
        input: String,

        /// Output resolution
        #[arg(short, long, default_value_t = String::from("fhd"))]
        res: String,

        /// ASCII character width per frame
        #[arg(short = 'w', long, default_value_t = 120)]
        char_width: u32,

        /// Font size
        #[arg(short = 'f', long, default_value_t = 9.0)]
        f_size: f32,

        /// ASCII art style [default: 0]
        #[arg(short, long)]
        style: Option<u8>,

        /// Fps of the output video [default: matches source]
        #[arg(long)]
        fps: Option<f64>,

        /// Output file path
        #[arg(short, long, default_value_t = String::from("ascii_res.mp4"))]
        output: String,
    },
}

fn main() {
    print_logo();
    let cli = Cli::parse();

    // initialize logging
    if cli.debug {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();


    match &cli.command {
        Commands::Console { input, width, style} => {
            debug!("Input file: {}", input);
            debug!("Width: {}", width);

            info!("Printing ASCII to stdout");

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



            let ascii = image_to_ascii(resize_image_simple(input, *char_width), *style);

            let img = timer_debug("render_ascii_to_image", || {
                render_ascii_to_image(&ascii, &get_embedded_font(), width_px, height_px, *f_size)
            });

            img.save(output).expect("Failed to save image");
            info!("ASCII art saved to: {}", output);
        },
        Commands::Video { input, res, output, char_width, style, f_size, fps } => {
            let (width_px, height_px) = parse_resolution(res).unwrap_or_else(|| {
                eprintln!("Invalid resolution: '{}'", res);
                std::process::exit(1);
            });
            info!("Converting video: {}", input);
            timer_debug("Video to ascii total", || { process_video_to_ascii_opencv(input, output, width_px, height_px, *char_width, *style, *fps, *f_size)});
            info!("ASCII video saved to: {}", output);
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
                let w = parts[0].parse::<u32>();
                let h = parts[1].parse::<u32>();
                if let (Ok(w), Ok(h)) = (w, h) {
                    if w > 0 && h > 0 && w <= 4096 && h <= 16384 {
                        return Some((w, h));
                    }
                }
            }
            None
        }
    }
}

fn print_logo() {
    let font = get_larry3d_font();
    let figure = font.convert("RASCII").unwrap();

    println!("{}", figure.to_string().truecolor(255, 165, 0).bold());
}
