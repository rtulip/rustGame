use crate::traits;
const STARTING_HEALTH: i32 = 10;
pub struct Player{
    pub position: [f64; 2],
    pub health: i32,
}

impl traits::Model<[f64; 2]> for Player {
    fn new(start_position: [f64; 2]) -> Self {
        Player{position: start_position, health: STARTING_HEALTH}
    }
}
