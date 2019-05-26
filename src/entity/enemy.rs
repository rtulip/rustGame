use crate::traits::{shape, entity, state};
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
