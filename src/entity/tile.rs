use crate::traits::shape;

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

/// Tiles implement Shape with ShapeVariant RectangleType
impl shape::Shape for Tile {
    type ShapeVairant = shape::RectangleType;
    fn get_shape(&self) -> Self::ShapeVairant {
        shape::RectangleType {}
    }
}