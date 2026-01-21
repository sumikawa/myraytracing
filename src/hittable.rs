use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.at(t);
                let mut rec = HitRecord {
                    p,
                    t,
                    normal: Vec3::new(0.0, 0.0, 0.0),
                    front_face: false,
                };
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                return Some(rec);
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.at(t);
                let mut rec = HitRecord {
                    p,
                    t,
                    normal: Vec3::new(0.0, 0.0, 0.0),
                    front_face: false,
                };
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                return Some(rec);
            }
        }
        None
    }
}
