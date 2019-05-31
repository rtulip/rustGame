use crate::entity::attack::Attack;
use crate::misc::vector2d::Vec2;
use crate::misc::point2d::Point2;
use crate::traits::draw::{GenericShape, ShapeVariant};
use crate::traits::{entity, state};
use crate::game::consts::{
    PLAYER_SIZE, 
    PLAYER_RADIUS, 
    PLAYER_COLOR, 
    PLAYER_SPEED, 
    PLAYER_STARTING_HEALTH,
    PI,
};

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
    pub shape: GenericShape,
    pub attack: Attack,
    pub health: i32,
    pub state: PlayerState,
    pub direction: Vec2,
    pub resources: i32,
}

impl Player {
    
    /// Creates a new Player
    pub fn new(start_position: Point2) -> Self {
        Player {
            shape: GenericShape::new(
                ShapeVariant::Circle {
                    size: PLAYER_SIZE,
                    radius: PLAYER_RADIUS
                },
                PLAYER_COLOR,
                start_position
            ),
            attack: Attack::new(), 
            health: PLAYER_STARTING_HEALTH,
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
    pub fn update_position(&mut self, dt: f64) {
        match self.state {
            PlayerState::Moving => {
                let delta = Point2{
                    x: self.direction.x * PLAYER_SPEED * dt,
                    y: self.direction.y * PLAYER_SPEED * dt
                };
                self.shape.update(delta, None);
            },
            _ => {}
        }
    }

    /// Sets the Player direction to point towards the cursor. The direction 
    /// must be a unit vector. 
    pub fn update_direction(&mut self, cursor_pos: &Point2) {

        let delta = *cursor_pos - self.shape.center_point();
        self.direction = Vec2::new_unit_from_point(delta);
    
        match self.state {
            PlayerState::Attacking => {
                let mut rad = self.direction.y / self.direction.x;
                rad = rad.atan();
                
                match [self.direction.x < 0.0, self.direction.y < 0.0] {
                    [true, true] => rad = PI * 2.0 - rad,
                    [true, false] => rad = rad * -1.0,
                    [false, true] => rad = PI + rad * -1.0,
                    [false, false] => rad = PI - rad
                }

                rad = PI - rad;

                self.attack.shape.set_rotation(rad);
            }
            _ => {}
        }

    }

}

impl entity::Entity for Player {
    fn tick(&mut self, dt: f64) {
        self.update_position(dt);
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
            [_, PlayerState::Attacking] => {
                self.state = new_state;
                self.attack.shape.set_position(self.shape.center_point());
            }
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
