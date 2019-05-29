use crate::game::{GameModel, GameView, AnimationEnum};
use crate::misc::random::Seed;
use crate::misc::point2d::Point2;
use crate::traits::entity::Entity;
use crate::traits::state::State;
use crate::entity::player;
use crate::entity::tile::{Tile, TileVariant};
use crate::level::MapIdx;

use std::collections::HashSet;
use std::f64::consts::PI;

use piston::input::{GenericEvent, Button, Key};

/// An enumeration describeing the different states for the Game. Running 
/// implies that both the GameModel Player and the GameModel Beacon have
/// health remaining. Finished implies that either the Beacon or the Player
/// have run out of health, and so the game is over.
pub enum GameState {
    Running,
    Finished,
}

/// A struct to control the game processes including user input, graphics and
/// game ticks.
/// 
/// # Input Handling
/// The position of the cursor is updated every time it is updated. 
/// Every key pressed is tracked in the keys_pressed list, and whenever a 
/// key is released, the key is removed from the key pressed list. 
/// 
/// ## Cursor Position
/// This input is used to control which direction the Player is facing.
/// 
/// ## W Key
/// This input is used to start and stop player movement. So long as this 
/// key is pressed, and the Space Bar isn't pressed, the player will move
/// towards the direction it's facing (towards the mouse). 
/// 
/// ## Space Bar
/// This input is used to control the player attack animation. Pressing 
/// space will start the animation. This will prevent all player movement 
/// until the space bar is released. The animation will draw a box in the
/// center of the Player towards the mouse. 
pub struct GameController {
    pub model: GameModel,
    pub view: GameView,
    pub state: GameState,
    cursor_pos: Point2,
    keys_pressed: HashSet<Key>,
}

impl GameController {
    
    /// Creates a new GameController. The GameModel will start with 2 spawning
    /// spaces created. 
    pub fn new(seed: Seed) -> Self {
        
        let view = GameView::new();
        let mut model = GameModel::new(seed, GameView::map_idx_to_point2);
        let cursor_pos = Point2 {x: 0.0, y: 0.0};
        let keys_pressed = HashSet::new();

        model.create_spawner();
        model.create_spawner();

        Self {model: model, view: view, state: GameState::Running, cursor_pos: cursor_pos, keys_pressed: keys_pressed}

    }

    /// Parses the event for cursor position, Keyboard presses and keyboard
    /// relseases. 
    pub fn handle_event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = Point2 {x: pos[0], y: pos[1]};
        }
        self.model.player.update_direction(&self.cursor_pos);
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

    /// Executes a single game tick 
    pub fn tick(&mut self) {
        // Update Movement state if W is pressed
        if self.keys_pressed.contains(&Key::W) {
            self.model.player.change_state(player::PlayerState::Moving);
        } else {
            self.model.player.change_state(player::PlayerState::Stationary);
        }
        // Start animation if Space is pressed
        if self.keys_pressed.contains(&Key::Space) {
            self.view.settings.player_attack_animation.change_state(AnimationEnum::Active);
            self.model.player.change_state(player::PlayerState::Attacking);
        }

        // Tick player
        self.model.player.tick();
        // Check for collision
        self.check_player_collision();
        // Tick Beacon
        self.model.beacon.tick();
        self.tick_resources();
        // Tick enemies and check for collision.
        self.tick_enemies();

        // Chreate spawner with constant chance
        self.model.chanced_create_spawner(5000);
        // Spawn enmies from spawners
        self.model.spawn_enemies(GameView::map_idx_to_point2);
        
    }

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

        
        let min_x = ( self.model.player.shape.get_position().x / tile_size).floor() as i32;
        let max_x = ((self.model.player.shape.get_position().x + player_size) / tile_size).floor() as i32 + 1;

        let min_y = ( self.model.player.shape.get_position().y / tile_size).floor() as i32;
        let max_y = ((self.model.player.shape.get_position().y + player_size) / tile_size).floor() as i32 + 1;
        
        for h in min_y..max_y {
            for w in min_x..max_x {
                if let Some(tile) = self.model.level.map.get(&MapIdx::new(w,h)) {
                    match tile.variant {
                        TileVariant::Wall => {
                            let tile_pos = GameView::map_idx_to_point2(MapIdx::new(w, h));
                            let shift_left = tile_pos.x - self.model.player.shape.get_position().x - player_size - 0.1;
                            let shift_right = tile_pos.x + tile_size - self.model.player.shape.get_position().x + 0.1;
                            let shift_up = tile_pos.y - self.model.player.shape.get_position().y - player_size - 0.1;
                            let shift_down = tile_pos.y + tile_size - self.model.player.shape.get_position().y + 0.1;
        
                            let moves = [shift_left, shift_right, shift_up, shift_down];
                            let mut min_move = moves[0];

                            for i in 0..4 {
                                if moves[i].abs() < min_move.abs() {
                                    min_move = moves[i];
                                }
                            }

                            if min_move == shift_left || min_move == shift_right {
                                let delta = Point2{x: min_move, y: 0.0};
                                self.model.player.shape.update(delta, None);
                            } else {
                                let delta = Point2{x: 0.0, y: min_move};
                                self.model.player.shape.update(delta, None);
                            }
                        }
                        _ => (),
                    }
                }
            }
        }

        let player_center = self.model.player.shape.center_point();
        let mut to_remove: Vec<usize> = Vec::new();
        for (i,resource) in self.model.resources.iter_mut().enumerate().rev() {
            let resource_center = resource.shape.center_point();
            if (resource_center.x - player_center.x).abs() + (resource_center.y - player_center.y).abs() <= self.view.settings.player_radius {
                to_remove.push(i);
                if self.model.player.resources < 9 {
                    self.model.player.resources += 1;
                }
            }
        }

        for i in to_remove {
            self.model.resources.remove(i);
        }

    }

    /// Moves each enemy in the direction of its path. Then the position of 
    /// each enemy is compared against the position of the Beacon and the 
    /// Player. If there is a collision with either, the appropriate entity
    /// takes damage, and the enemy is destroyed. If the Player and the Beacon
    /// are both colliding with the enemy within the same game tick, they both
    /// take damage.
    /// 
    /// For checking collisions with the beacon, the center point of the Beacon
    /// must within the Enemy's radius.
    /// 
    /// For checking collisions with the Player, the Player and the Enemy must
    /// overlap. 
    fn tick_enemies(&mut self) {
        let mut to_remove: Vec<(usize, bool)> = Vec::new();
        // Loop through enemies
        for (i, enemy) in self.model.enemies.iter_mut().enumerate().rev() {
            // move enemy
            enemy.tick();
            
            // check for collision with beacon. 
            let beacon_center = self.model.beacon.shape.center_point();
            let enemy_center = enemy.shape.center_point();
            if (beacon_center.x - enemy_center.x).abs() + (beacon_center.y - enemy_center.y).abs() <= self.view.settings.enemy_radius {
                to_remove.push((i,false));
                self.model.beacon.health -= 1;
            } 
            // check for collision with the player.
            let player_center = self.model.player.shape.center_point();
            if (player_center.x - enemy_center.x).abs() + (player_center.y - enemy_center.y).abs() <= self.view.settings.enemy_radius + self.view.settings.player_radius {
                to_remove.push((i,false));
                self.model.player.health -= 1;
            }

            match self.model.player.state {
                player::PlayerState::Attacking => {
                    let p1 = Point2 {
                        x:  self.view.settings.player_attack_animation.animation_position.x + 
                            self.view.settings.player_attack_animation.animation_width * 
                            self.view.settings.player_attack_animation.animation_rotation.cos(),
                        y:  self.view.settings.player_attack_animation.animation_position.y - 
                            self.view.settings.player_attack_animation.animation_width * 
                            self.view.settings.player_attack_animation.animation_rotation.sin(),
                    };
                    let p2 = Point2 {
                        x:  p1.x + 
                            self.view.settings.player_attack_animation.animation_height *
                            (PI / 2.0 - self.view.settings.player_attack_animation.animation_rotation).cos(),
                        y:  p1.y + 
                            self.view.settings.player_attack_animation.animation_height *
                            (PI / 2.0 - self.view.settings.player_attack_animation.animation_rotation).sin(),
                            
                    };

                    if  (p1.x - enemy_center.x).abs() + (p1.y - enemy_center.y).abs() <= self.view.settings.enemy_radius || 
                        (p2.x - enemy_center.x).abs() + (p2.y - enemy_center.y).abs() <= self.view.settings.enemy_radius {
                        to_remove.push((i,true));
                    }
                    
                },
                _ => (),
            }

        }

        // remove all enemies which had collisions
        for (i, resource) in to_remove {
            
            let enemy = self.model.enemies.remove(i);
            if resource {
                self.model.spawn_resource(&enemy);
            }
        }

        //Check Gamestate to see if GameOver.
        if self.model.beacon.health == 0 || self.model.player.health == 0{
            self.change_state(GameState::Finished);
        }

    }

    /// Ticks each resource in the GameModels resource list
    fn tick_resources(&mut self) {

        for resource in self.model.resources.iter_mut() {
            resource.tick();
        }

    }

}

// Basic state implementation for GameController.
impl State for GameController {
    type StateEnum = GameState;
    fn change_state(&mut self, new_state: Self::StateEnum) {
        self.state = new_state;
    }
}