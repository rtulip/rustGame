use crate::traits::{shape, entity};
use crate::traits::draw::{GenericShape, ShapeVariant};
use crate::misc::point2d::Point2;
use crate::game::consts::{
    DROP_SIZE,
    DROP_ROTATION_SPEED,
    RESOURCE_COLOR,
};
use std::f64::consts::PI;

const ROTATION_SPEED: f64 = -0.01;

/// A structure to represent a tower resource which can be used by the Player
pub struct Resource {
    pub shape: GenericShape,
    pub rotation: f64,
}

impl Resource {
    /// Creates a new Resource
    pub fn new(position: Point2) -> Self {
        Self {
            shape: GenericShape::new(
                ShapeVariant::Square{size: DROP_SIZE},
                RESOURCE_COLOR, 
                position
            ),
            rotation: 0.0,
        }
    }
}

impl entity::Entity for Resource {

    /// Rotates the Resource
    fn tick(&mut self){
        self.shape.update(Point2{x: 0.0, y: 0.0}, Some(DROP_ROTATION_SPEED));
    }
}