use crate::vec3::Vec3;
use crate::vec3::Point3;
use std::f64;

struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3) -> Self {
        Self {
            orig: origin.clone(),
            dir: direction.clone(),
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig.clone()
    }
    pub fn direction(&self) -> Vec3 {
        self.dir.clone()
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig.clone() + self.dir.clone() * t
    }
}