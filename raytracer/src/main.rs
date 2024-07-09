mod aabb;
mod bvh;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod medium;
mod onb;
mod pdf;
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
use crate::medium::ConstantMedium;
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
    let lights = Arc::new(HittableList::new());

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
    camera.render(world, lights);

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
    let lights = Arc::new(HittableList::new());

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
    camera.render(world, lights);

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
    let lights = Arc::new(HittableList::new());

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
    camera.render(world, lights);

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
    let lights = Arc::new(HittableList::new());

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
    camera.render(world, lights);

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

fn cornell_box() {
    let path = std::path::Path::new("output/book3/image14.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let diffuse = Arc::new(DiffuseLight::new(&Color::new(7.0, 7.0, 7.0)));
    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));

    let mut world = HittableList::new();
    let mut lights = HittableList::new();
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
        &Point3::new(213.0, 554.0, 227.0),
        &Vec3::new(130.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 105.0),
        diffuse.clone(),
    )));
    lights.add(Arc::new(Quad::new(
        &Point3::new(213.0, 554.0, 227.0),
        &Vec3::new(130.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 105.0),
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

    // let aluminum = Arc::new(Metal::new(Color::new(0.8, 0.85, 0.88), 0.0));
    let box1 = cuboid(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        white,
    );
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, &Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let glass = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        &Point3::new(190.0, 90.0, 190.0),
        90.0,
        glass,
    )));

    let world = HittableList::new_from(Arc::new(BvhNode::from_list(&mut world)));
    // let lights = HittableList::new_from(Arc::new(BvhNode::from_list(&mut lights)));

    let image_settings = ImageSettings {
        aspect_ratio: 1.0,
        image_width: 600,
        quality: 100,
        samples_per_pixel: 1000,
        max_depth: 50,
        background: Color::black(),
    };

    let camera_settings = CameraSettings {
        vfov: 40.0,
        look_from: Point3::new(278.0, 278.0, -800.0),
        look_at: Point3::new(278.0, 278.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_dist: 10.0,
    };

    let mut camera = Camera::new(image_settings, camera_settings);
    camera.render(world, Arc::new(lights));

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
    } else if thread_rng().gen_range(0.0..1.0) < 0.9999991 {
        cornell_box();
    }
    let path = std::path::Path::new("output/book2/image23.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_sides = 20;
    for i in 0..boxes_per_sides {
        for j in 0..boxes_per_sides {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let y0 = 0.0;
            let z0 = -1000.0 + j as f64 * w;
            let x1 = x0 + w;
            let y1 = thread_rng().gen_range(1.0..101.0);
            let z1 = z0 + w;

            boxes1.add(cuboid(
                &Point3::new(x0, y0, z0),
                &Point3::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }

    let mut world = HittableList::new();
    world.add(Arc::new(BvhNode::from_list(&mut boxes1)));

    let light = Arc::new(DiffuseLight::new(&Color::new(15.0, 15.0, 15.0)));
    world.add(Arc::new(Quad::new(
        &Point3::new(123.0, 554.0, 147.0),
        &Vec3::new(300.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 265.0),
        light,
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(Sphere::moving(
        &center1,
        50.0,
        sphere_material,
        &center2,
    )));

    world.add(Arc::new(Sphere::new(
        &Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::new(
        &Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let mut boundary = Arc::new(Sphere::new(
        &Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::new(
        boundary,
        0.2,
        &Color::new(0.2, 0.4, 0.9),
    )));
    boundary = Arc::new(Sphere::new(
        &Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Arc::new(ConstantMedium::new(
        boundary,
        0.0001,
        &Color::white(),
    )));

    let e_mat = Arc::new(Lambertian::new_tex(Arc::new(ImageTexture::new(
        "earthmap.jpg",
    ))));
    world.add(Arc::new(Sphere::new(
        &Point3::new(400.0, 200.0, 400.0),
        100.0,
        e_mat,
    )));
    let per_text = Arc::new(NoiseTexture::new(0.2));
    world.add(Arc::new(Sphere::new(
        &Point3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new_tex(per_text)),
    )));

    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _j in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            &Point3::random_in(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BvhNode::from_list(&mut boxes2)),
            15.0,
        )),
        &Vec3::new(-100.0, 270.0, 395.0),
    )));

    let world = HittableList::new_from(Arc::new(BvhNode::from_list(&mut world)));
    let lights = Arc::new(HittableList::new());

    let image_settings = ImageSettings {
        aspect_ratio: 1.0,
        image_width: 400,
        quality: 100,
        samples_per_pixel: 200,
        max_depth: 40,
        background: Color::black(),
    };

    let camera_settings = CameraSettings {
        vfov: 40.0,
        look_from: Point3::new(478.0, 278.0, -600.0),
        look_at: Point3::new(278.0, 278.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_dist: 10.0,
    };

    let mut camera = Camera::new(image_settings, camera_settings);
    camera.render(world, lights);

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
