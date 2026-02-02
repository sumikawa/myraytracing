use crate::aabb::Aabb;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::f64::consts::PI;
use std::sync::Arc;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub mat_ptr: Arc<dyn Material>,
    pub front_face: bool,
}

impl HitRecord {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb>;
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat_ptr: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat_ptr,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.at(t);
                let (u, v) = get_sphere_uv(&((p - self.center) / self.radius));
                let mut rec = HitRecord {
                    p,
                    t,
                    u,
                    v,
                    normal: Vec3::new(0.0, 0.0, 0.0),
                    front_face: false,
                    mat_ptr: Arc::clone(&self.mat_ptr), // Assign the sphere's material
                };
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                return Some(rec);
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.at(t);
                let (u, v) = get_sphere_uv(&((p - self.center) / self.radius));
                let mut rec = HitRecord {
                    p,
                    t,
                    u,
                    v,
                    normal: Vec3::new(0.0, 0.0, 0.0),
                    front_face: false,
                    mat_ptr: Arc::clone(&self.mat_ptr), // Assign the sphere's material
                };
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                return Some(rec);
            }
        }
        None
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        let output_box = Aabb::from_points(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        Some(output_box)
    }
}

pub fn get_sphere_uv(p: &Point3) -> (f64, f64) {
    // p: a given point on the sphere of radius 1 centered at the origin.
    // u: returned value [0,1] of angle around Y axis from X=-1.
    // v: returned value [0,1] of angle from Y=-1 to Y=+1.
    //     <1 0 0> yields <0.5 0.5>       <-1 0 0> yields <0.0 0.5>
    //     <0 1 0> yields <0.5 1.0>       <0 -1 0> yields <0.5 0.0>
    //     <0 0 1> yields <0.25 0.5>      <0 0 -1> yields <0.75 0.5>

    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    let u = 1.0 - (phi + PI) / (2.0 * PI);
    let v = (theta + PI / 2.0) / PI;
    (u, v)
}
