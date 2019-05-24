use crate::game::GameModel;
use crate::misc::random::Seed;
use crate::traits::entity::Entity;

use piston::input::GenericEvent;

pub struct GameController {
    pub model: GameModel,
}

impl GameController {
    pub fn new(seed: Seed) -> Self {
        Self {model: GameModel::new(seed) }
    }

    pub fn handle_event<E: GenericEvent>(&mut self, e: &E) {
        // TODO
    }

    pub fn tick(&mut self) {
        self.model.player.tick();
    }

}