extern crate pathfinding;
use crate::entity::tile::{Tile, TileVariant};
use crate::math::random::{Seed,RNG,from_seed, next_u32};
use crate::levels::map::{Map, MapIdx};
use std::collections::HashMap;
use crate::game::consts::{
    LEVEL_WIDTH,
    LEVEL_HEIGHT,
    LEVEL_GEN_ITERS,
};

/// A structure to fully describe the game board. A Map is used to store the 
/// Tiles representing the game board. Width and height are provided for easy
/// traversal. A random number generator is included to make random selections
/// for spawning entities. 
/// 
/// Random generation is done using a seeded random number generator. To allow
/// for fixed seeds (for testing), a Seed must be provided.
/// 
/// # Level Generation
/// Level generation is done using Conway's Game of Life. Initially roughly 50%
/// of all spaces are created as a Tile::Wall (chosen randomly). Then a number
/// of iterations of Conway's Game of Life are run to create natural looking 
/// level for the player to navigate in. 
/// 
/// Rosetta code was referenced for the [Game of Life implementation](http://rosettacode.org/wiki/Conway%27s_Game_of_Life#Rust).
/// 
/// After the Game of Life iterations are complete, the remaining spaces are
/// filled with Tile::Floor filling out the board. 
/// 
/// The next step in level generation is to ensure there is only one area which
/// can be navigated. This step might not be strictly necessary, but it ensures
/// that the beacon, the player, and all enemies are reachable from each other.
/// 
/// To acomplish this a flood fill algorithm is used to find all connected 
/// components of the Map. Then all but the largest connected component is 
/// filled with Tile::Wall. As such this ensures that any Tile::Floor is 
/// reachable from all other Tile::Floor in the Map.
/// 
/// Finally, to ensure all entities remain within the map, the outer rim of the
/// Map is turned into Tile::Wall. 
/// 
/// # Path Finding
/// To navigate the Map, an A* algorithm is used. 
/// 
/// # Example
/// 
/// ```
/// extern crate rust_game;
/// use rust_game::math::random::Seed;
/// use rust_game::levels::Level;
/// use rust_game::levels::map::{MapIdx, pathfind};
/// 
/// fn main() {
///     
///     // Create a Seed
///     let seed: Seed = [1,2,3,4,5,6,7,8,
///                       1,2,3,4,5,6,7,8,
///                       1,2,3,4,5,6,7,8,
///                       1,2,3,4,5,6,7,8];
///     // Create a level 
///     let level = Level::new(seed);
/// 
///     let start = MapIdx::new(5,5);
///     let target = MapIdx::new(10,3);
/// 
///     if let Some((path, cost)) = pathfind(&level.map, &start, &target){
///         // do something
///     }
/// 
/// }
/// ```
pub struct Level {
    pub map: Map,
    pub width: i32,
    pub height: i32,
    pub rng: RNG
}

impl Level {
    // Referenced http://rosettacode.org/wiki/Conway%27s_Game_of_Life#Rust for Game of Life
    // implementation.

    /// Returns a new level using a random number generator created from the 
    /// input seed.
    pub fn new(init: Seed) -> Self {
        let mut map: Map = Map::new();
        let mut rng = from_seed(init);

        // Add initial Tile::Walls to the Map.
        for h in 0..LEVEL_HEIGHT {
            for w in 0..LEVEL_WIDTH {
                // Any given tile has a 50/50 chance of being a wall initially.
                if next_u32(&mut rng) % 2 == 1 {
                    map.insert(MapIdx::new(w,h), Tile::new(TileVariant::Wall, MapIdx::new(w, h)));
                }
            }
        }

        // Run Conway's Game of Life on the Tile::Walls in the Map
        map = Level::iterate_map(map, LEVEL_GEN_ITERS);

        // Fill the empty spaces in the Map with Tile::Floor
        for h in 0..LEVEL_HEIGHT {
            for w in 0..LEVEL_WIDTH {
                match map.contains_key(&MapIdx::new(w,h)) {
                    false => {
                        map.insert(MapIdx::new(w,h), Tile::new(TileVariant::Floor, MapIdx::new(w, h)));
                    } ,
                    _ => (),
                }
            }
        }
        
        // Fill untraversable space with walls
        map = Level::fill_walls(map, LEVEL_WIDTH, LEVEL_HEIGHT);
        map = Level::fill_edge(map, LEVEL_WIDTH, LEVEL_HEIGHT);
        Level {map: map, width: LEVEL_WIDTH, height: LEVEL_HEIGHT, rng: rng}

    }

    /// Returns a list of indicies surrounding the input index. Differs from
    /// MapIdx.neighbours because it looks in all 8 surrounding locations. 
    fn neighbours(map_idx: &MapIdx) -> Vec<MapIdx> {
        vec![MapIdx::new(map_idx.x-1, map_idx.y-1),MapIdx::new(map_idx.x-1, map_idx.y),MapIdx::new(map_idx.x-1, map_idx.y+1),
             MapIdx::new(map_idx.x,   map_idx.y-1)  ,                                  MapIdx::new(map_idx.x,   map_idx.y+1),
             MapIdx::new(map_idx.x+1, map_idx.y-1),MapIdx::new(map_idx.x+1, map_idx.y),MapIdx::new(map_idx.x+1, map_idx.y+1)]
    }

    /// Returns a HashMap relating each position in the input Map to the number
    /// of neighbours surrounding the point. A neighbour is considered to be
    /// any Tile variant within the Map. 
    /// 
    /// For use in level generation, only Tile::Walls should be present in the
    /// input Map. 
    fn neighbour_counts(map: &Map) -> HashMap<MapIdx, i32> {
        let mut ncnts = HashMap::new();
        for (idx, _tile) in map.iter() {
            for neighbour in Level::neighbours(idx) {
                *ncnts.entry(neighbour).or_insert(0) += 1;
            }
        }
        ncnts
    }
 
    /// Returns a new Map as created by simulating a generation of Conway's 
    /// Game of Life. Assumes that the input Map only contains Tile::Walls.
    fn generation(map: Map) -> Map {
        Level::neighbour_counts(&map)
            .into_iter()
            .filter_map(|(idx, cnt)|
                match (cnt, map.contains_key(&idx)) {
                    (2, true) |
                    (3, ..) => Some((idx, Tile::new(TileVariant::Wall, idx))),
                    _ => None
            })
            .collect()
    }

    /// Returns the Map created after processing the input Map for iters 
    /// generations of Conway's Game of Life. 
    fn iterate_map(init: Map, iters: i32) -> Map {
        let mut map: Map = init; 
        for i in 0..iters+1 {
            if i != 0 {
                map = Level::generation(map);
            }
        }
        map
    }

    /// returns: An augmented version of the map which has only one traversable
    /// area. The connected area of the map is chosen, every other space is 
    /// converted into a Tile::Wall. 
    /// 
    /// Filling the walls follows a 3 step process:
    ///     1. Traverse all tiles in the map and flood_fill() each Floor:Tile. 
    ///     2. Find the largest traversable area.
    ///     3. Traverse all tiles in the map again, and convert the largest 
    ///        traversable area into Tile::Floor and everything else into
    ///        Tile::Wall.
    fn fill_walls(mut map: Map, width: i32, height:i32) -> Map {

        /// Convertes all reachable Tile::Floor from start to Tile::Cust(new_val)
        fn flood_fill(mut map: &mut Map, start: &MapIdx, new_val: &i32, sets: &mut HashMap<i32, i32>) {
            if let Some(tile) = map.get(start) {
                match tile.variant {
                    TileVariant::Floor => {
                        map.remove(start);
                        map.insert(MapIdx::new(start.x,start.y), Tile::new(TileVariant::Cust(*new_val), *start));
                        *sets.entry(*new_val).or_insert(1) += 1;
                        flood_fill(&mut map, &MapIdx::new(start.x, start.y-1), new_val, sets);
                        flood_fill(&mut map, &MapIdx::new(start.x, start.y+1), new_val, sets);
                        flood_fill(&mut map, &MapIdx::new(start.x-1, start.y), new_val, sets);
                        flood_fill(&mut map, &MapIdx::new(start.x+1, start.y), new_val, sets);
                    },
                    _ => (),
                }
            }
        }

        // Used to track the number of different regions in the Map.
        let mut region = 0;
        // Used to count the sizes of each different region.
        // Key: region number & Value: region count.
        let mut sets: HashMap<i32, i32> = HashMap::new();
        
        // Traverse map and flood_fill each Tile::Floor.
        for h in 0..height {
            for w in 0..width {

                if let Some(tile) = map.get(&MapIdx::new(w,h)) {
                    match tile.variant {
                        TileVariant::Floor => {
                            flood_fill(&mut map, &MapIdx::new(w,h), &region, &mut sets);
                            // increment region counter.
                            region += 1;    
                        },
                        _ => ()
                    }
                }

            }
        }

        // Find the region in sets with the largest number of traversable spaces.
        let mut max = (-1,-1);
        for (region, count) in sets {
            if count > max.1 {
                max = (region, count);
            }
        }

        // Convert back to Tile::Floor and Tile::Wall
        for h in 0..height {
            for w in 0..width {
                
                if let Some(tile) = map.get(&MapIdx::new(w,h)) {
                    match tile.variant {
                        TileVariant::Cust(max_val) if max_val == max.0 => {
                            let idx = MapIdx::new(w, h);
                            map.remove(&idx);
                            map.insert(idx, Tile::new(TileVariant::Floor, idx));
                        },
                        TileVariant::Cust(_val) => {
                            let idx = MapIdx::new(w, h);
                            map.remove(&idx);
                            map.insert(idx, Tile::new(TileVariant::Wall, idx));
                        },
                        _ => (),
                    }
                }
            }
        }

        map
    }

    /// Fills in the edges of the Map with Walls, to prevent anyone from exiting
    /// the Level.
    fn fill_edge(mut map: Map, width: i32, height:i32) -> Map {

        for w in 0..width {
            let idx = MapIdx::new(w, 0);
            map.remove(&idx);
            map.insert(idx, Tile::new(TileVariant::Wall, idx));
            let idx = MapIdx::new(w, height-1);
            map.remove(&idx);
            map.insert(idx, Tile::new(TileVariant::Wall, idx));
        }
        for h in 0..height {
            let idx = MapIdx::new(0, h);
            map.remove(&idx);
            map.insert(idx, Tile::new(TileVariant::Wall, idx));
            let idx = MapIdx::new(width-1, h);
            map.remove(&idx);
            map.insert(idx, Tile::new(TileVariant::Wall, idx));
        }

        map

    }

}