use crate::game::{GameModel, GameView};
use crate::misc::random::Seed;
use crate::traits::entity::Entity;
use crate::traits::state::State;
use crate::entity::{player, tile};

use std::collections::HashSet;

use piston::input::{GenericEvent, Button, Key};

/// GameController
/// 
/// A struct to control the game processes including user input, graphics and
/// game ticks
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

    /// handle_event()
    /// 
    /// args:
    ///     e: &GenericEvent: The generic event to be handled
    /// 
    /// Parses the event for cursor position, Keyboard presses and keyboard
    /// relseases
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

    /// tick()
    /// 
    /// Executes a single game tick
    pub fn tick(&mut self) {
        match [
            self.keys_pressed.contains(&Key::W),
            self.keys_pressed.contains(&Key::S),
        ] {
            [true, true] => {
                self.model.player.backwards = false;
                self.model.player.change_state(player::PlayerState::Stationary);
            },
            [true, false] => {
                self.model.player.backwards = false;
                self.model.player.change_state(player::PlayerState::Moving);
            },
            [false, true] => {
                self.model.player.backwards = true;
                self.model.player.change_state(player::PlayerState::Moving);
            },
            [false, false] => {
                self.model.player.backwards = false;
                self.model.player.change_state(player::PlayerState::Stationary);
            }
        }
        
        self.model.player.tick();
        self.check_player_collision();
    }

    fn check_player_collision(&mut self) {
        let tile_size = self.view.settings.tile_size;
        let player_size = self.view.settings.player_size;
        let player_pos = self.model.player.position;

        let min_x = ( player_pos[0] / tile_size).floor() as i32;
        let max_x = ((player_pos[0] + player_size) / tile_size).floor() as i32 + 1;

        let min_y = ( player_pos[1] / tile_size).floor() as i32;
        let max_y = ((player_pos[1] + player_size) / tile_size).floor() as i32 + 1;
        
        let mut found_collision = false;
        for h in min_y..max_y {
            for w in min_x..max_x {
                match self.model.level.map.get(&(w,h)) {
                    Some(tile::Tile::Wall) => {
                        let tile_pos = [w as f64 * tile_size, h as f64 * tile_size];

                        match [tile_pos[0] < player_pos[0], tile_pos[1] < player_pos[1]] {
                            [true, true] => {
                                self.model.player.position[0] += tile_pos[0] + tile_size - player_pos[0] + 0.1;
                                self.model.player.position[1] += tile_pos[1] + tile_size - player_pos[1] + 0.1;
                            },
                            [false, true] => {
                                self.model.player.position[0] += tile_pos[0] - player_pos[0] - player_size - 0.1;
                                self.model.player.position[1] += tile_pos[1] + tile_size - player_pos[1] + 0.1;
                            },
                            [true, false] => {
                                self.model.player.position[0] += tile_pos[0] + tile_size - player_pos[0] + 0.1;
                                self.model.player.position[1] += tile_pos[1] - player_pos[1] - player_size - 0.1;
                            }, 
                            [false, false] => {
                                self.model.player.position[0] += tile_pos[0] - player_pos[0] - player_size - 0.1;
                                self.model.player.position[1] += tile_pos[1] - player_pos[1] - player_size - 0.1;
                            }
                        }
                        
                        found_collision = true;
                    },
                    _ => {}
                }
            }
        }

        if found_collision {
            println!("Collision");
        } else {
            println!("No Collision");
        }
    }

}