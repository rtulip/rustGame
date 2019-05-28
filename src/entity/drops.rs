use crate::traits::{shape, entity};
use crate::misc::point2d::Point2;
use std::f64::consts::PI;

const ROTATION_SPEED: f64 = -0.01;

pub struct Resource {
    pub position: Point2,
    pub rotation: f64,
}

impl Resource {
    pub fn new(position: Point2) -> Self {
        Self {
            position: position,
            rotation: 0.0,
        }
    }
}

impl shape::Shape for Resource{
    type ShapeVairant = shape::RectangleType;
    fn get_shape(&self) -> Self::ShapeVairant {
        shape::RectangleType {}
    }
}

impl entity::Entity for Resource {

    fn tick(&mut self){
        self.rotation += ROTATION_SPEED;
        if self.rotation < -2.0 * PI {
            self.rotation = 0.0;
        }
    }
}