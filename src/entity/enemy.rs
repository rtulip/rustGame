use crate::traits::{shape, entity, state};
use crate::misc::{vector2d, point2d};
const ENEMY_SPEED: f64 = 0.1;

pub enum EnemyState {
    Beacon,
    Player,
}

pub struct Enemy {
    pub position: point2d::Point2,
    pub direction: vector2d::Vec2,
    pub path: Vec<point2d::Point2>,
    pub state: EnemyState,
}

impl Enemy {

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
        self.position.x += self.direction.x * ENEMY_SPEED;
        self.position.y += self.direction.y * ENEMY_SPEED;
    }
}

impl state::State for Enemy {
    type StateEnum = EnemyState;
    fn change_state(&mut self, new_state: Self::StateEnum) {
        self.state = new_state;
    }
}