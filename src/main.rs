// http://rosettacode.org/wiki/Conway%27s_Game_of_Life#Rust

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

/// print_map()
/// 
/// args: 
///     map: &Map: The map to be printed.
///     width: i32: The width of the map.
///     height: i32: The height of the map. 
///     
/// Traverseses the map and prints out all Tiles in a grid.
fn print_map(map: &Map, width: i32, height: i32) {
    for y in 0..height {
        for x in 0..width {
            match map.get(&(x,y)){
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
        for neighbour in neighbours(pos){
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
    neighbour_counts(&map)
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
            map = generation(map);
        }
    }
    map
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

fn generate_map(rng: &mut ChaChaRng, width: i32, height: i32, iters: i32) -> Map{
    let mut map: Map = Map::new(); 

    for h in 0..height {
        for w in 0..width {
            if rng.next_u32() % 2 == 1 {
                map.insert((w,h), Tile::Wall);
            }
        }
    }

    map = iterate_map(map, iters);

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

    map
}

fn main() {

    let mut debug = false;
    let args: Vec<String> = env::args().collect();

    match args.len(){
        len if len > 1 => {
            for i in 1..args.len(){
                match &args[i]{
                    string if *string == String::from("-d") => {
                        debug = true;
                    },
                    _ => ()
                }
            }
        },
        _ => () 

    }

    let seed = create_seed(debug);
    let mut rng = ChaChaRng::from_seed(seed);

    let width: i32 = 50;
    let height: i32 = 50;
    let iters: i32 = 5;

    let map = generate_map(&mut rng, width, height, iters);
    
    print_map(&map, width, height);

}