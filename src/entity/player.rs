use crate::traits::{shape, entity, state};
use crate::misc::vector2d::Vec2;
use std::f64;
const STARTING_HEALTH: i32 = 10;
const PLAYER_SPEED: f64 = 0.1;

/// PlayerState
/// 
/// A struct defining the different states a Player can have
pub enum PlayerState{
    Stationary,
    Moving,
    Attacking,
    FinishedAttacking,
}

/// Player
/// 
/// A representation of the Player. The Player struct is responsible for 
/// the logic surrounding how to update itself.
pub struct Player{
    pub position: [f64; 2],
    pub health: i32,
    pub state: PlayerState,
    pub direction: Vec2,
}

impl Player {
    pub fn new(start_position: [f64; 2]) -> Self {
        Player {
            position: start_position, 
            health: STARTING_HEALTH,
            state: PlayerState::Stationary,
            direction: Vec2::new_unit(0.0, 1.0),
        }
    }

    /// update_position()
    /// 
    /// A function to move the player. The Player moves at PLAYER_SPEED in the 
    /// direction defined by the unit vector self.direction. The Player only 
    /// moves while in the Moving state.
    pub fn update_position(&mut self) {
        match self.state {
            PlayerState::Moving => {
                self.position[0] += self.direction.x * PLAYER_SPEED;
                self.position[1] += self.direction.y * PLAYER_SPEED;
            },
            _ => {}
        }
    }

    

    /// update_direction()
    /// 
    /// args:
    ///     cursor_pos: [f64; 2]: The coordinates of the cursor on the screen.
    ///     player_size: f64: The size of the Player.
    /// 
    /// Sets the Player direction to point towards the cursor. The direction 
    /// must be a unit vector. 
    pub fn update_direction(&mut self, cursor_pos: [f64; 2], player_size: f64) {

        self.direction = Vec2::new_unit(cursor_pos[0] - self.position[0] + player_size/2.0,
                                        cursor_pos[1] - self.position[1] + player_size/2.0);
    }

}

/// Player implements Shape with ShapeVariant EllipseType
impl shape::Shape for Player {
    type ShapeVairant = shape::EllipseType;
    fn get_shape(&self) -> Self::ShapeVairant {
        shape::EllipseType {}
    }
}

/// Player implements Entity 
impl entity::Entity for Player {
    fn tick(&mut self) {
        self.update_position();
    }
}

/// Player implements State with StateEnum PlayerState
impl state::State for Player {
    type StateEnum = PlayerState;
    fn change_state(&mut self, new_state: Self::StateEnum) {
        match [&self.state, &new_state] {
            [PlayerState::Attacking, PlayerState::FinishedAttacking] => {
                self.state = new_state;
            },
            [PlayerState::FinishedAttacking, _] => {
                self.state = new_state;
            },
            [PlayerState::Moving, _] => {
                self.state = new_state;
            },[PlayerState::Stationary, _] => {
                self.state = new_state;
            },
            _ => ()
            
        }
    }
}
