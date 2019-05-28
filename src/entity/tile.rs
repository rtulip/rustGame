use crate::traits::shape;

/// Used to represent different types of tiles which can be found in the map.
/// Custom tiles can be created, but must contain a i32 which can be printed. 
pub enum Tile {
    Floor,
    Wall,
    Spawner,
    Cust(i32),
}

impl shape::Shape for Tile {
    type ShapeVairant = shape::RectangleType;
    fn get_shape(&self) -> Self::ShapeVairant {
        shape::RectangleType {}
    }
}