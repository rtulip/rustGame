use crate::traits::{shape, entity, state};
use crate::misc::{vector2d, point2d};
const ENEMY_SPEED: f64 = 0.1;

/// A structure describing the states of the Enemy game components. While in
/// the Beacon state, the Enemy will pathfind towards the Beacon. While in the
/// Player state, the Enemy will pathfind towards the Player
pub enum EnemyState {
    Beacon,
    Player,
}

/// A structure to describe the Enemy game component. They'll try to hunt down
/// the Beacon and the Player. 
pub struct Enemy {
    pub position: point2d::Point2,
    pub direction: vector2d::Vec2,
    pub path: Vec<point2d::Point2>,
    pub state: EnemyState,
}

impl Enemy {

    /// Creates a new enemy in the start position.
    pub fn new(start_position: point2d::Point2) -> Self {
        Self {position: start_position, direction: vector2d::Vec2 {x: 0.0, y: 0.0}, path: Vec::new(), state: EnemyState::Beacon}
    }

}

impl shape::Shape for Enemy {
    type ShapeVairant = shape::EllipseType;
    fn get_shape(&self) -> Self::ShapeVairant {
        shape::EllipseType {}
    }
}

impl entity::Entity for Enemy {
    fn tick(&mut self) {
        if self.path.len() > 0 {
            let mut dist = self.path[0] - self.position;
            if (dist.x).abs() < 1.0 && (dist.y).abs() < 1.0 {
                self.path.remove(0);
                if self.path.len() > 0 {
                    dist = self.path[0] - self.position;
                } else {
                    return;
                }
            }
            self.direction = vector2d::Vec2::new_unit_from_point(dist);
            self.position.x += self.direction.x * ENEMY_SPEED;
            self.position.y += self.direction.y * ENEMY_SPEED;
        }
        
    }
}

impl state::State for Enemy {
    type StateEnum = EnemyState;
    fn change_state(&mut self, new_state: Self::StateEnum) {
        self.state = new_state;
    }
}