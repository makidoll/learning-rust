use image::DynamicImage;

pub fn crop(image: &mut DynamicImage, crop: (u32, u32, u32, u32)) {
    (*image) = image.crop(crop.0, crop.1, crop.2, crop.3);
}
