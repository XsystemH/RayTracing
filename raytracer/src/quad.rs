use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{cross, dot, unit_vector, Point3, Vec3};
use std::sync::Arc;

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
}

impl Quad {
    pub fn new(q: &Point3, u: &Vec3, v: &Vec3, mat: Arc<dyn Material>) -> Self {
        let bbox1 = Aabb::two_point(q, &(*q + *u + *v));
        let bbox2 = Aabb::two_point(&(*q + *u), &(*q + *v));

        let n = cross(u, v);
        let normal = unit_vector(&n);
        let d = dot(&normal, q);
        let w = n / dot(&n, &n);
        Self {
            q: *q,
            u: *u,
            v: *v,
            w,
            mat,
            bbox: Aabb::two_aabb(&bbox1, &bbox2),
            normal,
            d,
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let denom = dot(&self.normal, &r.direction());

        if denom.abs() < 1e-8 {
            return None;
        }
        let t = (self.d - dot(&self.normal, &r.origin())) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = r.at(t);
        let p_q = intersection - self.q;
        let alpha = dot(&self.w, &cross(&p_q, &self.v));
        let beta = dot(&self.w, &cross(&self.u, &p_q));

        let range = Interval::new(0.0, 1.0);
        if !range.contains(alpha) || !range.contains(beta) {
            return None;
        }
        let rec = HitRecord::new(
            &intersection,
            t,
            &self.normal,
            r,
            self.mat.clone(),
            alpha,
            beta,
        );
        Some(rec)
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}
