use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Clone)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn zero() -> Self {
        Self {
            x: Interval::new(0.0, 0.0),
            y: Interval::new(0.0, 0.0),
            z: Interval::new(0.0, 0.0),
        }
    }
    pub fn _new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }
    pub fn two_point(a: &Point3, b: &Point3) -> Self {
        let x = if a.x <= b.x {
            Interval::new(a.x, b.x)
        } else {
            Interval::new(b.x, a.x)
        };
        let y = if a.y <= b.y {
            Interval::new(a.y, b.y)
        } else {
            Interval::new(b.y, a.y)
        };
        let z = if a.z <= b.z {
            Interval::new(a.z, b.z)
        } else {
            Interval::new(b.z, a.z)
        };
        Self { x, y, z }
    }
    pub fn two_aabb(box0: &AABB, box1: &AABB) -> Self {
        Self {
            x: Interval::two_interval(&box0.x, &box1.x),
            y: Interval::two_interval(&box0.y, &box1.y),
            z: Interval::two_interval(&box0.z, &box1.z),
        }
    }
    pub fn axis_interval(&self, n: u32) -> Interval {
        if n == 1 {
            self.y.clone()
        } else if n == 2 {
            self.z.clone()
        } else {
            self.x.clone()
        }
    }
    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let axis: usize = axis as usize;
            let adinv = 1.0 / ray_dir[axis];

            let t0 = (ax.min - ray_orig[axis]) * adinv;
            let t1 = (ax.max - ray_orig[axis]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0
                };
                if t1 < ray_t.max {
                    ray_t.max = t1
                };
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1
                };
                if t0 < ray_t.max {
                    ray_t.max = t0
                };
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }
    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                0
            } else {
                2
            }
        } else {
            if self.y.size() > self.z.size() {
                1
            } else {
                2
            }
        }
    }
}
