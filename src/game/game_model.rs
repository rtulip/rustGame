use crate::level::Level;
use crate::misc::random::{Seed, RNG, from_seed}
pub struct GameModel {
    // beacon: Beacon,
    // entities: Vec<Entity>,
    level: Level,
    rng: RNG,
}

impl GameModel {
    pub fn new(seed: Seed) -> Self {
        let mut level = Level::new(seed);
        let mut rng = from_seed(seed);
        
        Self {
            level: level,
            rng: rng,
        }
    }
}