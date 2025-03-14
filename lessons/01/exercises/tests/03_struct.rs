//! Run this file with `cargo test --test 03_struct`.

// TODO: Implement a `Vec3` structure that represents a three-dimensional vector.
// Use field names `x`, `y` and `z`.
// Implement `new`, `add`, `length` and `normalize` methods, so that tests pass.
// If you `normalize` a vector that has length 0, it should return a zero-length vector.

struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
        }
    }
}

impl Vec3 {
    pub fn add(&self, vector: Vec3) -> Vec3 {
        let new_vector = Vec3::new(self.x + vector.x, self.y + vector.y, self.z + vector.z);
        return new_vector;
    }
}

impl Vec3 {
    pub fn length(&self) -> f64 {
        let mut vec_length:f64 = self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0);
        vec_length = vec_length.sqrt();
        return vec_length;
    }
}

impl Vec3 {
    pub fn normalize(self) -> Vec3 {
        let magnitude = self.length();
        let mut normalized_vector = Vec3::new(0.0,0.0,0.0);
        if magnitude != 0.0 {
            normalized_vector = Vec3::new(self.x/magnitude, self.y/magnitude, self.z/magnitude);
        }
        return normalized_vector;
    }
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn new() {
        let v1 = Vec3::new(1.2, 3.5, 6.0);
        assert_eq!(v1.x, 1.2);
        assert_eq!(v1.y, 3.5);
        assert_eq!(v1.z, 6.0);
    }

    #[test]
    fn add() {
        let v1 = Vec3::new(1.2, 3.5, 6.0);
        let v2 = Vec3::new(4.8, 6.2, -2.3);
        let v3 = v1.add(v2);
        assert_almost_eq(v3.x, 6.0);
        assert_almost_eq(v3.y, 9.7);
        assert_almost_eq(v3.z, 3.7);
    }

    #[test]
    fn length_zero() {
        assert_eq!(Vec3::new(0.0, 0.0, 0.0).length(), 0.0);
    }

    #[test]
    fn length() {
        assert_almost_eq(Vec3::new(-6.2, 13.85, 12.8).length(), 19.852);
    }

    #[test]
    fn normalize_zero() {
        let norm = Vec3::new(0.0, 0.0, 0.0).normalize();
        assert_eq!(norm.x, 0.0);
        assert_eq!(norm.y, 0.0);
        assert_eq!(norm.z, 0.0);
    }

    #[test]
    fn normalize() {
        let norm = Vec3::new(1.5, 28.4, -5.6).normalize();
        assert_almost_eq(norm.x, 0.051);
        assert_almost_eq(norm.y, 0.98);
        assert_almost_eq(norm.z, -0.1932);
    }

    #[track_caller]
    fn assert_almost_eq(value: f64, expected: f64) {
        assert!(
            (value - expected).abs() < 0.001,
            "{value} does not equal {expected}"
        );
    }
}
