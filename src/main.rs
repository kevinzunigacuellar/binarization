// use image::GenericImageView;
use image::GrayImage;
use imageproc::contrast::{self, threshold};

// struct input {
//     let mut image: GrayImage;
//     let mut threshold: u8;
// }

fn main() {
    let mut img = image::open("samples/image-1g.png")
        .ok()
        .expect("Opening image failed");
    let img = img.crop(0, 0, img.width(), img.height());

    let luma8_img: GrayImage = img.clone().to_luma8();
    let otsu_threshold = contrast::otsu_level(&luma8_img);

    let binarized_image = threshold(&luma8_img, otsu_threshold);
    binarized_image.save("output/sample-1g.png").unwrap();
}
