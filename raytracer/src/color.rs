use crate::vec3::Vec3;

type Color = Vec3;
use image::{ImageBuffer, RgbImage};

fn write_color (pixel_color: &Color) -> image::Rgb<u8> {
    image::Rgb([pixel_color.x() as u8, pixel_color.y() as u8, pixel_color.z() as u8])
}