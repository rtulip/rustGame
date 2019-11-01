use crate::game::consts::{
    map_idx_to_point2, BEACON_COLOR, BEACON_ROTATION_SPEED, BEACON_SIZE, BEACON_STARTING_HEALTH,
    DAMAGE_COLOR, HEALTH_BAR_HEIGHT, HEALTH_COLOR,
};
use crate::levels::map::MapIdx;
use crate::math::Point2;
use crate::traits::draw::{GenericShape, ShapeVariant};
use crate::traits::entity;

/// A struct representing the Beacon game component. The beacon is the game
/// piece the player is trying to defend. If enemies collide with the Beacon,
/// the Beacon will lose health. If the Beacon runs out of health, the game
/// will be over.
pub struct Beacon {
    pub idx: MapIdx,
    pub shape: GenericShape,
    pub health_bar: GenericShape,
    pub damage_bar: GenericShape,
    pub health: i32,
    pub rotation: f64,
}

impl Beacon {
    /// Creates a new beacon.
    pub fn new(pos: MapIdx) -> Self {
        let mut shape = GenericShape::new(
            ShapeVariant::Rect {
                width: BEACON_SIZE,
                height: BEACON_SIZE,
            },
            BEACON_COLOR,
            map_idx_to_point2(pos),
        );
        shape.update(
            Point2 {
                x: BEACON_SIZE / 2.0,
                y: BEACON_SIZE / 2.0,
            },
            None,
        );
        shape.set_offset(Point2 {
            x: -BEACON_SIZE / 2.0,
            y: -BEACON_SIZE / 2.0,
        });

        let mut health_bar = GenericShape::new(
            ShapeVariant::Rect {
                width: BEACON_SIZE,
                height: HEALTH_BAR_HEIGHT,
            },
            HEALTH_COLOR,
            shape.get_position()
                + Point2 {
                    x: 0.0,
                    y: BEACON_SIZE + HEALTH_BAR_HEIGHT * 2.0,
                },
        );
        health_bar.set_offset(Point2 {
            x: -BEACON_SIZE / 2.0,
            y: -BEACON_SIZE / 2.0,
        });
        let mut damage_bar = GenericShape::new(
            ShapeVariant::Rect {
                width: 0.0,
                height: HEALTH_BAR_HEIGHT,
            },
            DAMAGE_COLOR,
            shape.get_position()
                + Point2 {
                    x: 0.0,
                    y: BEACON_SIZE + HEALTH_BAR_HEIGHT * 2.0,
                },
        );
        damage_bar.set_offset(Point2 {
            x: -BEACON_SIZE / 2.0,
            y: -BEACON_SIZE / 2.0,
        });
        Self {
            idx: pos,
            shape: shape,
            health_bar: health_bar,
            damage_bar: damage_bar,
            health: BEACON_STARTING_HEALTH,
            rotation: 0.0,
        }
    }

    pub fn damage(&mut self) {
        self.health -= 1;
        if let Some(offset) = self.shape.get_offset() {
            self.health_bar = GenericShape::new(
                ShapeVariant::Rect {
                    width: BEACON_SIZE * self.health as f64 / BEACON_STARTING_HEALTH as f64,
                    height: HEALTH_BAR_HEIGHT,
                },
                HEALTH_COLOR,
                self.shape.get_position()
                    + Point2 {
                        x: 0.0,
                        y: BEACON_SIZE + HEALTH_BAR_HEIGHT * 2.0,
                    },
            );
            self.health_bar.set_offset(offset);
            self.damage_bar = GenericShape::new(
                ShapeVariant::Rect {
                    width: BEACON_SIZE * (BEACON_STARTING_HEALTH - self.health) as f64
                        / BEACON_STARTING_HEALTH as f64,
                    height: HEALTH_BAR_HEIGHT,
                },
                DAMAGE_COLOR,
                self.shape.get_position()
                    + Point2 {
                        x: BEACON_SIZE * self.health as f64 / BEACON_STARTING_HEALTH as f64,
                        y: BEACON_SIZE + HEALTH_BAR_HEIGHT * 2.0,
                    },
            );
            self.damage_bar.set_offset(offset);
        }
    }
}

impl entity::Entity for Beacon {
    fn tick(&mut self, dt: f64) {
        let delta = Point2 { x: 0.0, y: 0.0 };
        self.shape.update(delta, Some(BEACON_ROTATION_SPEED * dt));
    }
}
