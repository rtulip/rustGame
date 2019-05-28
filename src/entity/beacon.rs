use crate::traits::{shape, entity};
use crate::level::MapIdx;
use std::f64;

const ROTATION_SPEED: f64 = 0.01;
const STARTING_HEALTH: i32 = 4;

/// A struct representing the Beacon game component. The beacon is the game 
/// piece the player is trying to defend. If enemies collide with the Beacon,
/// the Beacon will lose health. If the Beacon runs out of health, the game 
/// will be over.
pub struct Beacon {
    pub position: MapIdx,
    pub health: i32,
    pub rotation: f64,
}

impl Beacon {
    /// Creates a new beacon.
    pub fn new(pos: MapIdx) -> Self {
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