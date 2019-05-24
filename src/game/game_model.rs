use crate::level::{Level, MapIdx};
use crate::misc::random::{Seed, RNG, from_seed, next_u32};
use crate::entity::player::Player;
use crate::entity::tile::Tile;

/// GameModel 
/// 
/// A model of the games entities and controls game logic
pub struct GameModel {
    // beacon: Beacon,
    // entities: Vec<Entity>,
    pub level: Level,
    pub player: Player,
    rng: RNG,
}

impl GameModel {
    pub fn new(seed: Seed) -> Self {
        let mut level = Level::new(seed);
        let mut rng = from_seed(seed);
        let player_spawn = GameModel::find_player_spawn(&level, &mut rng);
        let mut player = Player::new([player_spawn.0 as f64 * 20.0, player_spawn.1 as f64 * 20.0]);
        Self {
            level: level,
            rng: rng,
            player: player,
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
    fn find_player_spawn(level: &Level, rng: &mut RNG) -> MapIdx {

        let mut spawnable_spaces: Vec<MapIdx> = Vec::new();

        for h in 0..level.get_height() {
            for w in 0..level.get_width(){
                match level.get_map().get(&(w,h)) {
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

}