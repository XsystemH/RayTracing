use crate::color::Color;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{random_unit_vector, unit_vector, Point3, Vec3};
use image::RgbImage; // ImageBuffer
use indicatif::ProgressBar;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone)]
pub struct Camera {
    // image
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub image_height: u32,
    pub quality: u8,
    pub samples_per_pixel: u32,
    pub pixel_samples_scale: f64,
    pub max_depth: i32,
    pub img: RgbImage,
    // Camera & Viewport
    pub focal_length: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub camera_center: Point3,
    pub viewport_u: Vec3,
    pub viewport_v: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub viewport_upper_left: Point3,
    pub pixel100_loc: Point3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        quality: u8,
        samples_per_pixel: u32,
        max_depth: i32,
        focal_length: f64,
        viewport_height: f64,
    ) -> Self {
        let mut image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
        if image_height == 0 {
            image_height = 1;
        }
        let pixel_samples_scale: f64 = 1.0 / samples_per_pixel as f64;
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
            - viewport_u.clone() / 2.0
            - viewport_v.clone() / 2.0;
        let pixel100_loc: Point3 =
            viewport_upper_left.clone() + (pixel_delta_u.clone() + pixel_delta_v.clone()) * 0.5;
        Self {
            aspect_ratio,
            image_width,
            image_height,
            quality,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
            img: RgbImage::new(image_width, image_height),
            focal_length,
            viewport_height,
            viewport_width,
            camera_center,
            viewport_u,
            viewport_v,
            pixel_delta_u,
            pixel_delta_v,
            viewport_upper_left,
            pixel100_loc,
        }
    }

    pub fn render(&mut self, world: HittableList) {
        let progress = if option_env!("CI").unwrap_or_default() == "true" {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((self.image_height * self.image_width) as u64)
        };

        let img = Arc::new(Mutex::new(self.img.clone()));
        let progress = Arc::new(Mutex::new(progress));

        let mut rend_lines = vec![];

        for j in (0..self.image_height).rev() {
            let img = Arc::clone(&img);
            let progress = Arc::clone(&progress);
            let world = world.clone();
            let image_width = self.image_width;
            let copy = Sensor::new(self);
            let rend_line = thread::spawn(move || {
                for i in 0..image_width {
                    let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);

                    for _sample in 0..copy.samples_per_pixel {
                        let r = copy.get_ray(i, j);
                        pixel_color += ray_color(r, copy.max_depth, &world);
                    }
                    pixel_color *= copy.pixel_samples_scale;

                    let mut img = img.lock().unwrap();
                    let pixel = img.get_pixel_mut(i, j);
                    *pixel = pixel_color.write_color();
                    drop(img);

                    let progress = progress.lock().unwrap();
                    progress.inc(1);
                }
            });
            rend_lines.push(rend_line);
        }

        for rend_line in rend_lines {
            rend_line.join().unwrap();
        }

        progress.lock().unwrap().finish();

        let img = Some(Arc::try_unwrap(img).unwrap().into_inner().unwrap());
        self.img = img.as_ref().unwrap().clone();
    }
}

struct Sensor {
    pub samples_per_pixel: u32,
    pub pixel_samples_scale: f64,
    pub max_depth: i32,
    pub pixel100_loc: Point3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub camera_center: Point3,
}

impl Sensor {
    pub fn new(camera: &Camera) -> Self {
        Self {
            samples_per_pixel: camera.samples_per_pixel,
            pixel_samples_scale: camera.pixel_samples_scale,
            max_depth: camera.max_depth,
            pixel100_loc: camera.pixel100_loc.clone(),
            pixel_delta_u: camera.pixel_delta_u.clone(),
            pixel_delta_v: camera.pixel_delta_v.clone(),
            camera_center: camera.camera_center.clone(),
        }
    }
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel100_loc.clone()
            + (self.pixel_delta_u.clone() * (i as f64 + offset.x))
            + (self.pixel_delta_v.clone() * (j as f64 + offset.y));
        let ray_direction = pixel_sample - self.camera_center.clone();
        Ray::new(&self.camera_center, &ray_direction)
    }
}

fn ray_color(r: Ray, depth: i32, world: &dyn Hittable) -> Color {
    if depth < 0 {
        return Color::black();
    }

    if let Some(hit_record) = world.hit(&r, Interval::new(0.001, f64::INFINITY)) {
        let direction = hit_record.normal + random_unit_vector(); // random_on_hemisphere(&hit_record.normal);
        return ray_color(Ray::new(&hit_record.p, &direction), depth - 1, world) * 0.5;
    }

    let unit_direction = unit_vector(&r.direction());
    let a = 0.5 * (unit_direction._y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn sample_square() -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5), 0.0)
}
