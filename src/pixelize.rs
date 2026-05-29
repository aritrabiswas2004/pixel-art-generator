use image::{DynamicImage, GenericImageView, RgbImage};
use image::imageops::FilterType;
use std::path::{Path, PathBuf};
use std::fs;

const DEFAULT_FILTER: FilterType = FilterType::Nearest;

type Color = [u8; 3];

const GAMEBOY: [Color; 4] = [
    [15, 56, 15],
    [48, 98, 48],
    [139, 172, 15],
    [155, 188, 15],
];

const NES: [Color; 8] = [
    [0, 0, 0],
    [255, 255, 255],
    [128, 128, 128],
    [255, 0, 0],
    [0, 255, 0],
    [0, 0, 255],
    [255, 255, 0],
    [255, 128, 0],
];

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

fn color_distance(a: [u8; 3], b: [u8; 3]) -> u32 {
    let dr = a[0] as i32 - b[0] as i32;
    let dg = a[1] as i32 - b[1] as i32;
    let db = a[2] as i32 - b[2] as i32;

    (dr * dr + dg * dg + db * db) as u32
}

fn nearest_palette_color(pixel: [u8; 3], palette: &[[u8; 3]]) -> Color {
    let mut best = palette[0];
    let mut best_distance = color_distance(pixel, best);

    for &color in palette.iter().skip(1) {
        let dist = color_distance(pixel, color);

        if dist < best_distance {
            best_distance = dist;
            best = color;
        }
    }

    best
}

fn apply_palette(img: &mut RgbImage, palette: &[[u8; 3]]) {
    for pixel in img.pixels_mut() {
        let [r, g, b] = pixel.0;

        pixel.0 = nearest_palette_color(
            [r, g, b],
            palette,
        );
    }
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
    verbose: bool,
    palette: &str,
    levels: u8,
) -> Result<(), Box<dyn std::error::Error>>{
    let img_path = Path::new(image_path);
    let img = image::open(img_path)?;
    let (img_width, img_height) = img.dimensions();
    let name_root = extract_name_root(img_path);

    println!("Input Image: {image_path} ({img_width}x{img_height})");

    let dir = PathBuf::from(name_root);
    let output_path = |property: &str| dir.join(format!("{name_root}_{property}_{downscale_height}.png"));

    fs::create_dir_all(&dir)?;

    let new_img = initial_downscale(&img, downscale_height);
    let (new_width, new_height) = new_img.dimensions();
    if verbose {
        println!("[LOG] Downscaled image dimensions {new_width} x {new_height}");
    }
    let ds_format_path = output_path("downscaled");
    new_img.save(&ds_format_path)?;

    let mut small_image = new_img.to_rgb8();

    match palette.to_ascii_lowercase().as_str() {
        "nes" => apply_palette(&mut small_image, &NES),
        "gameboy" => apply_palette(&mut small_image, &GAMEBOY),
        "none" => quantize_image(&mut small_image,levels),
        val => panic!("Color palette name '{val}' does not exist")
    }

    let color_format_path = output_path("color_reduced");
    small_image.save(&color_format_path)?;

    let final_upscale = upscale(&DynamicImage::ImageRgb8(small_image), img_width, img_height);
    let (final_width, final_height) = new_img.dimensions();
    let upscale_format_path = output_path("final");
    final_upscale.save(&upscale_format_path)?;

    println!("======== Processing Complete ===========");
    if verbose {
        println!("[LOG] Final image upscale resolution --- {} x {}", final_width, final_height);
    }
    println!("Downscaled image saved at {:?}", ds_format_path);
    println!("Final output (upscaled) saved at {:?}", upscale_format_path);

    Ok(())
}
