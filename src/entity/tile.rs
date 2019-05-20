use std::fmt;
use std::env;

/// Tile Enum
/// 
/// Used to represent different types of tiles which can be found in the map.
/// Custom tiles can be created, but must contain a i32 which can be printed. 
/// 
/// Tiles implement fmt::Display so that a tile can be printed.
pub enum Tile {
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