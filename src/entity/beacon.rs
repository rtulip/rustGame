use crate::traits::{shape, entity};
use std::f64;

const ROTATION_SPEED: f64 = 0.01;
const STARTING_HEALTH: i32 = 10;

pub struct Beacon {
    pub position: [f64;2],
    pub health: i32,
    pub rotation: f64,
}

impl Beacon {
    pub fn new(pos: [f64;2]) -> Self {
        Self {
            position: pos,
            health: STARTING_HEALTH,
            rotation: 0.0
        }
    }
}

impl shape::Shape for Beacon {
    type ShapeVairant = shape::RectangleType;
    fn get_shape(&self) -> Self::ShapeVairant {
        shape::RectangleType {}
    }
}

impl entity::Entity for Beacon {
    fn tick(&mut self) {
        self.rotation += ROTATION_SPEED;
        if self.rotation > 2.0 * f64::consts::PI {
            self.rotation = 0.0;
        }
    }
}