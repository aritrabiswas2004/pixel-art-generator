mod pixelize;

use pixelize::pixelize;
use clap::Parser;

/// Simple conversion tool to convert regular JPG images to pixel art version of it
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the JPG image
    #[arg(short, long)]
    imagepath: String,

    /// Downscaling height value in pixels
    #[arg(short, long, default_value_t = 128)]
    downscale: u8,

    /// Includes verbose output of image processing
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main(){
    let args = Args::parse();

    pixelize(args.imagepath.as_str(), args.downscale as u32, args.verbose).expect("Something went horribly wrong");
}
