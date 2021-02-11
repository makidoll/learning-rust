use image::DynamicImage;

pub fn blur(image: &mut DynamicImage, amount: f32) {
    (*image) = image.blur(amount);
}
