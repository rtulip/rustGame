pub use crate::level::{Level, MapIdx};
use crate::misc::random::{Seed, RNG, from_seed, next_u32};
use crate::misc::point2d::Point2;
use crate::entity::player::Player;
use crate::entity::tile::Tile;
use crate::entity::beacon::Beacon;
use crate::entity::enemy::Enemy;

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
/// ## Enemies
/// 
/// A random Tile::Floor is chosen for enemy spawn. 
/// 
/// If no spawnable space is found for the Enemy, the program panics.
/// 
/// # Spawning Enemies
/// 
/// The GameModel is also responsible for spawning enmies. First the location
/// is found for where the enemy should be placed. Then, if a path can be 
/// created from the Tile to the Beacon, the new enemy is added to the enemy 
/// list. Otherwise, no enemy is added. See GameView's map_idx_to_point2 
/// function.
pub struct GameModel {
    pub level: Level,
    pub player: Player,
    pub beacon: Beacon,
    pub enemies: Vec<Enemy>,
    rng: RNG,
}

impl GameModel {
    
    /// Creates a new GameModel. idx_to_point is a function pointer which
    /// will convert a MapIdx into a Point2. This is required for creating the
    /// Player position since find_player_spawn() returns a MapIdx instead of a
    /// Point2. 
    pub fn new(seed: Seed, idx_to_point: fn(MapIdx) -> Point2) -> Self {
        let level = Level::new(seed);
        let mut rng = from_seed(seed);
        let beacon_spawn = GameModel::find_beacon_spawn(&level, &mut rng);
        let beacon = Beacon::new(beacon_spawn);
        let player_spawn = GameModel::find_player_spawn(&level, &beacon, &mut rng);
        let player = Player::new( idx_to_point(player_spawn));
        let enemies: Vec<Enemy> = Vec::new();
        Self {
            level: level,
            player: player,
            beacon: beacon,
            enemies: enemies,
            rng: rng
        }
    }

    /// Chooses a spawn point randomly from any Tile::Floor spaces surrounding
    /// the input Beacon.
    /// 
    /// If no spawnable space is found the program will panic.
    fn find_player_spawn(level: &Level, beacon: &Beacon, rng: &mut RNG) -> MapIdx {

        let mut spawnable_spaces: Vec<MapIdx> = Vec::new();

        for h in beacon.position.x-10..beacon.position.y+11 {
            for w in beacon.position.x-10..beacon.position.y+11 {
                match level.map.get(&MapIdx::new(w,h)) {
                    Some(Tile::Floor) => {
                        spawnable_spaces.push(MapIdx::new(w,h));
                    },
                    _ => (),
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
                        match level.map.get(&MapIdx::new(x,y)) {
                            Some(Tile::Floor) => count += 1,
                            Some(Tile::Wall) => count -= 1,
                            _ => (),
                        }
                    }
                }
                if count > threshold {
                    match [
                        level.map.get(&MapIdx::new(w-1,h-1)),
                        level.map.get(&MapIdx::new(w-1,h)),
                        level.map.get(&MapIdx::new(w,h-1)),
                        level.map.get(&MapIdx::new(w,h)),
                    ] {
                        [Some(Tile::Floor),Some(Tile::Floor),Some(Tile::Floor),Some(Tile::Floor)] => {
                            spawnable_spaces.push(MapIdx::new(w,h));
                        },
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

    /// Chooses a spawn point randomly from any Tile::Floor spaces in the Level
    /// and if no spawnable space exists, the program panics.
    fn find_enemy_spawn(level: &Level, rng: &mut RNG) -> MapIdx {

        let mut spawnable_spaces: Vec<MapIdx> = Vec::new();

        for h in 0..level.height {
            for w in 0..level.width {
                match level.map.get(&MapIdx::new(w,h)) {
                    Some(Tile::Floor) => {
                        spawnable_spaces.push(MapIdx::new(w,h));
                    },
                    _ => (),
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

    pub fn create_spawner(&mut self) {
        let mut canditate_spaces: Vec<MapIdx> = Vec::new();
        for h in 0..self.level.height {
            for w in 0..self.level.width {
                
                // Check surrounding neighbours
                let pos = MapIdx::new(w,h);
                match self.level.map.get(&pos){
                    // If Tile at Pos is a wall, see if there is a floor surrounding it
                    Some(Tile::Wall) => {
                        
                        for idx in pos.neighbours() {
                            match self.level.map.get(&idx) {
                                Some(Tile::Floor) => {
                                    canditate_spaces.push(pos);
                                    break;
                                },
                                Some(Tile::Spawner) => {
                                    canditate_spaces.push(pos);
                                    break;
                                },
                                _ => (),
                            }
                        }

                    },
                    _ => (),
                }
            }
        }

        // randomly choose a candidate space
        if canditate_spaces.len() > 0 {
            let idx = next_u32(&mut self.rng) as usize % canditate_spaces.len();
            let pos = canditate_spaces[idx];
            self.level.map.remove(&pos);
            self.level.map.insert(pos, Tile::Spawner);

        }

    }

    /// Creates a new enemy if a path can be found from the randomly generated
    /// spawn point to the Beacon. 
    /// 
    /// Requires a function to convert a MapIdx to a Point2. See GameView's 
    /// map_idx_to_point2 function.
    pub fn spawn_enemy(&mut self, idx_to_point: fn(MapIdx) -> Point2) {
        
        let spawn = GameModel::find_enemy_spawn(&self.level, &mut self.rng);
        let target = &self.beacon.position;
        let mut enemy = Enemy::new(idx_to_point(spawn));
        
        if let Some(path) = self.level.pathfind(&spawn, target) {
            let mut enemy_path: Vec<Point2> = Vec::new();
            for idx in path.0 {
                enemy_path.push(idx_to_point(idx));
            }
            enemy.path = enemy_path;
            self.enemies.push(enemy);
        }

    }

}