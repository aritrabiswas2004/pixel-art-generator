use image::{DynamicImage, GenericImageView, RgbImage};
use image::imageops::{FilterType};
use std::path::{Path, PathBuf};
use std::fs;

const DEFAULT_FILTER: FilterType = FilterType::Nearest;

// NOTE: Downscale height can theoretically be any u32 but keep it 64, 128, 256
fn initial_downscale(img: &DynamicImage, downscale_height: u32) -> DynamicImage {
    let (original_width, original_height) = img.dimensions();
    let downscale_width = (original_width * downscale_height) / original_height;

    img.resize(downscale_width, downscale_height, DEFAULT_FILTER)
}

// Ideally make new_width and new_height as the original JPG dimensions
// Debate over using resize vs resize_exact
fn upscale(downscaled_img: &DynamicImage, new_width: u32, new_height: u32) -> DynamicImage{
    downscaled_img.resize(new_width, new_height, DEFAULT_FILTER)
}

fn extract_name_root(img_path: &Path) -> &str {
    img_path
        .file_stem()
        .and_then(|f| {f.to_str()})
        .unwrap_or("output")
}

fn quantize_channel(v: u8, levels: u8) -> u8{
    let step = 255 / (levels - 1);
    ((v as f32 / step as f32).round() * step as f32) as u8
}

fn quantize_image(img: &mut RgbImage, levels: u8) {
    for pixel in img.pixels_mut() {
        let [r, g, b] = pixel.0;

        pixel.0 = [
            quantize_channel(r, levels),
            quantize_channel(g, levels),
            quantize_channel(b, levels),
        ];
    }
}

pub fn pixelize(
    image_path: &str,
    downscale_height: u32,
    verbose: bool
) -> Result<(), Box<dyn std::error::Error>>{
    let img_path = Path::new(image_path);
    let img = image::open(img_path)?;
    let (img_width, img_height) = img.dimensions();

    println!("Input Image: {image_path} ({img_width}x{img_height})");

    let dir = PathBuf::from(extract_name_root(img_path));

    fs::create_dir_all(&dir)?;

    let new_img = initial_downscale(&img, downscale_height);
    let (new_width, new_height) = new_img.dimensions();
    if verbose {
        println!("[LOG] Downscaled image dimensions {new_width} x {new_height}");
    }
    let ds_format_path = dir.join(format!("{}_downscaled_{downscale_height}.png", extract_name_root(img_path)));
    new_img.save(&ds_format_path)?;

    let mut small_image = new_img.to_rgb8();
    quantize_image(&mut small_image, 8);
    let color_format_path = dir.join(format!("{}_color_reduced_{downscale_height}.png", extract_name_root(img_path)));
    small_image.save(&color_format_path)?;

    let final_upscale = upscale(&DynamicImage::ImageRgb8(small_image), img_width, img_height);
    let (final_width, final_height) = new_img.dimensions();
    let upscale_format_path = dir.join(format!("{}_final_{downscale_height}.png", extract_name_root(img_path)));
    final_upscale.save(&upscale_format_path)?;

    println!("======== Processing Complete ===========");
    if verbose {
        println!("[LOG] Final image upscale resolution --- {} x {}", final_width, final_height);
    }
    println!("Downscaled image saved at {:?}", ds_format_path);
    println!("Final output (upscaled) saved at {:?}", upscale_format_path);

    Ok(())
}
