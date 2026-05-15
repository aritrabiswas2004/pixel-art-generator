use image::{DynamicImage, GenericImageView};
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

fn pixelize(image_path: &str, downscale_height: u32) -> Result<(), Box<dyn std::error::Error>>{
    let img_path = Path::new(image_path);
    let img = image::open(img_path)?;
    let (img_width, img_height) = img.dimensions();
    println!("Original image dimensions {} x {}", img_width, img_height);

    let dir = PathBuf::from(
        img_path
            .file_stem()
            .and_then(|f| {f.to_str()})
            .unwrap_or("output")
    );

    fs::create_dir_all(&dir)?;

    let new_img = initial_downscale(&img, downscale_height);
    let (new_width, new_height) = new_img.dimensions();
    println!("Downscaled image dimensions {} x {}", new_width, new_height);
    new_img.save(dir.join(format!("downscaled_{downscale_height}.jpg")))?;

    let final_upscale = upscale(&new_img, img_width, img_height);
    final_upscale.save(dir.join(format!("final_{downscale_height}.jpg")))?;

    println!("======== DONE ===========");
    println!("Final image upscale resolution --- {} x {}", img_width, img_height);

    Ok(())
}

fn main(){
    pixelize("bluemarble.jpg", 128).expect("Something went horribly wrong");
}
