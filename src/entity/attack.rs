use crate::traits::draw::{GenericShape, ShapeVariant};
use crate::misc::point2d::Point2;
use crate::game::consts::{
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
        Self { 
            shape: GenericShape::new(
                ShapeVariant::Rect{
                    width: PLAYER_ATTACK_WIDTH, 
                    height: PLAYER_ATTACK_HEIGHT
                }, 
                PLAYER_ATTACK_COLOR, 
                position
            ),
        }
    }

}
