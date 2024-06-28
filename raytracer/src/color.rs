use crate::interval::Interval;
use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn write_color(&self) -> image::Rgb<u8> {
        let intensity: Interval = Interval::new(0.000, 0.999);

        let r = (intensity.clamp(self.x) * 256.0) as u8;
        let g = (intensity.clamp(self.y) * 256.0) as u8;
        let b = (intensity.clamp(self.z) * 256.0) as u8;
        image::Rgb([r, g, b])
    }
    pub fn white() -> Color {
        Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }
}
