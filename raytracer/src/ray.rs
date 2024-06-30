use crate::vec3::Point3;
use crate::vec3::Vec3;
use std::f64;

pub struct Ray {
    _orig: Point3,
    dir: Vec3,
    tm: f64,
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3, tm: f64) -> Self {
        Self {
            _orig: origin.clone(),
            dir: direction.clone(),
            tm,
        }
    }

    pub fn origin(&self) -> Point3 {
        self._orig.clone()
    }
    pub fn direction(&self) -> Vec3 {
        self.dir.clone()
    }
    pub fn time(&self) -> f64 {
        self.tm
    }

    pub fn at(&self, t: f64) -> Point3 {
        self._orig.clone() + self.dir.clone() * t
    }
}
