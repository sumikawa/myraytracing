use rand::Rng;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    // Returns a random real in [0,1).
    rand::rng().random_range(0.0..1.0)
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    rand::rng().random_range(min..max)
}

pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degrees_to_radians() {
        let epsilon = 1e-9;
        assert!((degrees_to_radians(0.0) - 0.0).abs() < epsilon);
        assert!((degrees_to_radians(90.0) - PI / 2.0).abs() < epsilon);
        assert!((degrees_to_radians(180.0) - PI).abs() < epsilon);
        assert!((degrees_to_radians(360.0) - 2.0 * PI).abs() < epsilon);
    }

    #[test]
    fn test_random_double() {
        for _ in 0..1000 {
            let r = random_double();
            assert!(r >= 0.0 && r < 1.0, "random_double() returned {} which is not in [0, 1)", r);
        }
    }

    #[test]
    fn test_random_double_range() {
        let min = -10.0;
        let max = 10.0;
        for _ in 0..100 {
            let r = random_double_range(min, max);
            assert!(r >= min && r < max, "random_double_range({}, {}) returned {} which is not in [{}, {})", min, max, r, min, max);
        }

        let min_small = 0.001;
        let max_small = 0.002;
        for _ in 0..100 {
            let r = random_double_range(min_small, max_small);
            assert!(r >= min_small && r < max_small, "random_double_range({}, {}) returned {} which is not in [{}, {})", min_small, max_small, r, min_small, max_small);
        }
    }

    #[test]
    fn test_clamp() {
        // Test values within the range
        assert_eq!(clamp(0.5, 0.0, 1.0), 0.5);
        assert_eq!(clamp(0.0, 0.0, 1.0), 0.0);
        assert_eq!(clamp(1.0, 0.0, 1.0), 1.0);

        // Test values below the minimum
        assert_eq!(clamp(-0.5, 0.0, 1.0), 0.0);
        assert_eq!(clamp(-10.0, -5.0, 5.0), -5.0);

        // Test values above the maximum
        assert_eq!(clamp(1.5, 0.0, 1.0), 1.0);
        assert_eq!(clamp(10.0, -5.0, 5.0), 5.0);

        // Test with negative ranges
        assert_eq!(clamp(-1.0, -2.0, -0.5), -1.0);
        assert_eq!(clamp(-3.0, -2.0, -0.5), -2.0);
        assert_eq!(clamp(0.0, -2.0, -0.5), -0.5);

        // Test with min > max (should still work due to f64::max then f64::min logic, effectively clamping to max then min if value is outside)
        // However, standard clamp assumes min <= max. If min > max, the result will be max if value <= max, else min if value >= min.
        // Let's assume min <= max for typical clamp behavior. If this is not the case, the function definition should be adjusted or clarified.
        assert_eq!(clamp(0.5, 1.0, 0.0), 0.0); // value.max(min) -> 1.0, then 1.0.min(max) -> 0.0
        assert_eq!(clamp(1.5, 1.0, 0.0), 0.0); // value.max(min) -> 1.5, then 1.5.min(max) -> 0.0
        assert_eq!(clamp(-0.5, 1.0, 0.0), 0.0); // value.max(min) -> 1.0, then 1.0.min(max) -> 0.0
    }
}
