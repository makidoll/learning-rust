use image::DynamicImage;

pub fn grayscale(image: &mut DynamicImage) {
    (*image) = image.grayscale();
}
