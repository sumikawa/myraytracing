use crate::aabb::{Aabb, surrounding_box};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use rand::Rng;
use std::cmp::Ordering;
use std::sync::Arc;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(objects: &mut [Arc<dyn Hittable>], time0: f64, time1: f64) -> Self {
        let axis = rand::rng().random_range(0..3);
        let comparator = |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| box_compare(a, b, axis);

        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;

        let object_span = objects.len();

        if object_span == 1 {
            left = objects[0].clone();
            right = objects[0].clone();
        } else if object_span == 2 {
            if comparator(&objects[0], &objects[1]) == Ordering::Less {
                left = objects[0].clone();
                right = objects[1].clone();
            } else {
                left = objects[1].clone();
                right = objects[0].clone();
            }
        } else {
            objects.sort_by(comparator);
            let mid = object_span / 2;
            let (left_half, right_half) = objects.split_at_mut(mid);
            left = Arc::new(BvhNode::new(left_half, time0, time1));
            right = Arc::new(BvhNode::new(right_half, time0, time1));
        }

        let box_left = left.bounding_box(time0, time1);
        let box_right = right.bounding_box(time0, time1);

        let bbox = if let (Some(left_box), Some(right_box)) = (box_left, box_right) {
            surrounding_box(&left_box, &right_box)
        } else {
            panic!("No bounding box in BvhNode constructor.");
        };

        Self { left, right, bbox }
    }
}

fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> Ordering {
    let box_a = a.bounding_box(0.0, 0.0);
    let box_b = b.bounding_box(0.0, 0.0);

    if let (Some(a), Some(b)) = (box_a, box_b) {
        let a_min = match axis {
            0 => a.min.x,
            1 => a.min.y,
            2 => a.min.z,
            _ => unreachable!(),
        };
        let b_min = match axis {
            0 => b.min.x,
            1 => b.min.y,
            2 => b.min.z,
            _ => unreachable!(),
        };
        a_min.partial_cmp(&b_min).unwrap_or(Ordering::Equal)
    } else {
        panic!("No bounding box in box_compare.");
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bbox.hit(r, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(r, t_min, t_max);

        let t_max_for_right = if let Some(ref rec) = hit_left {
            rec.t
        } else {
            t_max
        };

        let hit_right = self.right.hit(r, t_min, t_max_for_right);

        if hit_right.is_some() {
            hit_right
        } else {
            hit_left
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        Some(self.bbox)
    }
}
