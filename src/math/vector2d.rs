use crate::math::Point2;
#[derive(Clone,Copy,Debug)]
/// A structure to represent a vector in 2d space.
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    
    /// Creates a new vector
    pub fn new(x: f64, y: f64) -> Self {
        Self {x: x, y: y}
    }

    /// Creates a new unit vector
    pub fn new_unit(x: f64, y: f64) -> Self {
        Vec2::convert_to_unit_vector(Self {x: x, y: y})
    }

    /// Converts a point to a vector
    pub fn new_from_point(p: Point2) -> Self {
        Self {x: p.x, y: p.y}
    }

    /// Converts a point to a unit vector
    pub fn new_unit_from_point(p: Point2) -> Self {
        Vec2::convert_to_unit_vector(Self {x: p.x, y: p.y})
    }

    /// Uses an approximation method to convert a vector to a unit vector
    #[allow(unused_assignments)]
    fn convert_to_unit_vector(vector: Vec2) -> Vec2 {
        let ax = vector.x.abs();
        let ay = vector.y.abs();
        let mut ratio = 1.0;
        match ax > ay {
            true => {
                ratio = 1.0 / ax;
            },
            false => {
                ratio = 1.0 / ay;
            },
        };
        ratio = ratio * (1.29289 - (ax + ay) * ratio * 0.29289);
        Vec2 {x: vector.x * ratio, y: vector.y * ratio}
    }

    /// returns the length of a vector.
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

