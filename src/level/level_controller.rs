use crate::level::{Level, Map};
use crate::misc::random;

pub struct LevelController {
    level: Level,
}

impl LevelController {

    pub fn new(level: Level) -> Self {
        Self {level: level}
    } 

    pub fn get_map(&self) -> &Map {
        self.level.get_map()
    }

    pub fn get_width(&self) -> i32 {
        self.level.get_width()
    }

    pub fn get_height(&self) -> i32 {
        self.level.get_height()
    }

    pub fn get_rng(&self) -> &random::RNG {
        self.level.get_rng()
    }

    pub fn print_level(&self) {
        self.level.print_level();
    }
    // pub fn event<E: GenericEvent>(&mut self, e: &E) {
    //     TODO
    // } 

}