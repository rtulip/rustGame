use crate::traits::{entity};
use crate::traits::draw::{GenericShape, ShapeVariant};
use crate::math::Point2;
use crate::game::consts::{
    DROP_SIZE,
    DROP_ROTATION_SPEED,
    RESOURCE_COLOR,
};
/// A structure to represent a tower resource which can be used by the Player
pub struct Resource {
    pub shape: GenericShape,
    pub rotation: f64,
}

impl Resource {
    /// Creates a new Resource
    pub fn new(position: Point2) -> Self {
        let mut shape = GenericShape::new(
            ShapeVariant::Rect{
                width: DROP_SIZE,
                height: DROP_SIZE,
            },
            RESOURCE_COLOR, 
            position
        );
        shape.set_offset(Point2{
            x: DROP_SIZE / -2.0, 
            y: DROP_SIZE / -2.0
        });
        Self {
            shape: shape,
            rotation: 0.0,
        }
    }
}

impl entity::Entity for Resource {
    /// Rotates the Resource
    fn tick(&mut self, dt: f64){
        self.shape.update(Point2{x: 0.0, y: 0.0}, Some(DROP_ROTATION_SPEED * dt));
    }
}