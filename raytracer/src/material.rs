use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};
use crate::vec3::{dot, random_unit_vector, reflect, refract, unit_vector};
use rand::Rng;
use std::sync::Arc;

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Clone)]
pub struct Lambertian {
    tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(&albedo)),
        }
    }
    pub fn new_tex(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(&rec.p, &scatter_direction, r_in.time());
        let attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        Some((scattered, attenuation))
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut reflected = reflect(&r_in.direction(), &rec.normal);
        reflected = unit_vector(&reflected) + random_unit_vector() * self.fuzz;
        let scattered = Ray::new(&rec.p, &reflected, r_in.time());
        let attenuation = self.albedo;
        if dot(&scattered.direction(), &rec.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub(crate) fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
    pub fn reflectance(cos: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 *= r0;
        r0 + (1.0 - r0) * f64::powf(1.0 - cos, 5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::white();
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = unit_vector(&r_in.direction());
        let cos_theta = f64::min(dot(&(-unit_direction), &rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let mut rng = rand::thread_rng();
        let direction =
            if ri * sin_theta > 1.0 || Self::reflectance(cos_theta, ri) > rng.gen_range(0.0..1.0) {
                reflect(&unit_direction, &rec.normal)
            } else {
                refract(&unit_direction, &rec.normal, ri)
            };

        let scattered = Ray::new(&rec.p, &direction, r_in.time());
        Some((scattered, attenuation))
    }
}
