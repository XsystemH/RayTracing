mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;

use crate::camera::{Camera, CameraSettings, ImageSettings};
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};
use console::style;
use std::sync::Arc;
use std::{fs::File, process::exit};

fn main() {
    let path = std::path::Path::new("output/book1/image21.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    // Materials
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.50));
    let material_bubble = Arc::new(Dielectric::new(1.00 / 1.50));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    // World
    let mut world: HittableList = HittableList::new();
    world.add(Arc::new(Sphere::new(
        &Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Arc::new(Sphere::new(
        &Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Arc::new(Sphere::new(
        &Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
        &Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));
    world.add(Arc::new(Sphere::new(
        &Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let image_settings = ImageSettings {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        quality: 100,
        samples_per_pixel: 50,
        max_depth: 10,
    };

    let camera_settings = CameraSettings {
        vfov: 20.0,
        look_from: Point3::new(-2.0, 2.0, 1.0),
        look_at: Point3::new(0.0, 0.0, -1.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
    };

    let mut camera = Camera::new(image_settings, camera_settings);
    camera.render(world);

    println!(
        "Output image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    let output_image = image::DynamicImage::ImageRgb8(camera.img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(
        &mut output_file,
        image::ImageOutputFormat::Jpeg(camera.quality),
    ) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}
