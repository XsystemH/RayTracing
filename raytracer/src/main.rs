mod aabb;
mod bvh;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod perlin;
mod quad;
mod ray;
mod rtw_stb_image;
mod sphere;
mod texture;
mod translate;
mod vec3;

use crate::bvh::BvhNode;
use crate::camera::{Camera, CameraSettings, ImageSettings};
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use crate::quad::{cuboid, Quad};
use crate::sphere::Sphere;
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use crate::translate::{RotateY, Translate};
use crate::vec3::{Point3, Vec3};
use console::style;
use rand::{thread_rng, Rng};
use std::sync::Arc;
use std::{fs::File, process::exit};

fn bouncing_spheres() {
    let path = std::path::Path::new("output/book2/image2.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    // Materials
    let checker = Arc::new(CheckerTexture::new_color(
        0.32,
        &Color::new(0.2, 0.3, 0.1),
        &Color::new(0.9, 0.9, 0.9),
    ));
    let material_ground = Arc::new(Lambertian::new_tex(checker));
    let material1 = Arc::new(Dielectric::new(1.5));
    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    // World
    let mut world: HittableList = HittableList::new();
    world.add(Arc::new(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));
    world.add(Arc::new(Sphere::new(
        &Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    world.add(Arc::new(Sphere::new(
        &Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    world.add(Arc::new(Sphere::new(
        &Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut rng = thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0..1.0);
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * rng.gen_range(0.0..1.0),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material> = if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    Arc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_in(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    Arc::new(Dielectric::new(1.5))
                };
                if choose_mat < 0.8 {
                    let center2 = center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    world.add(Arc::new(Sphere::moving(
                        &center,
                        0.2,
                        sphere_material,
                        &center2,
                    )));
                } else {
                    world.add(Arc::new(Sphere::new(&center, 0.2, sphere_material)));
                }
            }
        }
    }

    let world = HittableList::new_from(Arc::new(BvhNode::from_list(&mut world)));

    let image_settings = ImageSettings {
        aspect_ratio: 16.0 / 9.0,
        image_width: 3840,
        quality: 100,
        samples_per_pixel: 100,
        max_depth: 50,
        background: Color::new(0.7, 0.8, 1.0),
    };

    let camera_settings = CameraSettings {
        vfov: 20.0,
        look_from: Point3::new(13.0, 2.0, 3.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.6,
        focus_dist: 10.0,
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

fn earth() {
    if thread_rng().gen_range(0.0..1.0) < 0.0000001 {
        bouncing_spheres();
    }
    let path = std::path::Path::new("output/book2/image5.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let earth_texture = Arc::new(ImageTexture::new("zbh.jpg"));
    let earth_surface = Arc::new(Lambertian::new_tex(earth_texture));
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(
        &Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface,
    )));
    let world = HittableList::new_from(Arc::new(BvhNode::from_list(&mut world)));

    let image_settings = ImageSettings {
        aspect_ratio: 16.0 / 9.0,
        image_width: 1920,
        quality: 100,
        samples_per_pixel: 100,
        max_depth: 50,
        background: Color::new(0.7, 0.8, 1.0),
    };

    let camera_settings = CameraSettings {
        vfov: 20.0,
        look_from: Point3::new(0.0, 0.0, 12.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_dist: 10.0,
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

fn perlin() {
    let path = std::path::Path::new("output/book2/image15.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let pertext = Arc::new(NoiseTexture::new(4.0));
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_tex(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        &Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new_tex(pertext)),
    )));
    let world = HittableList::new_from(Arc::new(BvhNode::from_list(&mut world)));

    let image_settings = ImageSettings {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        quality: 100,
        samples_per_pixel: 100,
        max_depth: 50,
        background: Color::new(0.7, 0.8, 1.0),
    };

    let camera_settings = CameraSettings {
        vfov: 20.0,
        look_from: Point3::new(13.0, 2.0, 3.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_dist: 10.0,
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

fn quads() {
    let path = std::path::Path::new("output/book2/image16.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let pertext = Arc::new(NoiseTexture::new(4.0));
    let left_red = Arc::new(Lambertian::new(Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new_tex(pertext));
    let right_blue = Arc::new(Lambertian::new(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::new(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::new(Color::new(0.2, 0.8, 0.8)));

    let mut world = HittableList::new();
    world.add(Arc::new(Quad::new(
        &Point3::new(-3.0, -2.0, 5.0),
        &Vec3::new(0.0, 0.0, -4.0),
        &Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(-2.0, -2.0, 0.0),
        &Vec3::new(4.0, 0.0, 0.0),
        &Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(3.0, -2.0, 1.0),
        &Vec3::new(0.0, 0.0, 4.0),
        &Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(-2.0, 3.0, 1.0),
        &Vec3::new(4.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(-2.0, -3.0, 5.0),
        &Vec3::new(4.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));
    let world = HittableList::new_from(Arc::new(BvhNode::from_list(&mut world)));

    let image_settings = ImageSettings {
        aspect_ratio: 1.0,
        image_width: 400,
        quality: 100,
        samples_per_pixel: 100,
        max_depth: 50,
        background: Color::new(0.7, 0.8, 1.0),
    };

    let camera_settings = CameraSettings {
        vfov: 80.0,
        look_from: Point3::new(0.0, 0.0, 9.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_dist: 10.0,
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

fn main() {
    if thread_rng().gen_range(0.0..1.0) < 0.0000001 {
        bouncing_spheres();
    } else if thread_rng().gen_range(0.0..1.0) < 0.0000001 {
        earth();
    } else if thread_rng().gen_range(0.0..1.0) < 0.0000001 {
        perlin();
    } else if thread_rng().gen_range(0.0..1.0) < 0.0000001 {
        quads();
    }
    let path = std::path::Path::new("output/book2/image21.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let diffuse = Arc::new(DiffuseLight::new(&Color::new(15.0, 15.0, 15.0)));
    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));

    let mut world = HittableList::new();
    world.add(Arc::new(Quad::new(
        &Point3::new(555.0, 0.0, 0.0),
        &Vec3::new(0.0, 555.0, 0.0),
        &Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(0.0, 0.0, 0.0),
        &Vec3::new(2.0, 555.0, 0.0),
        &Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(343.0, 554.0, 332.0),
        &Vec3::new(-130.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, -105.0),
        diffuse,
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(0.0, 0.0, 0.0),
        &Vec3::new(555.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(555.0, 555.0, 555.0),
        &Vec3::new(-555.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(0.0, 0.0, 555.0),
        &Vec3::new(555.0, 0.0, 0.0),
        &Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let box1 = cuboid(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, &Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let box2 = cuboid(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 165.0, 165.0),
        white,
    );
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, &Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);

    let world = HittableList::new_from(Arc::new(BvhNode::from_list(&mut world)));

    let image_settings = ImageSettings {
        aspect_ratio: 1.0,
        image_width: 600,
        quality: 100,
        samples_per_pixel: 200,
        max_depth: 50,
        background: Color::black(),
    };

    let camera_settings = CameraSettings {
        vfov: 40.0,
        look_from: Point3::new(278.0, 278.0, -900.0),
        look_at: Point3::new(278.0, 278.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_dist: 10.0,
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
