use crate::levels::{Level, MapIdx};
use crate::traits::state::State;
use crate::traits::entity::Entity;
use crate::misc::random::{Seed, RNG, from_seed, next_u32};
use crate::misc::point2d::Point2;
use crate::entity::player::Player;
use crate::entity::tile::{Tile, TileVariant};
use crate::entity::beacon::Beacon;
use crate::entity::enemy::Enemy;
use crate::entity::drops::Resource;
use crate::entity::towers::tower::{Tower, TowerState};
use crate::game::consts::{
    map_idx_to_point2,
    PI,
    INF,
    TILE_SIZE,
};

/// A structure to fully encapsulate all components of the game. The different
/// components include a Level, a Player, a Beacon and a collection of enemies.
/// A random number generator is part of the structure to allow for randomly 
/// choosing spawn points
/// 
/// # Entity Spawn Points 
/// 
/// The GameModel is also responsible for finding the spawnpoints for each 
/// entity indlucing the Beacon, the Player, and Enemies. 
/// 
/// ## Beacon
/// 
/// To find the spawn point of the Beacon, for each Tile::Floor in the Level,
/// the ratio of Floors to Walls surrounding the point is calculated. This 
/// ratio is used as a way to measure how open the surrounding area is. Only 
/// Tiles which are above a threshold are considered for spawning. Once all the
/// candidate spaces are found, one is chosen at random. 
/// 
/// If no spawnable space is found for the Beacon, the program panics.
/// 
/// ## Player
/// 
/// The spawn point of the player depends on the location of the Beacon. Each
/// Tile::Floor in an area surrounding the Beacon is a candidate spawning 
/// space. Once all candidate spaces have been found, one is chosen at random.
/// 
/// If no spawnable space is found for the Player, the program panics.  
/// 
/// ## Spawners
/// 
/// Any Tile::Wall with at least one Tile::Floor or Tile::Spawner to the north,
/// east, south or west will be considered a candidate space. 
/// 
/// If there are no candidate spaces found for the Enemy nothing happens.
/// 
/// # Spawning Enemies
/// 
/// The GameModel is also responsible for spawning enmies. For each spawning
/// Tile in the Map, there is a constant chance of having an enemy spawn at 
/// that location.
pub struct GameModel {
    pub level: Level,
    pub player: Player,
    pub beacon: Beacon,
    pub enemies: Vec<Enemy>,
    pub max_enemies: usize,
    pub spawners: Vec<MapIdx>,
    pub resources: Vec<Resource>,
    pub towers: Vec<Tower>,
    rng: RNG,
}

impl GameModel {
    
    /// Creates a new GameModel. idx_to_point is a function pointer which
    /// will convert a MapIdx into a Point2. This is required for creating the
    /// Player position since find_player_spawn() returns a MapIdx instead of a
    /// Point2. 
    pub fn new(seed: Seed) -> Self {
        let level = Level::new(seed);
        let mut rng = from_seed(seed);
        let beacon_spawn = GameModel::find_beacon_spawn(&level, &mut rng);
        let beacon = Beacon::new(beacon_spawn);
        let player_spawn = GameModel::find_player_spawn(&level, &beacon, &mut rng);
        let player = Player::new( map_idx_to_point2(player_spawn));
        let enemies: Vec<Enemy> = Vec::new();
        let spawners: Vec<MapIdx> = Vec::new();
        let resources: Vec<Resource> = Vec::new();
        let towers: Vec<Tower> = Vec::new();
        Self {
            level: level,
            player: player,
            beacon: beacon,
            enemies: enemies,
            max_enemies: 15,
            spawners: spawners,
            resources: resources,
            towers: towers,
            rng: rng
        }
    }

    /// Chooses a spawn point randomly from any Tile::Floor spaces surrounding
    /// the input Beacon.
    /// 
    /// If no spawnable space is found the program will panic.
    fn find_player_spawn(level: &Level, beacon: &Beacon, rng: &mut RNG) -> MapIdx {

        let mut spawnable_spaces: Vec<MapIdx> = Vec::new();
        for h in beacon.idx.x-10..beacon.idx.y+11 {
            for w in beacon.idx.x-10..beacon.idx.y+11 {
                if let Some(tile) = level.map.get(&MapIdx::new(w,h)) {
                    match tile.variant {
                        TileVariant::Floor => spawnable_spaces.push(MapIdx::new(w,h)),
                        _ => (),

                    }
                }
            }
        }

        if spawnable_spaces.len() == 0 {
            panic!("No spawnable spaces!");
        }

        let idx = next_u32(rng) as usize % spawnable_spaces.len();
        let idx = spawnable_spaces.remove(idx);
        idx

    }

    /// Finds an open space to spawn the beacon. To be sufficiently open there 
    /// must be at least threshold more Floors than Walls in a surrounding area
    /// 
    /// If no spawnable spaces are found, the program panics.
    fn find_beacon_spawn(level: &Level, rng: &mut RNG) -> MapIdx {
        
        let mut spawnable_spaces: Vec<MapIdx> = Vec::new();
        let threshold = 30;

        for h in level.height/4..level.height*3/4 {
            for w in level.width/4..level.width*3/4 {
                let mut count = 0;
                for y in h-3..h+3 {
                    for x in w-3..w+3{
                        if let Some(tile) = level.map.get(&MapIdx::new(x,y)) {
                            match tile.variant {
                                TileVariant::Floor => count += 1,
                                TileVariant::Wall => count -= 1,
                                _ => (),
                            }
                        }
                    }
                }
                if count > threshold {
                    spawnable_spaces.push(MapIdx::new(w,h));
                }
                
            }
        }

        if spawnable_spaces.len() == 0 {
            panic!("No spawnable spaces!");
        }

        let idx = next_u32(rng) as usize % spawnable_spaces.len();
        let idx = spawnable_spaces.remove(idx);
        idx

    } 

    /// Has a chance of creating a new spawner
    pub fn chanced_create_spawner(&mut self, chance: u32) {

        let rand = next_u32(&mut self.rng);
        if rand % chance == 0 {
            self.create_spawner();
        }

    } 

    /// Creates a new spawner in a random location with a Floor or Spawner to  
    /// the north, east, south or west. 
    pub fn create_spawner(&mut self) {
           
        let mut canditate_spaces: Vec<MapIdx> = Vec::new();
        for h in 0..self.level.height {
            for w in 0..self.level.width {
                
                // Check surrounding neighbours
                let pos = MapIdx::new(w,h);
                if let Some(tile) = self.level.map.get(&pos){
                    match tile.variant {
                        TileVariant::Wall => {
                            
                            for idx in pos.neighbours() {
                                
                                if let Some(n_tile) = self.level.map.get(&idx) {
                                    match n_tile.variant {
                                        TileVariant::Floor => {
                                            canditate_spaces.push(pos);
                                            break;
                                        },
                                        TileVariant::Spawner => {
                                            canditate_spaces.push(pos);
                                            break;          
                                        },
                                        _ => (),
                                    }
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
        }

        // randomly choose a candidate space
        if canditate_spaces.len() > 0 {
            let idx = next_u32(&mut self.rng) as usize % canditate_spaces.len();
            let pos = canditate_spaces[idx];
            self.level.map.remove(&pos);
            self.level.map.insert(pos, Tile::new(TileVariant::Spawner, pos));
            self.spawners.push(pos);
        }

    }

    /// Creates a new enemy if a path can be found from the randomly generated
    /// spawn point to the Beacon. 
    /// 
    /// Requires a function to convert a MapIdx to a Point2. See GameView's 
    /// map_idx_to_point2 function.
    pub fn spawn_enemies(&mut self) {
        
        for spawner in self.spawners.iter() {
            let r = next_u32(&mut self.rng);
            if r % 50 == 0 && self.enemies.len() < self.max_enemies {
                let target = &self.beacon.idx;
                let mut enemy = Enemy::new(map_idx_to_point2(*spawner));
                
                if let Some(path) = self.level.pathfind(&spawner, target) {
                    let mut enemy_path: Vec<Point2> = Vec::new();
                    for idx in path.0 {
                        enemy_path.push(map_idx_to_point2(idx));
                    }
                    enemy.path = enemy_path;
                    self.enemies.push(enemy);
                }
            }
        }
    }

    /// Function to spawn a new resource at the location of the Enemy which was
    /// killed. There is a roughly 33% chance of spawning a resource. 
    pub fn spawn_resource(&mut self, enemy: &Enemy) {

        let r = next_u32(&mut self.rng);
        if r % 3 == 0 {
            self.resources.push(Resource::new(enemy.shape.center_point()));
        }

    }

    /// Creates a tower at the player's position and consumes a resource.
    pub fn create_tower(&mut self) {

        if self.player.resources != 0 {
            self.player.resources -= 1;
            self.towers.push(Tower::new(self.player.shape.get_position()));
        }

    }

    /// Updates each tower in the tower list. If any enemies are close enough,
    /// visible, and are within tower range the towers switch to Attacking, (if
    /// not already attacking).
    pub fn tick_towers(&mut self, dt: f64){

        for tower in self.towers.iter_mut() {
            let mut new_dir = Point2{x: 0.0, y: 0.0};
            let mut min_dist = INF;
            for enemy in self.enemies.iter() {

                let dir = enemy.shape.center_point() - tower.base_shape.center_point();
                let slope = dir.y / dir.x;
                let vertical_offset = tower.base_shape.center_point().y;
                let x0 = (tower.base_shape.center_point().x / TILE_SIZE).floor() as i32;
                let xn = (enemy.shape.center_point().x / TILE_SIZE).floor() as i32;

                let mut wall_hit = false;
                let previous = tower.base_shape.get_position();
                let mut previous = MapIdx::new((previous.x / TILE_SIZE).floor() as i32, (previous.y / TILE_SIZE).floor() as i32);
                for x in x0+1..xn {
                    let y = ((slope * x as f64 + vertical_offset)/ TILE_SIZE).floor() as i32;
                    if y != previous.y {
                        if let Some(tile) = self.level.map.get(&MapIdx::new(previous.x, y)) {
                            match tile.variant {
                                TileVariant::Wall => {
                                    wall_hit = true;
                                    continue;
                                },
                                _ => (),
                            }
                        }
                    }
                    
                    if let Some(tile) = self.level.map.get(&MapIdx::new(x, y)) {
                        match tile.variant {
                            TileVariant::Wall => {
                                wall_hit = true;
                                continue;
                            },
                            _ => (),
                        }
                    } 
                    previous = MapIdx::new(x, y);
                }
                if wall_hit {
                    continue;
                }

                let dist = dir.x.abs() + dir.y.abs();
                if dist < min_dist {
                    min_dist = dist;
                    new_dir = dir; 
                }

            }

            if min_dist < tower.range {
                
                let mut rad = new_dir.y / new_dir.x;
                rad = rad.atan();
                
                match [new_dir.x < 0.0, new_dir.y < 0.0] {
                    [true, true] => rad = PI * 2.0 - rad,
                    [true, false] => rad = rad * -1.0,
                    [false, true] => rad = PI + rad * -1.0,
                    [false, false] => rad = PI - rad
                }

                rad = PI - rad;
                tower.set_rotation(rad);

                match tower.state {
                    TowerState::Ready => tower.change_state(TowerState::Attacking),
                    _ => (),
                }
            }

            tower.tick(dt);

        }

    }

}