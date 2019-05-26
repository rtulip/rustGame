use crate::level::{Level, MapIdx};
use crate::misc::random::{Seed, RNG, from_seed, next_u32};
use crate::entity::player::Player;
use crate::entity::tile::Tile;
use crate::entity::beacon::Beacon;

/// GameModel 
/// 
/// A model of the games entities and controls game logic
pub struct GameModel {
    // entities: Vec<Entity>,
    pub level: Level,
    pub player: Player,
    pub beacon: Beacon,
    rng: RNG,
}

impl GameModel {
    pub fn new(seed: Seed) -> Self {
        let mut level = Level::new(seed);
        let mut rng = from_seed(seed);
        let beacon_spawn = GameModel::find_beacon_spawn(&level, &mut rng);
        let mut beacon = Beacon::new(beacon_spawn);
        let player_spawn = GameModel::find_player_spawn(&level, &beacon, &mut rng);
        let mut player = Player::new([player_spawn.0 as f64 * 20.0, player_spawn.1 as f64 * 20.0]);
        
        Self {
            level: level,
            rng: rng,
            player: player,
            beacon: beacon,
        }
    }

    /// find_player_spawn()
    /// 
    /// args:
    ///     level: &Level: A reference to the level to serach for a player 
    ///         spwan point
    ///     rng: &mut RNG: A mutable reference to a random number generator
    ///         which is used to decide which of the open spaces is to be the
    ///         spawn point
    /// 
    /// Chooses a spawn point randomly from any Tile::Floor spaces in the Level
    fn find_player_spawn(level: &Level, beacon: &Beacon, rng: &mut RNG) -> MapIdx {

        let mut spawnable_spaces: Vec<MapIdx> = Vec::new();

        for h in beacon.position.1-10..beacon.position.1+11 {
            for w in beacon.position.0-10..beacon.position.0+11 {
                match level.map.get(&(w,h)) {
                    Some(Tile::Floor) => {
                        spawnable_spaces.push((w,h));
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

    fn find_beacon_spawn(level: &Level, rng: &mut RNG) -> MapIdx {
        
        let mut open_idx = (0,0);
        let mut open_count = 0;

        for h in level.height/4..level.height*3/4 {
            for w in level.width/4..level.width*3/4 {
                let mut count = 0;
                for y in h-2..h+2 {
                    for x in w-2..w+2{
                        match level.map.get(&(x,y)) {
                            Some(Tile::Floor) => count += 1,
                            _ => (),
                        }
                    }
                }
                if count > open_count{
                    open_idx = (w,h);
                    open_count = count;
                }
            }
        }

        open_idx
    } 

}