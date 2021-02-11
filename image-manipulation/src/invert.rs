use image::DynamicImage;

pub fn invert(image: &mut DynamicImage) {
    image.invert();
}
