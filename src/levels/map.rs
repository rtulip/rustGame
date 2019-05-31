use crate::entity::tile::{Tile,TileVariant};
use pathfinding::prelude::{absdiff, astar};
use std::collections::HashMap;

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
    pub fn neighbours(&self) -> Vec<MapIdx> {
        vec![MapIdx::new(self.x-1, self.y), MapIdx::new(self.x, self.y-1),
             MapIdx::new(self.x+1, self.y), MapIdx::new(self.x, self.y+1)]
    }

    /// Returns a the positions surronding a MapIdx which are traversable in the 
    /// input Map and a cost. If the space is traversable, the cost is 1. Only
    /// Tile::Floor and Tile::Spawner variants are traversable. Any other Tile
    /// variant surrounding the MapIdx will be counted as impassable.
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
            if let Some(tile) = map.get(idx) {
                match tile.variant {
                    TileVariant::Floor => (),
                    TileVariant::Spawner => (),
                    _ => {
                        remove.push(i);
                    }
                }
            } else {
                remove.push(i);
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

/// Returns a list of MapIdx and a total cost if there exists a path from
/// start to target, otherwise returns None.
pub fn pathfind(map: &Map, start: &MapIdx, target: &MapIdx) -> Option<(Vec<MapIdx>, u32)> {

    astar(start, |p| p.successors(map), |p| p.distance(&target) / 3,|p| *p == *target)

}

/// A HashMap mapping MapIdxs to Tiles. Used to represent the game board.
pub type Map = HashMap<MapIdx, Tile>;