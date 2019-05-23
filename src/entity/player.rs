use crate::shape;
const STARTING_HEALTH: i32 = 10;
pub struct Player{
    pub position: [f64; 2],
    pub health: i32,
}

impl Player {
    pub fn new(start_position: [f64; 2]) -> Self {
        Player{position: start_position, health: STARTING_HEALTH}
    }
}

impl shape::Shape for Player {
    type ShapeVairant = shape::CircleType;
    fn get_shape(&self) -> Self::ShapeVairant {
        shape::CircleType {}
    }
}

