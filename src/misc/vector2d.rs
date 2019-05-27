use crate::misc::point2d::Point2;
#[derive(Clone,Copy,Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self {x: x, y: y}
    }

    pub fn new_unit(x: f64, y: f64) -> Self {
        Vec2::convert_to_unit_vector(Self {x: x, y: y})
    }

    pub fn new_from_point(p: Point2) -> Self {
        Self {x: p.x, y: p.y}
    }

    pub fn new_unit_from_point(p: Point2) -> Self {
        Vec2::convert_to_unit_vector(Self {x: p.x, y: p.y})
    }

    /// convert_to_unit_vector()
    /// 
    /// args:
    ///     vector: vec2: A vector to be converted to a unit vector.
    /// 
    /// returns: A unit vector of type vec2.
    /// 
    /// Uses an approximation method is used to calculate the unit vector.
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

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

