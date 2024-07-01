use std::cmp::Ordering;
use std::cmp::Ordering::{Greater, Less};
use std::sync::Arc;
use rand::{Rng, thread_rng};
use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;

#[derive(Clone)]
pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BvhNode {
    pub fn new(objects: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let axis = thread_rng().gen_range(0..3);

        let object_span = end - start;

        if object_span == 1 {
            Self {
                left: objects[start].clone(),
                right: objects[start].clone(),
                bbox: objects[start].bounding_box().clone(),
            }
        } else if object_span == 2 {
            let left = objects[start].clone();
            let right = objects[start + 1].clone();
            let bbox = AABB::two_aabb(&left.bounding_box(), &right.bounding_box());
            Self {
                left,
                right,
                bbox,
            }
        } else {
            if axis == 0 {
                objects[start..end - 1].sort_unstable_by(|a, b| box_x_compare(a, b))
            } else if axis == 1 {
                objects[start..end - 1].sort_unstable_by(|a, b| box_y_compare(a, b))
            } else {
                objects[start..end - 1].sort_unstable_by(|a, b| box_z_compare(a, b))
            };
            let mid = start + object_span / 2;
            let left= BvhNode::new(objects, start, mid);
            let right = BvhNode::new(objects, mid, end);
            Self {
                left: Arc::new(left.clone()),
                right: Arc::new(right.clone()),
                bbox: AABB::two_aabb(&left.bounding_box(), &right.bounding_box()),
            }
        }
    }
    pub fn from_list(list: &mut HittableList) -> Self {
        let len = list.objects.len();
        Self::new(&mut list.objects, 0, len)
    }
}

fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: u32) -> Ordering {
    let a_axis_interval = a.bounding_box().axis_interval(axis);
    let b_axis_interval = b.bounding_box().axis_interval(axis);
    if a_axis_interval.min < b_axis_interval.min { Less } else { Greater }
}
fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}
fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}
fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r, ray_t.clone()) {
            return None
        };
        return if let Some(hit_left) = self.left.hit(r, ray_t.clone()) {
            if let Some(hit_right) = self.right.hit(r, ray_t.clone()) {
                if hit_left.t < hit_right.t {
                    Some(hit_left)
                } else {
                    Some(hit_right)
                }
            } else {
                Some(hit_left)
            }
        } else {
            if let Some(hit_right) = self.right.hit(r, ray_t.clone()) {
                Some(hit_right)
            }
            else {
                None
            }
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}
