mod pixelize;

use pixelize::pixelize;
use clap::Parser;

/// Simple conversion tool to convert regular JPG/PNG images to pixel art version of it
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the image file
    #[arg(value_name = "IMAGE")]
    imagepath: String,

    /// Downscaling height value in pixels
    #[arg(short, long, default_value_t = 128)]
    downscale: u32,

    /// Includes verbose output of image processing
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Applies color palette to image, one of NES, GAMEBOY, PICO8, C64
    #[arg(short, long, default_value = "NONE")]
    palette: String,

    /// Image quantization levels (common RGB channel grouping)
    #[arg(short, long, default_value_t = 8, value_parser = clap::value_parser!(u8).range(2..=255))]
    levels: u8,
}

fn main(){
    let args = Args::parse();

    pixelize(
        args.imagepath.as_str(), 
        args.downscale, 
        args.verbose,
        args.palette.as_str(),
        args.levels,
    ).expect("Something went horribly wrong");
}
