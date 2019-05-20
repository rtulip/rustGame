extern crate rand_chacha;
extern crate rand_core;
extern crate rand;

use std::collections::HashMap;
use std::fmt;
use std::env;

use rand_chacha::{ChaChaCore, ChaChaRng};
use rand_core::{SeedableRng,RngCore};

/// Tile Enum
/// 
/// Used to represent different types of tiles which can be found in the map.
/// Custom tiles can be created, but must contain a i32 which can be printed. 
/// 
/// Tiles implement fmt::Display so that a tile can be printed.
enum Tile {
    Floor,
    Wall,
    Cust(i32),
}

/// Tile Display implementation
/// 
/// Floors are written as a "."
/// Walls are written as a "W"
/// Custome tiles are written as whatever i32 is provided  
impl fmt::Display for Tile{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match *self{
            Tile::Floor => write!(f, "."),
            Tile::Wall => write!(f, "W"),
            Tile::Cust(i) => write!(f, "{}", i)
        }
    }
}

/// Pos Type
/// 
/// Pos represents a position in 2D space. 
/// Pos.0 is x and Pos.1 is y
type Pos = (i32, i32);
/// Map Type
/// 
/// A Map is a HashMap which associates positions to Tiles. 
type Map = HashMap<Pos, Tile>;

/// Level Struct
/// 
/// Encapsulates all information for a level.
/// Includes a Map, the width and height of the level, and the ChaChaRng. 
struct Level {
    map: Map,
    width: i32,
    height: i32,
    rng: ChaChaRng
}

impl Level{

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
    fn new(width: i32, height:i32, iters:i32, seed: <ChaChaCore as SeedableRng>::Seed) -> Self {
        let mut map: Map = Map::new();
        let mut rng = ChaChaRng::from_seed(seed);

        // Add initial Tile::Walls to the Map.
        for h in 0..height {
            for w in 0..width {
                // Any given tile has a 50/50 chance of being a wall initially.
                if rng.next_u32() % 2 == 1 {
                    map.insert((w,h), Tile::Wall);
                }
            }
        }

        // Run Conway's Game of Life on the Tile::Walls in the Map
        map = Level::iterate_map(map, iters);

        // Fill the empty spaces in the Map with Tile::Floor
        for h in 0..height {
            for w in 0..width {
                match map.contains_key(&(w,h)){
                    false =>{
                        map.insert((w,h), Tile::Floor);
                    } ,
                    _ => (),
                }
            }
        }
        
        // Fill untraversable space with walls
        map = Level::fill_walls(map, width, height);
        Level {map: map, width: width, height: height, rng: rng}

    }

    /// print_level()
    /// 
    /// args: 
    ///     map: &Map: The map to be printed.
    ///     width: i32: The width of the map.
    ///     height: i32: The height of the map. 
    ///     
    /// Traverseses the map and prints out all Tiles in a grid.
    fn print_level(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.map.get(&(x,y)){
                    Some(tile) => print!("{} ", tile),
                    None => print!("  ")
                }
            }
            println!();
        }
    }

    /// neighbours()
    /// 
    /// args:
    ///     &(x,y): &Pos: A reference to a position.
    /// 
    /// return: A vector of all 8 positions which surround the input position.
    /// 
    /// Will go beyond width and height boundaries in edge cases. Ensure that edges
    /// are checked to avoid issues. 
    fn neighbours(&(x,y): &Pos) -> Vec<Pos> {
        vec![
            (x-1,y-1), (x,y-1), (x+1,y-1),
            (x-1,y),            (x+1,y),
            (x-1,y+1), (x,y+1), (x+1,y+1),
        ]
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
    fn neighbour_counts(map: &Map) -> HashMap<Pos, i32> {
        let mut ncnts = HashMap::new();
        for (pos, _tile) in map.iter(){
            for neighbour in Level::neighbours(pos){
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
            .filter_map(|(pos, cnt)|
                match (cnt, map.contains_key(&pos)) {
                    (2, true) |
                    (3, ..) => Some((pos, Tile::Wall)),
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
        ///     start: &Pos: The starting position of the flood fill.
        ///     new_val: &i32: The replacement value of the flood fill.
        ///     sets: &mut HashMap<i32, i32>: A hashmap to track the sets of 
        ///         traversable area and their sizes.
        /// 
        /// Convertes all reachable Tile::Floor from start to Tile::Cust(new_val)
        fn flood_fill(mut map: &mut Map, start: &Pos, new_val: &i32, sets: &mut HashMap<i32, i32>){
            match map.get(start){
                Some(Tile::Floor) => {
                    map.remove(start);
                    map.insert(*start, Tile::Cust(*new_val));
                    *sets.entry(*new_val).or_insert(1) += 1;
                    flood_fill(&mut map, &(start.0, start.1-1), new_val, sets);
                    flood_fill(&mut map, &(start.0, start.1+1), new_val, sets);
                    flood_fill(&mut map, &(start.0-1, start.1), new_val, sets);
                    flood_fill(&mut map, &(start.0+1, start.1), new_val, sets);
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

                match map.get(&(w,h)){
                    Some(Tile::Floor) => {
                        flood_fill(&mut map, &(w,h), &region, &mut sets);
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
                
                match map.get(&(w,h)) {
                    // If Tile is of the most traversable region, convert to floor
                    Some(Tile::Cust(max_val)) if *max_val == max.0 => {
                        map.remove(&(w,h));
                        map.insert((w,h), Tile::Floor);
                    },
                    // If Tile is of another region, convert to Wall.
                    Some(Tile::Cust(_val)) => {
                        map.remove(&(w,h));
                        map.insert((w,h), Tile::Wall);
                    },
                    _ => ()
                    
                }
            }
        }

        map
    }

}

/// create_seed()
/// 
/// args:
///     debug: bool: A flag to used a fixed debug seed
/// 
/// returns: A Seed which can be used for the ChaCha random number generator
/// which will be used for the entirety of a game
/// 
/// The ChaCha random number generator requires [u8: 32] as input. If in debug
/// mode, the seed defualts to [1,2,3,4,5,6,7,8,
///                             1,1,1,1,1,1,1,1,
///                             2,2,2,2,2,2,2,2,
///                             1,2,3,4,5,6,7,8]. Otherwise, a new seed is 
/// generated using rand::random::<u8>() 32 times to fill the array.
fn create_seed(debug: bool) -> <ChaChaCore as SeedableRng>::Seed {

    if debug {
        let seed: <ChaChaCore as SeedableRng>::Seed = [1,2,3,4,5,6,7,8,
                                                       1,1,1,1,1,1,1,1,
                                                       2,2,2,2,2,2,2,2,
                                                       1,2,3,4,5,6,7,8];
        seed
    } else {
        let seed: <ChaChaCore as SeedableRng>::Seed = [rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 
                                                       rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 
                                                       rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 
                                                       rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 
                                                       rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 
                                                       rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 
                                                       rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 
                                                       rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()];   

        seed
    }
}

fn main() {

    let mut debug = false;
    let args: Vec<String> = env::args().collect();

    // Argument parsing
    // cargo run -- *arguments go here*

    // Arguments: 
    //      -d | --debug: Use a constant known seed
    match args.len(){
        len if len > 1 => {
            for i in 1..args.len(){
                match &args[i]{
                    string if *string == String::from("-d") || *string == String::from("--debug") => {
                        debug = true;
                    },
                    _ => ()
                }
            }
        },
        _ => () 

    }

    let seed = create_seed(debug);
    let width: i32 = 50;
    let height: i32 = 50;
    let iters: i32 = 5;

    let lvl: Level = Level::new(width, height, iters, seed);
    
    lvl.print_level();

}