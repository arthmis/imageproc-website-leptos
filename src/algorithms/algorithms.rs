use image::RgbaImage;
use image_processing::pixel_ops::invert_mut;

const CHANNEL_COUNT: u32 = 4;

pub fn invert(input_image: Vec<u8>, width: u32) -> Vec<u8> {
    let height = (input_image.len() as u32 / CHANNEL_COUNT) / width;
    let mut image: RgbaImage = image::ImageBuffer::from_vec(width, height, input_image)
        .expect("expected image from canvas");
    invert_mut(&mut image);

    image.into_vec()
}
