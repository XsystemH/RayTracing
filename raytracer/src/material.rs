use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{dot, random_unit_vector, reflect, unit_vector};

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal.clone() + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }

        let scattered = Ray::new(&rec.p, &scatter_direction);
        let attenuation = self.albedo.clone();
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
        let scattered = Ray::new(&rec.p, &reflected);
        let attenuation = self.albedo.clone();
        if dot(&scattered.direction(), &rec.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
