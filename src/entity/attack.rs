use crate::game::consts::{PLAYER_ATTACK_COLOR, PLAYER_ATTACK_HEIGHT, PLAYER_ATTACK_WIDTH};
use crate::math::Point2;
use crate::traits::draw::{GenericShape, ShapeVariant};

/// A structure representing the player attack. Attacks are of a RectangleType
pub struct Attack {
    pub shape: GenericShape,
}

impl Attack {
    pub fn new() -> Self {
        Self {
            shape: GenericShape::new(
                ShapeVariant::Rect {
                    width: PLAYER_ATTACK_WIDTH,
                    height: PLAYER_ATTACK_HEIGHT,
                },
                PLAYER_ATTACK_COLOR,
                Point2 { x: 0.0, y: 0.0 },
            ),
        }
    }
}
