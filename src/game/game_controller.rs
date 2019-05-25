use crate::game::{GameModel, GameView, AnimationEnum};
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
            self.keys_pressed.contains(&Key::Space),
        ] {
            [_, true] => {
                self.model.player.change_state(player::PlayerState::Attacking);
                self.view.settings.player_attack_animation.change_state(AnimationEnum::Active);
            }
            [true, _] => {
                self.model.player.change_state(player::PlayerState::Moving);
            },
            [false, _] => {
                self.model.player.change_state(player::PlayerState::Stationary);
            },
        }
        
        self.model.player.tick();
        match self.view.tick_animation() {
            Some(_state) => {
                self.model.player.change_state(player::PlayerState::FinishedAttacking);
            },
            _ => ()
        }
        self.check_player_collision();
    }

    /// check_player_collision()
    /// 
    /// Checks the position of the player against the level walls. If the bounding
    /// box of the player overlaps with a wall, the position of the player is 
    /// corrected by the smallest move. Will take two game ticks to resolve corner
    /// collisions, as the player is only every moved in one direction at a time. 
    /// 
    /// The player's position is approximated as a square despite actually being a
    /// circle. This is only noticeable on corners. Can improve this to compare 
    /// circle's to rectangles in the future.
    fn check_player_collision(&mut self) {
        let tile_size = self.view.settings.tile_size;
        let player_size = self.view.settings.player_size;
        let player_pos = self.model.player.position;

        let min_x = ( player_pos[0] / tile_size).floor() as i32;
        let max_x = ((player_pos[0] + player_size) / tile_size).floor() as i32 + 1;

        let min_y = ( player_pos[1] / tile_size).floor() as i32;
        let max_y = ((player_pos[1] + player_size) / tile_size).floor() as i32 + 1;
        
        for h in min_y..max_y {
            for w in min_x..max_x {
                match self.model.level.map.get(&(w,h)) {
                    Some(tile::Tile::Wall) => {
                        let tile_pos = [w as f64 * tile_size, h as f64 * tile_size];
                        let shift_left = tile_pos[0] - player_pos[0] - player_size - 0.1;
                        let shift_right = tile_pos[0] + tile_size - player_pos[0] + 0.1;
                        let shift_up = tile_pos[1] - player_pos[1] - player_size - 0.1;
                        let shift_down = tile_pos[1] + tile_size - player_pos[1] + 0.1;
    
                        let moves = [shift_left, shift_right, shift_up, shift_down];
                        let mut min_move = moves[0];

                        for i in 0..4 {
                            if moves[i].abs() < min_move.abs() {
                                min_move = moves[i];
                            }
                        }

                        if min_move == shift_left || min_move == shift_right {
                            self.model.player.position[0] += min_move;
                        } else {
                            self.model.player.position[1] += min_move;
                        }

                    },
                    _ => {}
                }
            }
        }
    }

}