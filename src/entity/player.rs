use crate::traits::{shape, entity, state};
const STARTING_HEALTH: i32 = 10;
pub enum PlayerState{
    Stationary,
    Moving,
}

pub struct Player{
    pub position: [f64; 2],
    pub health: i32,
    pub state: PlayerState,
}

impl Player {
    pub fn new(start_position: [f64; 2]) -> Self {
        Player {
            position: start_position, 
            health: STARTING_HEALTH,
            state: PlayerState::Stationary,
        }
    }
}

impl shape::Shape for Player {
    type ShapeVairant = shape::CircleType;
    fn get_shape(&self) -> Self::ShapeVairant {
        shape::CircleType {}
    }
}

impl entity::Entity for Player {
    fn tick(&mut self) {
        self.position[1] += 1.0;
    }
}

impl state::State for Player {
    type StateEnum = PlayerState;
    fn change_state(&mut self, new_state: Self::StateEnum) {
        self.state = new_state;
    }
}
