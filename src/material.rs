use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Color;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct NoMaterial;

impl Material for NoMaterial {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false // For now, no material scatters anything
    }
}
