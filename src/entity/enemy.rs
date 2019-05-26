use crate::traits::{shape, entity, state};
const ENEMY_SPEED: f64 = 0.1;

pub enum EnemyState {
    Beacon,
    Player,
}

pub struct Enemy {
    pub position: [f64; 2],
    pub direction: [f64; 2],
    pub state: EnemyState,
}

impl shape::Shape for Enemy {
    type ShapeVairant = shape::EllipseType;
    fn get_shape(&self) -> Self::ShapeVairant {
        shape::EllipseType {}
    }
}

impl entity::Entity for Enemy {
    fn tick(&mut self) {
        self.position[0] += self.direction[0] * ENEMY_SPEED;
        self.position[1] += self.direction[0] * ENEMY_SPEED;
    }
}

impl state::State for Enemy {
    type StateEnum = EnemyState;
    fn change_state(&mut self, new_state: Self::StateEnum) {
        self.state = new_state;
    }
}