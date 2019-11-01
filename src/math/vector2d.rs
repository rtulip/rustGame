use crate::math::Point2;
#[derive(Clone, Copy, Debug)]
/// A structure to represent a vector in 2d space.
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    /// Creates a new vector
    pub fn new(x: f64, y: f64) -> Self {
        Self { x: x, y: y }
    }

    /// Creates a new unit vector
    pub fn new_unit(x: f64, y: f64) -> Self {
        Vec2::normalize(Self { x: x, y: y })
    }

    /// Converts a point to a vector
    pub fn new_from_point(p: Point2) -> Self {
        Self { x: p.x, y: p.y }
    }

    /// Converts a point to a unit vector
    pub fn new_unit_from_point(p: Point2) -> Self {
        Vec2::normalize(Self { x: p.x, y: p.y })
    }

    /// Uses an approximation method to convert a vector to a unit vector
    #[allow(unused_assignments)]
    fn normalize(vector: Vec2) -> Vec2 {
        let ax = vector.x.abs();
        let ay = vector.y.abs();
        let mut ratio = 1.0;
        match ax > ay {
            true => {
                ratio = 1.0 / ax;
            }
            false => {
                ratio = 1.0 / ay;
            }
        };
        ratio = ratio * (1.29289 - (ax + ay) * ratio * 0.29289);
        Vec2 {
            x: vector.x * ratio,
            y: vector.y * ratio,
        }
    }

    pub fn dot_product(vec1: Vec2, vec2: Vec2) -> f64 {
        vec1.x * vec2.x + vec1.y * vec2.y
    }

    pub fn normal_unit(&self) -> Vec2 {
        Vec2::normalize(Self {
            x: self.y,
            y: -self.x,
        })
    }
}

#[cfg(test)]
mod vector2d_tests {

    use super::Vec2;

    // the acceptable amount of error
    const EPSILON: f64 = 0.03;

    #[test]
    fn test_normalize() {
        let v = Vec2::new_unit(3.0, 4.0);
        let u = Vec2::new(3.0 / 5.0, 4.0 / 5.0);
        assert!(EPSILON > (u.x - v.x).abs());
        assert!(EPSILON > (u.y - v.y).abs());
    }

    #[test]
    fn test_dot_product() {
        let v1 = Vec2::new(3.0, 4.0);
        let v2 = Vec2::new(-0.5, 0.5);
        let val = 3.0 * -0.5 + 4.0 * 0.5;
        assert_eq!(val, Vec2::dot_product(v1, v2));
    }

    #[test]
    fn test_normal_unit() {
        let v = Vec2::new(3.0, 4.0);
        let v = v.normal_unit();
        let u = Vec2::new(4.0 / 5.0, -3.0 / 5.0);

        assert!(EPSILON > (u.x - v.x).abs());
        assert!(EPSILON > (u.y - v.y).abs());
    }
}
