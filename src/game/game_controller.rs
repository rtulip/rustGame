use crate::game::GameModel;
use crate::misc::random::Seed;

use piston::input::GenericEvent;

pub struct GameController {
    model: GameModel,
}

impl GameController {
    pub fn new(seed: Seed) -> Self {
        Self {model: GameModel::new(seed) }
    }

    pub fn handle_event<E: GenericEvent>(&mut self, e: &E) {
        // TODO
    }

    pub fn tick(&self) {
        // TODO
    }

}