const STARTING_HEALTH: i32 = 10;
pub struct Player{
    pub position: [f64; 2],
    pub health: i32,
}

impl Player {

    pub fn new(start_position: [f64; 2]) -> Player {
        Player{position: start_position, health: STARTING_HEALTH}
    }

}
