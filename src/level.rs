extern crate pathfinding;
use crate::entity::tile::Tile;
use crate::misc::random::{Seed,RNG,from_seed, next_u32};
use pathfinding::prelude::{absdiff, astar};
use std::collections::HashMap;

const WIDTH: i32 = 50;
const HEIGHT: i32 = 50;
const ITERS: i32 = 5;

/// MapIdx Type
/// 
/// MapIdx represents a grid position in 2D space. 
/// MapIdx.0 is x and MapIdx.1 is y
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MapIdx {
    pub x: i32,
    pub y: i32
}

impl MapIdx {
    pub fn new(x: i32, y: i32) -> Self {
        Self {x: x, y: y}
    }

    fn distance(&self, other: &MapIdx) -> u32 {
        (absdiff(self.x, other.x) + absdiff(self.y, other.y)) as u32
    }

    fn neighbours(&self) -> Vec<MapIdx> {
        vec![MapIdx::new(self.x-1, self.y), MapIdx::new(self.x, self.y-1),
             MapIdx::new(self.x+1, self.y), MapIdx::new(self.x, self.y+1)]
    }

    fn successors(&self, map: &Map) -> Vec<(MapIdx, u32)>  {
        let mut neighbours = self.neighbours();
        let mut remove: Vec<usize> = Vec::new();
        for i in (0..neighbours.len()).rev() {
            match map.get(&neighbours[i]){
                Some(Tile::Wall) => {
                    remove.push(i);
                },
                Some(Tile::Cust(_val)) => {
                    remove.push(i);
                }
                _ => ()
            }
        }
        for i in remove {
            neighbours.remove(i);
        }
        neighbours.into_iter().map(|p| (p, 1)).collect()

    }
}
/// Map Type
/// 
/// A Map is a HashMap which associates positions to Tiles. 
pub type Map = HashMap<MapIdx, Tile>;

/// Level Struct
/// 
/// Encapsulates all information for a level.
/// Includes a Map, the width and height of the level, and the ChaChaRng. 
pub struct Level {
    pub map: Map,
    pub width: i32,
    pub height: i32,
    pub rng: RNG
}

impl Level {
    // Referenced http://rosettacode.org/wiki/Conway%27s_Game_of_Life#Rust for Game of Life
    // implementation.

    /// new()
    /// 
    /// args: 
    ///     width: i32: The width of the Map to be created.
    ///     height: i32: The height of the Map to be created.
    ///     iters: i32: The number of iterations of Conway's Game of Life to run.
    ///     seed: <ChaChaCore as SeedableRng::Seed>: The seed used to create a new random number
    ///         generator. 
    /// 
    /// returns: The newly created Level.
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

    /// neighbours()
    /// 
    /// args:
    ///     &(x,y): &MapIdx: A reference to a position.
    /// 
    /// return: A vector of all 8 positions which surround the input position.
    /// 
    /// Will go beyond width and height boundaries in edge cases. Ensure that edges
    /// are checked to avoid issues. 
    fn neighbours(map_idx: &MapIdx) -> Vec<MapIdx> {
        vec![MapIdx::new(map_idx.x-1, map_idx.y-1),MapIdx::new(map_idx.x-1, map_idx.y),MapIdx::new(map_idx.x-1, map_idx.y+1),
             MapIdx::new(map_idx.x,   map_idx.y-1)  ,                                  MapIdx::new(map_idx.x,   map_idx.y+1),
             MapIdx::new(map_idx.x+1, map_idx.y-1),MapIdx::new(map_idx.x+1, map_idx.y),MapIdx::new(map_idx.x+1, map_idx.y+1)]
    }

    /// neighbour_counts()
    /// 
    /// args:
    ///     map: &Map: A reference to a Map.
    /// 
    /// returns: A HashMap relating each position in the input Map to the number of
    /// neighbours surrounding the point. 
    /// 
    /// Assumes that the input Map only contains Tile::Walls at this time. Should  
    /// be updated to count the surrounding number of Tile::Wall instead of number  
    /// of elements surrounding each point.
    fn neighbour_counts(map: &Map) -> HashMap<MapIdx, i32> {
        let mut ncnts = HashMap::new();
        for (idx, _tile) in map.iter() {
            for neighbour in Level::neighbours(idx) {
                *ncnts.entry(neighbour).or_insert(0) += 1;
            }
        }
        ncnts
    }

    /// generation()
    /// 
    /// args:
    ///     map: Map: The Map to be progressed by a generation.
    /// 
    /// returns: A new Map as created by simulating a generation of Conway's Game
    /// of Life.
    /// 
    /// Assumes that the input Map only contains Tile::Walls at this time. Should  
    /// be updated to handle other Tile types within the Map.
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

    /// iterate_map()
    /// 
    /// args:
    ///     init: Map: The initial arangement of the Map to be generated.
    ///     iters: i32: The number of iterations of Conway's Game of Life to run.
    ///
    /// returns: The iterated Map 
    fn iterate_map(init: Map, iters: i32) -> Map {
        let mut map: Map = init; 
        for i in 0..iters+1 {
            if i != 0 {
                map = Level::generation(map);
            }
        }
        map
    }

    /// fill_walls()
    /// 
    /// args:
    ///     mut map: Map: A mutable version of the Map to be filled.
    ///     width: i32: The width of the map.
    ///     height: i32: The height of the map.
    /// 
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

        /// flood_fill()
        /// 
        /// args:
        ///     mut map: &mut Map: The Map which is being traversed.
        ///     start: &MapIdx: The starting position of the flood fill.
        ///     new_val: &i32: The replacement value of the flood fill.
        ///     sets: &mut HashMap<i32, i32>: A hashmap to track the sets of 
        ///         traversable area and their sizes.
        /// 
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

    /// fill_edge()
    /// 
    /// args:
    ///     map: Map: A mutable map of which the edges will be filled
    ///     width: i32: The width of the Map
    ///     height: i32 The height of the Map
    /// 
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

    pub fn pathfind(&self, start: &MapIdx, target: &MapIdx) -> Option<(Vec<MapIdx>, u32)> {

        astar(start, |p| p.successors(&self.map), |p| p.distance(&target) / 3,|p| *p == *target)

    }

}


