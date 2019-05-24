use crate::game::{GameModel, GameView};
use crate::misc::random::Seed;
use crate::traits::entity::Entity;
use crate::traits::state::State;
use crate::entity::player::PlayerState;
use std::collections::HashSet;

use piston::input::{GenericEvent, Button, Key};

pub struct GameController {
    pub model: GameModel,
    pub view: GameView,
    cursor_pos: [f64;2],
    keys_pressed: HashSet<Key>,
}

impl GameController {
    pub fn new(seed: Seed) -> Self {
        Self {
            model: GameModel::new(seed),
            view: GameView::new(),
            cursor_pos: [0.0; 2],
            keys_pressed: HashSet::new(),
        }
    }

    pub fn handle_event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
            self.model.player.update_direction(self.cursor_pos, self.view.settings.player_size);
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            self.keys_pressed.insert(key);
        }
        if let Some(Button::Keyboard(key)) = e.release_args() {
            if self.keys_pressed.contains(&key) {
                self.keys_pressed.remove(&key);
            }
        }
    }

    pub fn tick(&mut self) {
        match [
            self.keys_pressed.contains(&Key::W),
            self.keys_pressed.contains(&Key::S),
        ] {
            [true, true] => {
                self.model.player.backwards = false;
                self.model.player.change_state(PlayerState::Stationary);
            },
            [true, false] => {
                self.model.player.backwards = false;
                self.model.player.change_state(PlayerState::Moving);
            },
            [false, true] => {
                self.model.player.backwards = true;
                self.model.player.change_state(PlayerState::Moving);
            },
            [false, false] => {
                self.model.player.backwards = false;
                self.model.player.change_state(PlayerState::Stationary);
            }
        }
        
        self.model.player.tick();
    }

}