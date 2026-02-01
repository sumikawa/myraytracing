use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Debug, Copy, Clone)]
pub struct Aabb {
    pub min: Point3,
    pub max: Point3,
}

impl Aabb {
    pub fn new() -> Self {
        Aabb {
            min: Point3::default(),
            max: Point3::default(),
        }
    }

    pub fn from_points(a: Point3, b: Point3) -> Self {
        Aabb { min: a, max: b }
    }

    pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for a in 0..3 {
            let t0_numerator = match a {
                0 => self.min.x - r.origin.x,
                1 => self.min.y - r.origin.y,
                2 => self.min.z - r.origin.z,
                _ => unreachable!(),
            };
            let t1_numerator = match a {
                0 => self.max.x - r.origin.x,
                1 => self.max.y - r.origin.y,
                2 => self.max.z - r.origin.z,
                _ => unreachable!(),
            };
            let r_direction_a = match a {
                0 => r.direction.x,
                1 => r.direction.y,
                2 => r.direction.z,
                _ => unreachable!(),
            };

            let t0_candidate = t0_numerator / r_direction_a;
            let t1_candidate = t1_numerator / r_direction_a;

            let t0 = f64::min(t0_candidate, t1_candidate);
            let t1 = f64::max(t0_candidate, t1_candidate);

            t_min = f64::max(t0, t_min);
            t_max = f64::min(t1, t_max);

            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}

pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
    let small = Point3::new(
        f64::min(box0.min.x, box1.min.x),
        f64::min(box0.min.y, box1.min.y),
        f64::min(box0.min.z, box1.min.z),
    );
    let big = Point3::new(
        f64::max(box0.max.x, box1.max.x),
        f64::max(box0.max.y, box1.max.y),
        f64::max(box0.max.z, box1.max.z),
    );
    Aabb::from_points(small, big)
}
