use crate::game::{GameModel, GameView};
use crate::misc::random::Seed;
use crate::traits::entity::Entity;

use piston::input::GenericEvent;

pub struct GameController {
    pub model: GameModel,
    pub view: GameView,
    cursor_pos: [f64;2],
}

impl GameController {
    pub fn new(seed: Seed) -> Self {
        Self {
            model: GameModel::new(seed),
            view: GameView::new(),
            cursor_pos: [0.0; 2],
        }
    }

    pub fn handle_event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
            self.model.player.update_direction(self.cursor_pos, self.view.settings.player_size);
        }
    }

    pub fn tick(&mut self) {
        self.model.player.tick();
    }

}