use image::{
    GenericImageView
};
use image::imageops::{FilterType};

fn main() {
    let img = image::open("./earthrise.jpg").expect("Failed to load initial image");
    let (img_width, img_height) = img.dimensions();
    println!("Original image dimensions {} x {}", img_width, img_height);

    let new_img = img.resize(186, 128, FilterType::Nearest);
    let (new_width, new_height) = new_img.dimensions();
    println!("Downscaled image dimensions {} x {}", new_width, new_height);
    new_img.save("step1.jpg").expect("Failed to save new image");

    let final_upscale = new_img.resize(3840, 2649, FilterType::Nearest);
    final_upscale.save("final3.jpg").expect("Failed to save final image");

    println!("======== DONE ===========");
    println!("Final image upscale resolution --- {} x {}", img_width, img_height);
}
