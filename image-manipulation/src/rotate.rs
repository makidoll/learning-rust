use image::DynamicImage;

pub fn rotate(image: &mut DynamicImage, amount: i32) {
    match amount {
        90 => {
            (*image) = image.rotate90();
        }
        180 => {
            (*image) = image.rotate180();
        }
        270 => {
            (*image) = image.rotate270();
        }
        _ => {}
    }
}
