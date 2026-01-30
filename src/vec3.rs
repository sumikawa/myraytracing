use crate::rtweekend::{random_double, random_double_range};
use std::f64::consts::PI;
use std::fmt;
use std::ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point3(pub Vec3);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color(pub Vec3);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn random() -> Self {
        Self {
            x: random_double(),
            y: random_double(),
            z: random_double(),
        }
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Self {
            x: random_double_range(min, max),
            y: random_double_range(min, max),
            z: random_double_range(min, max),
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        let a = random_double_range(0.0, 2.0 * PI);
        let z = random_double_range(-1.0, 1.0);
        let r = (1.0 - z * z).sqrt();

        Self {
            x: r * a.cos(),
            y: r * a.sin(),
            z,
        }
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - (*n * 2.0 * v.dot(*n))
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta: f64 = -uv.dot(*n);
        let r_out_parallel: Vec3 = etai_over_etat * (*uv + cos_theta * *n);
        let r_out_perp: Vec3 = -((1.0 - r_out_parallel.length_squared()).sqrt()) * *n;

        r_out_parallel + r_out_perp
    }

    pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(
                random_double_range(-1.0, 1.0),
                random_double_range(-1.0, 1.0),
                0.0,
            );
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Vec3::new(x, y, z))
    }
}

impl Color {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Vec3::new(x, y, z))
    }

    pub fn random() -> Self {
        Self(Vec3::random())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Self(Vec3::random_range(min, max))
    }
}

impl Deref for Point3 {
    type Target = Vec3;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Point3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Color {
    type Target = Vec3;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl Default for Point3 {
    fn default() -> Self {
        Self(Vec3::default())
    }
}

impl Default for Color {
    fn default() -> Self {
        Self(Vec3::default())
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

// Point3 operators
impl Sub for Point3 {
    type Output = Vec3;
    fn sub(self, other: Self) -> Self::Output {
        self.0 - other.0
    }
}

impl Add<Vec3> for Point3 {
    type Output = Self;
    fn add(self, other: Vec3) -> Self::Output {
        Self(self.0 + other)
    }
}

impl Sub<Vec3> for Point3 {
    type Output = Self;
    fn sub(self, other: Vec3) -> Self::Output {
        Self(self.0 - other)
    }
}

// Color operators
impl Add for Color {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        self.0.add_assign(other.0);
    }
}

impl Mul for Color {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0)
    }
}

impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        self.0.mul_assign(rhs);
    }
}

impl Div<f64> for Color {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        self.0.div_assign(rhs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let result = v1 + v2;
        assert_eq!(result, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_unit_vector() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let unit_direction = a.unit_vector();
        // A simple check to see if the length is close to 1.0
        assert!((unit_direction.length() - 1.0).abs() < 0.000001);
    }
}
