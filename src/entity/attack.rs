use crate::traits::draw::{GenericShape, ShapeVariant};
use crate::misc::point2d::Point2;
use crate::game::consts::{
    PLAYER_SIZE,
    PLAYER_ATTACK_HEIGHT,
    PLAYER_ATTACK_WIDTH,
    PLAYER_ATTACK_COLOR,
};

/// A structure representing the player attack. Attacks are of a RectangleType
pub struct Attack {
    pub shape: GenericShape
}

impl Attack {
    
    pub fn new(position: Point2) -> Self {
        let mut shape = GenericShape::new(
            ShapeVariant::Rect{
                width: PLAYER_ATTACK_WIDTH, 
                height: PLAYER_ATTACK_HEIGHT
            },
            PLAYER_ATTACK_COLOR, 
            position
        );
            
        shape.update(
            Point2{
                x: PLAYER_SIZE / 2.0,
                y: PLAYER_SIZE / 2.0,
            },
            None
        );
        Self { 
            shape: shape,
        }
    }

}
