use crate::onb::Onb;
use crate::vec3::{dot, random_cosine_direction, random_unit_vector, unit_vector, Vec3};

pub trait Pdf: Send + Sync {
    fn value(&self, _dir: &Vec3) -> f64 {
        0.0
    }
    fn generate(&self) -> Vec3 {
        Vec3::black()
    }
}

pub struct SpherePDF {}

impl SpherePDF {
    fn _new() -> Self {
        Self {}
    }
}

impl Pdf for SpherePDF {
    fn value(&self, _dir: &Vec3) -> f64 {
        1.0 / (4.0 * std::f64::consts::PI)
    }
    fn generate(&self) -> Vec3 {
        random_unit_vector()
    }
}

pub struct CosinePDF {
    uvw: Onb,
}

impl CosinePDF {
    pub fn new(w: &Vec3) -> Self {
        let uvw = Onb::new(w);
        Self { uvw }
    }
}

impl Pdf for CosinePDF {
    fn value(&self, dir: &Vec3) -> f64 {
        let cosine_theta = dot(&unit_vector(dir), &self.uvw.w());
        f64::max(0.0, cosine_theta / std::f64::consts::PI)
    }
    fn generate(&self) -> Vec3 {
        self.uvw.local(&random_cosine_direction())
    }
}
