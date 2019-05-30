use crate::traits::draw::{GenericShape, ShapeVariant};
use crate::traits::entity::Entity;
use crate::misc::point2d::Point2;
use crate::misc::vector2d::Vec2;
use crate::game::consts::{
    BULLET_WIDTH,
    BULLET_HEIGHT,
    BULLET_COLOR,
    BULLET_SPEED
};


pub struct Bullet {
    pub shape: GenericShape,
    direction: Vec2,
}

impl Bullet {

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
    fn tick(&mut self) {
        let delta = Point2{
            x: self.direction.x * BULLET_SPEED,
            y: self.direction.y * BULLET_SPEED,
        };
        self.shape.update(delta,None);
        // println!("BULLET POSITION: {:?}", self.shape.get_position());

    }
}