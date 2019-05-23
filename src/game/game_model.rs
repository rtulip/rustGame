use crate::level::Level;
use crate::misc::random::{Seed, RNG, from_seed, next_u32};
use crate::traits::Model;

pub struct GameModel {
    // beacon: Beacon,
    // entities: Vec<Entity>,
    pub level: Level,
    rng: RNG,
}

impl GameModel {
    pub fn new(seed: Seed) -> Self {
        let mut level = Level::new(seed);
        let mut rng = from_seed(seed);
        for i in 0..level.get_width() * level.get_height() + 100 {
            next_u32(&mut rng);
        }
        Self {
            level: level,
            rng: rng,
        }
    }
}