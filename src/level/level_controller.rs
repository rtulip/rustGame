use crate::level::{Level, Map, MapIdx};
use crate::entity::tile::Tile;
use crate::misc::random;

///LevelController
/// 
/// The controller of a Model-View-Controller design for a level.
pub struct LevelController {
    level: Level,
}

impl LevelController {
    
    pub fn new(level: Level) -> Self {
        Self {level: level}
    } 

    pub fn get_map(&self) -> &Map {
        self.level.get_map()
    }

    pub fn get_width(&self) -> i32 {
        self.level.get_width()
    }

    pub fn get_height(&self) -> i32 {
        self.level.get_height()
    }

    pub fn print_level(&self) {
        self.level.print_level();
    }

    pub fn next_u32(&mut self) -> u32 {
        self.level.next_u32()
    }

    pub fn find_player_spawn(&mut self) -> MapIdx {

        let mut spawnable_spaces: Vec<MapIdx> = Vec::new();

        for h in 0..self.get_height() {
            for w in 0..self.get_width(){
                match self.get_map().get(&(w,h)) {
                    Some(Tile::Wall) => {
                        spawnable_spaces.push((w,h));
                    },
                    _ => (),
                }
            }
        }

        if spawnable_spaces.len() == 0 {
            panic!("No spawnable spaces!");
        }

        let idx = self.next_u32() as usize % spawnable_spaces.len();
        spawnable_spaces.remove(idx)

    }
    // pub fn event<E: GenericEvent>(&mut self, e: &E) {
    //     TODO
    // } 

}