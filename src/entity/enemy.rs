use crate::traits::entity;
use crate::traits::state::State;
use crate::traits::draw::{GenericShape,ShapeVariant};
use crate::math::{Vec2, Point2};
use crate::game::consts::{
    ENEMY_SIZE, 
    ENEMY_RADIUS, 
    ENEMY_COLOR, 
    ENEMY_SPEED
};

/// A structure describing the states of the Enemy game components. While in
/// the Beacon state, the Enemy will pathfind towards the Beacon. While in the
/// Player state, the Enemy will pathfind towards the Player
pub enum EnemyState {
    Beacon,
    Player(f64),
}

/// A structure to describe the Enemy game component. They'll try to hunt down
/// the Beacon and the Player. 
pub struct Enemy {
    pub shape: GenericShape,
    pub direction: Vec2,
    pub path: Vec<Point2>,
    pub state: EnemyState,
}

impl Enemy {

    /// Creates a new enemy in the start position.
    pub fn new(start_position: Point2) -> Self {
        Self {
            shape: GenericShape::new(
                ShapeVariant::Circle{
                    size: ENEMY_SIZE,
                    radius: ENEMY_RADIUS,
                }, 
                ENEMY_COLOR,
                start_position
            ),
            direction: Vec2 {x: 0.0, y: 0.0},
            path: Vec::new(),
            state: EnemyState::Beacon
        }
    }

}

impl State for Enemy {
    type StateEnum = EnemyState;
    fn change_state(&mut self, new_state: Self::StateEnum) {
        self.state = new_state;
    }
}

impl entity::Entity for Enemy {
    fn tick(&mut self, dt: f64) {
        if self.path.len() > 0 {
            let mut dist = self.path[0] - self.shape.get_position();
            if (dist.x).abs() + (dist.y).abs() < 7.0 {
                self.path.remove(0);
                if self.path.len() > 0 {
                    dist = self.path[0] - self.shape.get_position();
                    self.direction = Vec2::new_unit_from_point(dist);
                } else {
                    return;
                }
            }
            let delta = Point2 { 
                x: self.direction.x * ENEMY_SPEED * dt, 
                y: self.direction.y * ENEMY_SPEED * dt
            };
            self.shape.update(delta, None);
        }
        
        match self.state {
            EnemyState::Player(t) => {
                self.change_state(EnemyState::Player(t-dt));
            },
            _ => (),
        }


    }
}

