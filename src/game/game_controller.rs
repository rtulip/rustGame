use crate::game::{GameModel, GameView};
use crate::math::random::Seed;
use crate::math::Point2;
use crate::traits::entity::Entity;
use crate::traits::state::State;
use crate::traits::draw::check_collision;
use crate::entity::player;
use crate::entity::tile::TileVariant;
use crate::entity::towers::tower::TowerState;
use crate::levels::map::MapIdx;
use crate::game::consts::{
    point2_to_map_idx,
    map_idx_to_point2,
    TILE_SIZE,
    PLAYER_SIZE,
};

use std::collections::HashSet;

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
    keys_locked: HashSet<Key>,
}

impl GameController {
    
    /// Creates a new GameController. The GameModel will start with 2 spawning
    /// spaces created. 
    pub fn new(seed: Seed) -> Self {
        
        let view = GameView::new();
        let mut model = GameModel::new(seed);
        let cursor_pos = Point2 {x: 0.0, y: 0.0};
        let keys_pressed = HashSet::new();
        let keys_locked = HashSet::new();

        model.create_spawner();
        model.create_spawner();

        Self {
            model: model, 
            view: view, 
            state: GameState::Running, 
            cursor_pos: cursor_pos, 
            keys_pressed: keys_pressed,
            keys_locked: keys_locked,
        }

    }

    /// Parses the event for cursor position, Keyboard presses and keyboard
    /// relseases. 
    pub fn handle_event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = Point2 {x: pos[0], y: pos[1]};
            return;
        }
        if let Some(args) = e.update_args() {
            self.tick(args.dt);
            return;
        } 
        if let Some(Button::Keyboard(key)) = e.press_args() {
            self.keys_pressed.insert(key);
            return;
        }
        if let Some(Button::Keyboard(key)) = e.release_args() {
            if self.keys_pressed.contains(&key) {
                self.keys_pressed.remove(&key);
                self.keys_locked.remove(&key);
                match key {
                    Key::Space => {
                        self.model.player.change_state(player::PlayerState::FinishedAttacking);
                    },
                    _ => (),
                }
            }
            return;
        }
    }

    /// Executes a single game tick 
    pub fn tick(&mut self, dt: f64) {
        self.model.player.update_direction(&self.cursor_pos);
        // Update Movement state if W is pressed
        if self.keys_pressed.contains(&Key::W) {
            self.model.player.change_state(player::PlayerState::Moving);
        } else {
            self.model.player.change_state(player::PlayerState::Stationary);
        }
        // Start animation if Space is pressed
        if self.keys_pressed.contains(&Key::Space) {
            self.model.player.change_state(player::PlayerState::Attacking);
        }

        if self.keys_pressed.contains(&Key::E) && !self.keys_locked.contains(&Key::E){
            self.model.create_tower();
            self.keys_locked.insert(Key::E);
        }
        self.model.tick_towers(dt);
        self.check_bullet_collision();
        // Tick player
        self.model.player.tick(dt);
        // Check for collision
        self.check_player_collision();
        // Tick Beacon
        self.model.beacon.tick(dt);
        self.tick_resources(dt);
        // Tick enemies and check for collision.
        self.tick_enemies(dt);

        // Chreate spawner with constant chance
        self.model.chanced_create_spawner(250);
        // Spawn enmies from spawners
        self.model.spawn_enemies();
        
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

        let min_idx = point2_to_map_idx(self.model.player.shape.get_position());
        let max_idx = point2_to_map_idx(self.model.player.shape.get_position() + Point2{x: PLAYER_SIZE, y: PLAYER_SIZE});
        
        for h in min_idx.y..max_idx.y+1 {
            for w in min_idx.x..max_idx.x+1 {
                if let Some(tile) = self.model.level.map.get(&MapIdx::new(w,h)) {
                    match tile.variant {
                        TileVariant::Wall => {
                            let tile_pos = map_idx_to_point2(MapIdx::new(w, h));
                            let shift_left = tile_pos.x - self.model.player.shape.get_position().x - PLAYER_SIZE - 0.1;
                            let shift_right = tile_pos.x + TILE_SIZE - self.model.player.shape.get_position().x + 0.1;
                            let shift_up = tile_pos.y - self.model.player.shape.get_position().y - PLAYER_SIZE - 0.1;
                            let shift_down = tile_pos.y + TILE_SIZE - self.model.player.shape.get_position().y + 0.1;
        
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
                                self.model.player.health_bar.update(delta, None);
                                self.model.player.damage_bar.update(delta, None);
                            } else {
                                let delta = Point2{x: 0.0, y: min_move};
                                self.model.player.shape.update(delta, None);
                                self.model.player.health_bar.update(delta, None);
                                self.model.player.damage_bar.update(delta, None);
                            }
                        }
                        _ => (),
                    }
                }
            }
        }

        let mut to_remove: Vec<usize> = Vec::new();
        for (i,resource) in self.model.resources.iter_mut().enumerate().rev() {
            
            if check_collision(resource.shape, self.model.player.shape) {
                to_remove.push(i);
                self.model.player.resources += 1;
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
    fn tick_enemies(&mut self, dt: f64) {
        let mut to_remove: Vec<(usize, bool)> = Vec::new();
        // Loop through enemies
        for (i, enemy) in self.model.enemies.iter_mut().enumerate().rev() {
            // move enemy
            enemy.tick(dt);
            
            if check_collision(self.model.beacon.shape, enemy.shape) {
                to_remove.push((i,false));
                self.model.beacon.damage();
            }
            if check_collision(self.model.player.shape, enemy.shape) {
                to_remove.push((i,false));
                self.model.player.damage();
            }

            match self.model.player.state {
                player::PlayerState::Attacking => {
                    
                    if check_collision(self.model.player.attack.shape, enemy.shape) {
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
            println!("Game Over!");
            println!("Player Health: {}, Beacon Health: {}",self.model.player.health, self.model.beacon.health);
            self.change_state(GameState::Finished);
        }

    }

    /// Ticks each resource in the GameModels resource list
    fn tick_resources(&mut self, dt: f64) {

        for resource in self.model.resources.iter_mut() {
            resource.tick(dt);
        }

    }

    /// Checks the position of every Tower's bullet (if attacking) and checks
    /// it against all Enemies and the Tile it's currently touching.
    fn check_bullet_collision(&mut self) {
        for tower in self.model.towers.iter_mut() {
            let mut to_remove: Vec<usize> = Vec::new();
            match tower.state {
                TowerState::Attacking => {
                    
                    if let Some(tile) = self.model.level.map.get(&point2_to_map_idx(tower.bullet.shape.center_point())) {
                        match tile.variant {
                            TileVariant::Wall => {
                                tower.change_state(TowerState::Ready);
                                continue;
                            },
                            _ => (),
                        }
                    } else {
                        tower.change_state(TowerState::Ready);
                        continue;
                    }
                    
                    for (i,enemy) in self.model.enemies.iter().enumerate().rev() {
                        if check_collision(tower.bullet.shape, enemy.shape){
                            to_remove.push(i);
                               tower.change_state(TowerState::Ready);
                        }
                    }

                    for i in to_remove {
                        self.model.enemies.remove(i);
                    }

                },
                _ => (),
            }

        }
    }

    /// Function to check the state of the GameController. Used to keep the 
    /// game loop running. Returns true while in the Running state, otherwise
    /// returns false if in the Finished State.
    pub fn check_state(&self) -> bool {

        match self.state {
            GameState::Finished => false,
            GameState::Running => true,
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