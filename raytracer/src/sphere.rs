use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};
use std::sync::Arc;
use rand::{Rng, thread_rng};
use crate::onb::Onb;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material>,
    is_moving: bool,
    center_vec: Vec3,
    bbox: Aabb,
}

impl Sphere {
    pub(crate) fn new(center: &Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        let r_vec = Vec3::new(radius, radius, radius);
        let bbox = Aabb::two_point(&(*center - r_vec), &(*center + r_vec));
        Self {
            center: *center,
            radius,
            mat,
            is_moving: false,
            center_vec: Vec3::new(0.0, 0.0, 0.0),
            bbox,
        }
    }
    pub(crate) fn moving(
        center: &Point3,
        radius: f64,
        mat: Arc<dyn Material>,
        center2: &Vec3,
    ) -> Self {
        let r_vec = Vec3::new(radius, radius, radius);
        let box1 = Aabb::two_point(&(*center - r_vec), &(*center + r_vec));
        let box2 = Aabb::two_point(&(*center2 - r_vec), &(*center2 + r_vec));
        let bbox = Aabb::two_aabb(&box1, &box2);
        Self {
            center: *center,
            radius,
            mat,
            is_moving: true,
            center_vec: *center2 - *center,
            bbox,
        }
    }
    pub(crate) fn sphere_center(&self, time: f64) -> Point3 {
        self.center + self.center_vec * time
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let center: Vec3 = if self.is_moving {
            self.sphere_center(r.time())
        } else {
            self.center
        };
        let oc: Vec3 = center - r.origin();
        let a: f64 = r.direction().length_squared();
        let h: f64 = dot(&r.direction(), &oc);
        let c: f64 = oc.length_squared() - self.radius * self.radius;

        let discriminant: f64 = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd: f64 = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let mut root: f64 = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t: f64 = root;
        let p: Point3 = r.at(t);
        let outward_normal: Vec3 = (p - self.center) / self.radius;

        let theta = f64::acos(-outward_normal.y);
        let phi = f64::atan2(-outward_normal.z, outward_normal.x) + std::f64::consts::PI;
        let u = phi / (2.0 * std::f64::consts::PI);
        let v = theta / std::f64::consts::PI;
        let rec: HitRecord = HitRecord::new(&p, t, &outward_normal, r, self.mat.clone(), u, v);
        Some(rec)
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }

    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        if let Some(_rec) = self.hit(
            &Ray::new(origin, direction, 0.0),
            Interval::new(0.001, f64::INFINITY),
        ) {
            let cos_theta_max = (1.0 - self.radius * self.radius / (self.center - *origin).length_squared()).sqrt();
            let solid_angle = 2.0 * std::f64::consts::PI * (1.0 - cos_theta_max);
            1.0 / solid_angle
        } else {
            0.0
        }
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let dir = self.center - *origin;
        let dis_squared = dir.length_squared();
        let uvw = Onb::new(&dir);
        uvw.local(&random_to_sphere(self.radius, dis_squared))
    }
}

fn random_to_sphere(radius: f64, dis_squared: f64) -> Vec3 {
    let r1 = thread_rng().gen_range(0.0..1.0);
    let r2 = thread_rng().gen_range(0.0..1.0);
    let z = 1.0 + r2 * ((1.0 - radius * radius / dis_squared).sqrt() - 1.0);

    let phi = 2.0 * r1 * std::f64::consts::PI;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();

    Vec3 { x, y, z, }
}
