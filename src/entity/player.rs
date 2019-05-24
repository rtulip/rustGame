use crate::traits::{shape, entity, state};
const STARTING_HEALTH: i32 = 10;
const PLAYER_SPEED: f64 = 0.1;

pub enum PlayerState{
    Stationary,
    Moving,
}

pub struct Player{
    pub position: [f64; 2],
    pub health: i32,
    pub state: PlayerState,
    pub direction: [f64; 2],
}

impl Player {
    pub fn new(start_position: [f64; 2]) -> Self {
        Player {
            position: start_position, 
            health: STARTING_HEALTH,
            state: PlayerState::Stationary,
            direction: [0.0, 1.0],
        }
    }

    pub fn update_position(&mut self) {

        self.position[0] += self.direction[0] * PLAYER_SPEED;
        self.position[1] += self.direction[1] * PLAYER_SPEED;

    }

    pub fn update_direction(&mut self, cursor_pos: [f64; 2], player_size: f64) {

        self.direction = self.convert_to_unit_vector(
                    [
                        cursor_pos[0] - self.position[0] + player_size/2.0,
                        cursor_pos[1] - self.position[1] + player_size/2.0
                    ]);
    }

    fn convert_to_unit_vector(&self, vector: [f64; 2]) -> [f64;2] {
        let ax = vector[0].abs();
        let ay = vector[1].abs();
        let mut ratio = 1.0;
        match ax > ay {
            true => {
                ratio = 1.0 / ax;
            },
            false => {
                ratio = 1.0 / ay;
            },
        };
        ratio = ratio * (1.29289 - (ax + ay) * ratio * 0.29289);
        [vector[0] * ratio, vector[1] * ratio]

        
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
        self.update_position();
    }
}

impl state::State for Player {
    type StateEnum = PlayerState;
    fn change_state(&mut self, new_state: Self::StateEnum) {
        self.state = new_state;
    }
}
