extern crate pathfinding;
use crate::entity::tile::Tile;
use crate::misc::random::{Seed,RNG,from_seed, next_u32};
use pathfinding::prelude::{absdiff, astar};
use std::collections::HashMap;

const WIDTH: i32 = 50;
const HEIGHT: i32 = 50;
const ITERS: i32 = 5;

/// A struct for indexing into a Map.
/// 
/// # Example 
/// ```
/// extern crate rust_game;
/// use rust_game::Level::MapIdx;
/// 
/// // create an index for the point (5,3)
/// fn main() {
///     let idx = MapIdx::new(5, 3);
///     assert_eq(idx.x, 5);
///     assert_eq(idx.y, 3);
/// 
///     // The new function doesn't need to be used
///     let idx = MapIdx {x: 5, y: 3};
///     assert_eq(idx.x, 5);
///     assert_eq(idx.y, 3);
/// }
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MapIdx {
    pub x: i32,
    pub y: i32
}

impl MapIdx {
    
    /// Create new MapIdx
    pub fn new(x: i32, y: i32) -> Self {
        Self {x: x, y: y}
    }

    /// Calculates the straight line distance between two MapIdx points.
    /// Used as a heuristic for A* pathfinding in a Map.
    fn distance(&self, other: &MapIdx) -> u32 {
        (absdiff(self.x, other.x) + absdiff(self.y, other.y)) as u32
    }

    /// Returns the positions surrounding a MapIdx to the north, south, east, 
    /// and west. Used to navigate a Map using A*. 
    fn neighbours(&self) -> Vec<MapIdx> {
        vec![MapIdx::new(self.x-1, self.y), MapIdx::new(self.x, self.y-1),
             MapIdx::new(self.x+1, self.y), MapIdx::new(self.x, self.y+1)]
    }

    /// Returns a the positions surronding a MapIdx which are traversable in the 
    /// input Map and a cost. If the space is traversable, the cost is 1. Only
    /// Tile::Floor is traversable. Any other Tile variant surrounding the 
    /// MapIdx will be counted as impassable.
    fn successors(&self, map: &Map) -> Vec<(MapIdx, u32)> {
        // Find surrounding spaces
        let mut neighbours = self.neighbours();
        // A list of indicies to remove 
        let mut remove: Vec<usize> = Vec::new();
        // Traverse the neibhbours backwards, so that removing by index doesn't
        // cause any issues
        for (i, idx) in neighbours.iter().enumerate().rev() {
            // If map.get(idx) contains a Tile::Floor do nothing, otherwise mark
            // the tile for removal
            match map.get(idx) {
                Some(Tile::Floor) => (),
                _ => {
                    remove.push(i);
                }
            }
        }
        // Remove all marked tiles from neighbour list
        for i in remove {
            neighbours.remove(i);
        }
        // return the traversable tiles mapped with a traversal cost of 1
        neighbours.into_iter().map(|p| (p, 1)).collect()

    }
}

/// A HashMap mapping MapIdxs to Tiles. Used to represent the game board.
pub type Map = HashMap<MapIdx, Tile>;

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
/// use rust_game::misc::random::Seed
/// use rust_game::level::{Level, MapIdx};
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
///     let (path, cost) = level.pathfind(&start, &target);
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
        for h in 0..HEIGHT {
            for w in 0..WIDTH {
                // Any given tile has a 50/50 chance of being a wall initially.
                if next_u32(&mut rng) % 2 == 1 {
                    map.insert(MapIdx::new(w,h), Tile::Wall);
                }
            }
        }

        // Run Conway's Game of Life on the Tile::Walls in the Map
        map = Level::iterate_map(map, ITERS);

        // Fill the empty spaces in the Map with Tile::Floor
        for h in 0..HEIGHT {
            for w in 0..WIDTH {
                match map.contains_key(&MapIdx::new(w,h)) {
                    false => {
                        map.insert(MapIdx::new(w,h), Tile::Floor);
                    } ,
                    _ => (),
                }
            }
        }
        
        // Fill untraversable space with walls
        map = Level::fill_walls(map, WIDTH, HEIGHT);
        map = Level::fill_edge(map, WIDTH, HEIGHT);
        Level {map: map, width: WIDTH, height: HEIGHT, rng: rng}

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
                    (3, ..) => Some((idx, Tile::Wall)),
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
            match map.get(start) {
                Some(Tile::Floor) => {
                    map.remove(start);
                    map.insert(MapIdx::new(start.x,start.y), Tile::Cust(*new_val));
                    *sets.entry(*new_val).or_insert(1) += 1;
                    flood_fill(&mut map, &MapIdx::new(start.x, start.y-1), new_val, sets);
                    flood_fill(&mut map, &MapIdx::new(start.x, start.y+1), new_val, sets);
                    flood_fill(&mut map, &MapIdx::new(start.x-1, start.y), new_val, sets);
                    flood_fill(&mut map, &MapIdx::new(start.x+1, start.y), new_val, sets);
                },
                _ => ()
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

                match map.get(&MapIdx::new(w,h)) {
                    Some(Tile::Floor) => {
                        flood_fill(&mut map, &MapIdx::new(w,h), &region, &mut sets);
                        // increment region counter.
                        region += 1;
                    },
                    _ => ()
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
                
                match map.get(&MapIdx::new(w,h)) {
                    // If Tile is of the most traversable region, convert to floor
                    Some(Tile::Cust(max_val)) if *max_val == max.0 => {
                        map.remove(&MapIdx::new(w,h));
                        map.insert(MapIdx::new(w,h), Tile::Floor);
                    },
                    // If Tile is of another region, convert to Wall.
                    Some(Tile::Cust(_val)) => {
                        map.remove(&MapIdx::new(w,h));
                        map.insert(MapIdx::new(w,h), Tile::Wall);
                    },
                    _ => ()
                    
                }
            }
        }

        map
    }

    /// Fills in the edges of the Map with Walls, to prevent anyone from exiting
    /// the Level.
    fn fill_edge(mut map: Map, width: i32, height:i32) -> Map {

        for w in 0..width {
            map.remove(&MapIdx::new(w,0));
            map.insert(MapIdx::new(w,0), Tile::Wall);
            map.remove(&MapIdx::new(w,height-1));
            map.insert(MapIdx::new(w,height-1), Tile::Wall);
        }
        for h in 0..height {
            map.remove(&MapIdx::new(0,h));
            map.insert(MapIdx::new(0,h), Tile::Wall);
            map.remove(&MapIdx::new(width-1,h));
            map.insert(MapIdx::new(width-1,h), Tile::Wall);
        }

        map

    }

    /// Returns a list of MapIdx and a total cost if there exists a path from
    /// start to target, otherwise returns None.
    pub fn pathfind(&self, start: &MapIdx, target: &MapIdx) -> Option<(Vec<MapIdx>, u32)> {

        astar(start, |p| p.successors(&self.map), |p| p.distance(&target) / 3,|p| *p == *target)

    }

}


