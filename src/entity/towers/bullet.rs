use crate::traits::draw::{GenericShape, ShapeVariant};
use crate::traits::entity::Entity;
use crate::math::Point2;
use crate::math::Vec2;
use crate::game::consts::{
    BULLET_WIDTH,
    BULLET_HEIGHT,
    BULLET_COLOR,
    BULLET_SPEED
};

/// A structure to represent a bullet fired from a tower.
pub struct Bullet {
    pub shape: GenericShape,
    direction: Vec2,
}

impl Bullet {
    /// Returns a new Bullet at the input position facing the input direction.
    pub fn new(position: Point2, direction: Vec2) -> Self {
        Self {
            shape: GenericShape::new(
                ShapeVariant::Rect{
                    width: BULLET_WIDTH,
                    height: BULLET_HEIGHT,
                }, 
                BULLET_COLOR, 
                position,
            ),
            direction: direction,
        }
    }

}

impl Entity for Bullet {
    /// Moves the bullet forward every tick.
    fn tick(&mut self, dt: f64) {
        let delta = Point2{
            x: self.direction.x * BULLET_SPEED * dt,
            y: self.direction.y * BULLET_SPEED * dt,
        };
        self.shape.update(delta,None);
    }
}