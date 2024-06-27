use crate::vec3::Vec3;

pub(crate) type Color = Vec3;
use image::{ImageBuffer, RgbImage};

impl Color {
    pub fn write_color(&self) -> image::Rgb<u8> {
        let r = (self.x() * 255.99) as u8;
        let g = (self.y() * 255.99) as u8;
        let b = (self.z() * 255.99) as u8;
        image::Rgb([r, g, b])
    }
}

pub fn write_color (pixel_color: &Color) -> image::Rgb<u8> {
    image::Rgb([pixel_color.x() as u8, pixel_color.y() as u8, pixel_color.z() as u8])
}