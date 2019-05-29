use crate::traits::entity;
use crate::traits::draw::{GenericShape, ShapeVariant};
use crate::level::MapIdx;
use crate::misc::point2d::Point2;
use crate::game::consts::{
    BEACON_SIZE,
    BEACON_COLOR,
    BEACON_STARTING_HEALTH,
    BEACON_ROTATION_SPEED,
    map_idx_to_point2
};

/// A struct representing the Beacon game component. The beacon is the game 
/// piece the player is trying to defend. If enemies collide with the Beacon,
/// the Beacon will lose health. If the Beacon runs out of health, the game 
/// will be over.
pub struct Beacon {
    pub idx: MapIdx,
    pub shape: GenericShape,
    pub health: i32,
    pub rotation: f64,
}

impl Beacon {
    /// Creates a new beacon.
    pub fn new(pos: MapIdx) -> Self {
        let mut shape = GenericShape::new(
            ShapeVariant::Square{size: BEACON_SIZE},
            BEACON_COLOR,
            map_idx_to_point2(pos)
        );
        shape.update(
            Point2{
                x: BEACON_SIZE / 2.0,
                y: BEACON_SIZE / 2.0,
            },
            None
        );
        shape.set_offset(Point2{
             x: -BEACON_SIZE / 2.0,
             y: -BEACON_SIZE / 2.0,
        });
        Self {
            idx: pos,
            shape: shape,
            health: BEACON_STARTING_HEALTH,
            rotation: 0.0
        }
    }
}

impl entity::Entity for Beacon {
    fn tick(&mut self) {
        let delta = Point2{
            x: 0.0,
            y: 0.0,
        };
        self.shape.update(delta, Some(BEACON_ROTATION_SPEED));
    }
}