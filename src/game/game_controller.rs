use crate::game::{GameModel, GameView, AnimationEnum};
use crate::misc::random::Seed;
use crate::misc::point2d::Point2;
use crate::misc::vector2d::Vec2;
use crate::traits::entity::Entity;
use crate::traits::state::State;
use crate::entity::{player, tile, enemy};
use crate::level::MapIdx;

use std::collections::HashSet;

use piston::input::{GenericEvent, Button, Key};

pub enum GameState {
    Running,
    Finished,
}

/// GameController
/// 
/// A struct to control the game processes including user input, graphics and
/// game ticks
pub struct GameController {
    pub model: GameModel,
    pub view: GameView,
    pub state: GameState,
    cursor_pos: Point2,
    keys_pressed: HashSet<Key>,
}

impl GameController {
    pub fn new(seed: Seed) -> Self {
        
        let view = GameView::new();
        let mut model = GameModel::new(seed, GameView::map_idx_to_point2);
        let cursor_pos = Point2 {x: 0.0, y: 0.0};
        let keys_pressed = HashSet::new();

        model.spawn_enemy(GameView::map_idx_to_point2);
        model.spawn_enemy(GameView::map_idx_to_point2);
        model.spawn_enemy(GameView::map_idx_to_point2);
        
        Self {model: model, view: view, state: GameState::Running, cursor_pos: cursor_pos, keys_pressed: keys_pressed}

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
            self.cursor_pos = Point2 {x: pos[0], y: pos[1]};
            self.model.player.update_direction(&self.cursor_pos, self.view.settings.player_size);
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            self.keys_pressed.insert(key);
        }
        if let Some(Button::Keyboard(key)) = e.release_args() {
            if self.keys_pressed.contains(&key) {
                self.keys_pressed.remove(&key);
                match key {
                    Key::Space => {
                        self.view.settings.player_attack_animation.change_state(AnimationEnum::Finished);
                        self.model.player.change_state(player::PlayerState::FinishedAttacking);
                    },
                    _ => (),
                }
            }
        }
    }

    /// tick()
    /// 
    /// Executes a single game tick
    pub fn tick(&mut self) {
        if self.keys_pressed.contains(&Key::W) {
            self.model.player.change_state(player::PlayerState::Moving);
        } else {
            self.model.player.change_state(player::PlayerState::Stationary);
        }

        if self.keys_pressed.contains(&Key::Space) {
            self.view.settings.player_attack_animation.change_state(AnimationEnum::Active);
            self.model.player.change_state(player::PlayerState::Attacking);
        }

        self.model.player.tick();
        self.check_player_collision();
        self.model.beacon.tick();
        self.tick_enemies();
        
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

        let min_x = ( self.model.player.position.x / tile_size).floor() as i32;
        let max_x = ((self.model.player.position.x + player_size) / tile_size).floor() as i32 + 1;

        let min_y = ( self.model.player.position.y / tile_size).floor() as i32;
        let max_y = ((self.model.player.position.y + player_size) / tile_size).floor() as i32 + 1;
        
        for h in min_y..max_y {
            for w in min_x..max_x {
                match self.model.level.map.get(&MapIdx::new(w,h)) {
                    Some(tile::Tile::Wall) => {
                        let tile_pos = GameView::map_idx_to_point2(MapIdx::new(w, h));
                        let shift_left = tile_pos.x - self.model.player.position.x - player_size - 0.1;
                        let shift_right = tile_pos.x + tile_size - self.model.player.position.x + 0.1;
                        let shift_up = tile_pos.y - self.model.player.position.y - player_size - 0.1;
                        let shift_down = tile_pos.y + tile_size - self.model.player.position.y + 0.1;
    
                        let moves = [shift_left, shift_right, shift_up, shift_down];
                        let mut min_move = moves[0];

                        for i in 0..4 {
                            if moves[i].abs() < min_move.abs() {
                                min_move = moves[i];
                            }
                        }

                        if min_move == shift_left || min_move == shift_right {
                            self.model.player.position.x += min_move;
                        } else {
                            self.model.player.position.y += min_move;
                        }

                    },
                    _ => {}
                }
            }
        }
    }

    fn tick_enemies(&mut self) {
        let mut to_remove: Vec<usize> = Vec::new();
        for (i, enemy) in self.model.enemies.iter_mut().enumerate().rev() {
            enemy.tick();
            
            let beacon_point = GameView::map_idx_to_point2(self.model.beacon.position);
            if (beacon_point.x < enemy.position.x + self.view.settings.enemy_size && 
                enemy.position.x < beacon_point.x + self.view.settings.beacon_size) || 
               (beacon_point.y > enemy.position.y + self.view.settings.enemy_size && 
                enemy.position.y > beacon_point.y + self.view.settings.beacon_size) {
                to_remove.push(i);
                self.model.beacon.health -= 1;
                
            } 

            if (self.model.player.position.x < enemy.position.x + self.view.settings.enemy_size && 
                enemy.position.x < self.model.player.position.x + self.view.settings.beacon_size) || 
               (self.model.player.position.y > enemy.position.y + self.view.settings.enemy_size && 
                enemy.position.y > self.model.player.position.y + self.view.settings.beacon_size) {
                to_remove.push(i);
                self.model.player.health -= 1;
            }

        }

        for i in to_remove {
            self.model.enemies.remove(i);
            
        }

        if self.model.beacon.health == 0 || self.model.player.health == 0{
            self.change_state(GameState::Finished);
        }

    }

}

impl State for GameController {
    type StateEnum = GameState;
    fn change_state(&mut self, new_state: Self::StateEnum) {
        self.state = new_state;
    }
}