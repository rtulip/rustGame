const STARTING_HEALTH
pub struct Player{
    pub position: [f64; 2],
    pub health: i32,
}

impl Player {

    pub fn new(start_position: [f64; 2]) -> Player {
        Player{position: start_position, health: STARTING_HEALTH}
    }

}
