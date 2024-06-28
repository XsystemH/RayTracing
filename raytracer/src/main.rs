mod color;
mod ray;
mod vec3;

use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::{dot, unit_vector, Point3, Vec3};
use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::{fs::File, process::exit};

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc: Vec3 = center.clone() - r.origin();
    let a: f64 = dot(&r.direction(), &r.direction());
    let b: f64 = dot(&r.direction(), &oc) * -2.0;
    let c: f64 = dot(&oc, &oc) - radius * radius;
    let discriminant: f64 = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    }
    (-b - f64::sqrt(discriminant)) / (2.0 * a)
}

fn ray_color(r: Ray) -> Color {
    let t: f64 = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let n: Vec3 = unit_vector(&(r.at(t) - Vec3::new(0.0, 0.0, -1.0)));
        return Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
    }

    let unit_direction = unit_vector(&r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn main() {
    let path = std::path::Path::new("output/book1/image4.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    // if image_height < 1 { let image_height: u32 = 1; }
    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    // Camera & Viewport
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center: Point3 = Point3::new(0.0, 0.0, 0.0);
    // edge vector
    let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);
    // delta vector
    let pixel_delta_u: Vec3 = viewport_u.clone() / image_width as f64;
    let pixel_delta_v: Vec3 = viewport_v.clone() / image_height as f64;
    // upper left
    let viewport_upper_left: Point3 = camera_center.clone()
        - Vec3::new(0.0, 0.0, focal_length)
        - viewport_u / 2.0  // .clone()
        - viewport_v / 2.0; // .clone()
    let pixel100_loc: Point3 =
        viewport_upper_left + (pixel_delta_u.clone() + pixel_delta_v.clone()) * 0.5;
    // viewport_upper_left.clone()
    // Render

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((image_height * image_width) as u64)
    };

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let pixel = img.get_pixel_mut(i, j);

            let pixel_center: Point3 = pixel100_loc.clone()
                + pixel_delta_u.clone() * i as f64
                + pixel_delta_v.clone() * j as f64;
            let ray_direction: Vec3 = pixel_center.clone() - camera_center.clone();
            let r: Ray = Ray::new(&camera_center, &ray_direction);

            let pixel_color: Color = ray_color(r);
            *pixel = pixel_color.write_color();
            // *pixel = image::Rgb([r as u8, g as u8, b as u8]);
        }
        progress.inc(1);
    }
    progress.finish();

    println!(
        "Ouput image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}
