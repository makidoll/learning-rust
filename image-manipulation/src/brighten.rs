use image::DynamicImage;

pub fn brighten(image: &mut DynamicImage, amount: i32) {
    (*image) = image.brighten(amount);
}
