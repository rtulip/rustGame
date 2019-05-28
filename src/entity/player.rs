use crate::traits::{shape, entity, state};
use crate::misc::vector2d::Vec2;
use crate::misc::point2d::Point2;
use std::f64;
const STARTING_HEALTH: i32 = 10000;
const PLAYER_SPEED: f64 = 0.1;

/// A struct defining the different states a Player can have. While Stationary,
/// the Player isn't moving. While Moving, the player will move in the 
/// direction of the Mouse Cursor. While attacking, the player will remain
/// stationary. The Player can only move out of the Attacking state to the 
/// FinishedAttacking state, which acts as a signal saying the attacking 
/// animation has finished. 
pub enum PlayerState{
    Stationary,
    Moving,
    Attacking,
    FinishedAttacking,
}

/// A representation of the Player. The Player struct is responsible for 
/// the logic surrounding how to update itself.
pub struct Player {
    pub position: Point2,
    pub health: i32,
    pub state: PlayerState,
    pub direction: Vec2,
    pub resources: i32,
}

impl Player {
    
    /// Creates a new Player
    pub fn new(start_position: Point2) -> Self {
        Player {
            position: start_position, 
            health: STARTING_HEALTH,
            state: PlayerState::Stationary,
            direction: Vec2::new_unit(0.0, 1.0),
            resources: 0,
        }
    }

    /// A function to move the player. The Player moves at PLAYER_SPEED in the 
    /// direction defined by the unit vector self.direction. The Player only 
    /// moves while in the Moving state.
    /// 
    /// Assumes that direction is a unit vector.
    pub fn update_position(&mut self) {
        match self.state {
            PlayerState::Moving => {
                self.position.x += self.direction.x * PLAYER_SPEED;
                self.position.y += self.direction.y * PLAYER_SPEED;
            },
            _ => {}
        }
    }

    /// Sets the Player direction to point towards the cursor. The direction 
    /// must be a unit vector. 
    pub fn update_direction(&mut self, cursor_pos: &Point2, player_size: f64) {

        self.direction = Vec2::new_unit(cursor_pos.x - self.position.x + player_size/2.0,
                                        cursor_pos.y - self.position.y + player_size/2.0);
    
    }

}

impl shape::Shape for Player {
    type ShapeVairant = shape::EllipseType;
    fn get_shape(&self) -> Self::ShapeVairant {
        shape::EllipseType {}
    }
}

impl entity::Entity for Player {
    fn tick(&mut self) {
        self.update_position();
    }
}

impl state::State for Player {
    type StateEnum = PlayerState;
    /// Can transition from any state to any state with the exception of the 
    /// Attacking state. 
    /// 
    /// The Player can transition to the attacking state from any state, but
    /// must transition to the FinishedAttacking state from the Attacking state
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
