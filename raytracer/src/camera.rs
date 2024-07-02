use crate::color::Color;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{cross, random_in_unit_disk, unit_vector, Point3, Vec3};
use image::RgbImage; // ImageBuffer
use indicatif::ProgressBar;
use rand::{thread_rng, Rng};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ImageSettings {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub quality: u8,
    pub samples_per_pixel: u32,
    pub max_depth: i32,
}

pub struct CameraSettings {
    pub vfov: f64,
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
}

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
    // Camera
    pub camera_center: Point3,
    pub vfov: f64,
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Vec3,
    pub theta: f64,
    pub h: f64,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    // Viewport
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub viewport_u: Vec3,
    pub viewport_v: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub viewport_upper_left: Point3,
    pub pixel100_loc: Point3,
    pub defocus_disk_u: Vec3,
    pub defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(image_settings: ImageSettings, camera_settings: CameraSettings) -> Self {
        let ImageSettings {
            aspect_ratio,
            image_width,
            quality,
            samples_per_pixel,
            max_depth,
        } = image_settings;

        let CameraSettings {
            vfov,
            look_from,
            look_at,
            vup,
            defocus_angle,
            focus_dist,
        } = camera_settings;
        let mut image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
        if image_height == 0 {
            image_height = 1;
        }
        let camera_center: Point3 = look_from;
        let theta: f64 = vfov * std::f64::consts::PI / 180.0;
        let h: f64 = f64::tan(theta / 2.0);
        let pixel_samples_scale: f64 = 1.0 / samples_per_pixel as f64;
        let viewport_height: f64 = 2.0 * h * focus_dist;
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
        // edge vector
        let w = unit_vector(&(look_from - look_at));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);
        let viewport_u: Vec3 = u * viewport_width;
        let viewport_v: Vec3 = v * -viewport_height;
        // delta vector
        let pixel_delta_u: Vec3 = viewport_u / image_width as f64;
        let pixel_delta_v: Vec3 = viewport_v / image_height as f64;
        // upper left
        let viewport_upper_left: Point3 =
            camera_center - w * focus_dist - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel100_loc: Point3 = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;
        // disk vector
        let defocus_radius =
            focus_dist * f64::tan(defocus_angle / 2.0 * std::f64::consts::PI / 180.0);
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;
        Self {
            aspect_ratio,
            image_width,
            image_height,
            quality,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
            img: RgbImage::new(image_width, image_height),
            camera_center,
            look_from,
            look_at,
            vup,
            vfov,
            theta,
            h,
            defocus_angle,
            focus_dist,
            viewport_height,
            viewport_width,
            viewport_u,
            viewport_v,
            pixel_delta_u,
            pixel_delta_v,
            viewport_upper_left,
            pixel100_loc,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&mut self, world: HittableList) {
        let progress = if option_env!("CI").unwrap_or_default() == "true" {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((self.image_height * self.image_width) as u64)
        };

        let lines: Vec<Option<Vec<Color>>> = vec![None; self.image_height as usize];
        let lines = Arc::new(Mutex::new(lines));
        // let img = Arc::new(Mutex::new(self.img.clone()));
        let progress = Arc::new(Mutex::new(progress));

        let mut rend_lines = vec![];

        for j in (0..self.image_height).rev() {
            let lines_clone = Arc::clone(&lines);
            // let img = Arc::clone(&img);
            let progress = Arc::clone(&progress);
            let world = world.clone();
            let image_width = self.image_width;
            let copy = Sensor::new(self);
            let rend_line = thread::spawn(move || {
                let mut line: Vec<Color> = Vec::with_capacity(image_width as usize);
                for i in 0..image_width {
                    let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);

                    for _sample in 0..copy.samples_per_pixel {
                        let r = copy.get_ray(i, j);
                        pixel_color += ray_color(r, copy.max_depth, &world);
                    }
                    pixel_color *= copy.pixel_samples_scale;

                    // let mut img = img.lock().unwrap();
                    // let pixel = img.get_pixel_mut(i, j);
                    // *pixel = pixel_color.write_color();
                    // drop(img);
                    line.push(pixel_color);

                    let progress = progress.lock().unwrap();
                    progress.inc(1);
                }
                let mut lines = lines_clone.lock().unwrap();
                lines[j as usize] = Some(line);
            });
            rend_lines.push(rend_line);
        }

        for rend_line in rend_lines {
            rend_line.join().unwrap();
        }

        progress.lock().unwrap().finish();

        let lines = Arc::try_unwrap(lines).expect("!").into_inner().unwrap();
        // let img = Some(Arc::try_unwrap(img).unwrap().into_inner().unwrap());
        // self.img = img.as_ref().unwrap().clone();
        for (j, line_option) in lines.into_iter().enumerate() {
            if let Some(line) = line_option {
                for (i, color) in line.into_iter().enumerate() {
                    let pixel = self.img.get_pixel_mut(i as u32, j as u32);
                    *pixel = color.write_color();
                }
            }
        }
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
    pub defocus_angle: f64,
    pub defocus_disk_u: Vec3,
    pub defocus_disk_v: Vec3,
}

impl Sensor {
    pub fn new(camera: &Camera) -> Self {
        Self {
            samples_per_pixel: camera.samples_per_pixel,
            pixel_samples_scale: camera.pixel_samples_scale,
            max_depth: camera.max_depth,
            pixel100_loc: camera.pixel100_loc,
            pixel_delta_u: camera.pixel_delta_u,
            pixel_delta_v: camera.pixel_delta_v,
            camera_center: camera.camera_center,
            defocus_angle: camera.defocus_angle,
            defocus_disk_u: camera.defocus_disk_u,
            defocus_disk_v: camera.defocus_disk_v,
        }
    }
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel100_loc
            + (self.pixel_delta_u * (i as f64 + offset.x))
            + (self.pixel_delta_v * (j as f64 + offset.y));
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.camera_center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = thread_rng().gen_range(0.0..1.0);

        Ray::new(&ray_origin, &ray_direction, ray_time)
    }
    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.camera_center + (self.defocus_disk_u * p.x) + (self.defocus_disk_v * p.y)
    }
}

fn ray_color(r: Ray, depth: i32, world: &dyn Hittable) -> Color {
    if depth < 0 {
        return Color::black();
    }

    if let Some(hit_record) = world.hit(&r, Interval::new(0.001, f64::INFINITY)) {
        // let direction = hit_record.normal + random_unit_vector();
        // return ray_color(Ray::new(&hit_record.p, &direction), depth - 1, world) * 0.5;
        if let Some((scattered, attenuation)) = hit_record.mat.scatter(&r, &hit_record) {
            return attenuation * ray_color(scattered, depth - 1, world);
        }
        return Color::black();
    }

    let unit_direction = unit_vector(&r.direction());
    let a = 0.5 * (unit_direction._y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn sample_square() -> Vec3 {
    let mut rng = thread_rng();
    Vec3::new(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5), 0.0)
}
