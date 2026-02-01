use crate::aabb::{Aabb, surrounding_box};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::sync::Arc;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn with_object(object: Arc<dyn Hittable>) -> Self {
        let mut list = Self::new();
        list.add(object);
        list
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }

        temp_rec
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        if self.objects.is_empty() {
            return None;
        }

        let mut output_box: Option<Aabb> = None;

        for object in &self.objects {
            if let Some(bbox) = object.bounding_box(t0, t1) {
                if let Some(prev_bbox) = output_box {
                    output_box = Some(surrounding_box(&prev_bbox, &bbox));
                } else {
                    output_box = Some(bbox);
                }
            } else {
                return None;
            }
        }
        output_box
    }
}
