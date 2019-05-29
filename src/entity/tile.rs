/// Used to represent different types of tiles which can be found in the map.
use crate::traits::draw::{GenericShape,ShapeVariant};
use crate::level::MapIdx;
use crate::game::consts::{
    TILE_SIZE,
    FLOOR_COLOR,
    WALL_COLOR,
    SPAWNER_COLOR,
    ERROR_COLOR,
    map_idx_to_point2,
};

pub enum TileVariant {
    Floor,
    Wall,
    Spawner,
    Cust(i32),
}

pub struct Tile {
    pub variant: TileVariant,
    pub shape: GenericShape,
}

impl Tile {
    pub fn new(variant: TileVariant, idx: MapIdx) -> Self {
        match variant {
            TileVariant::Floor => {
                Self {
                    variant: variant,
                    shape: GenericShape::new(
                        ShapeVariant::Square{size: TILE_SIZE},
                        FLOOR_COLOR, 
                        map_idx_to_point2(idx)
                    ),
                }
            },
            TileVariant::Wall => {
                Self {
                    variant: variant,
                    shape: GenericShape::new(
                        ShapeVariant::Square{size: TILE_SIZE},
                        WALL_COLOR, 
                        map_idx_to_point2(idx)
                    ),
                }
            },
            TileVariant::Spawner => {
                Self {
                    variant: variant,
                    shape: GenericShape::new(
                        ShapeVariant::Square{size: TILE_SIZE},
                        SPAWNER_COLOR, 
                        map_idx_to_point2(idx)
                    ),
                }
            },
            TileVariant::Cust(_) => {
                Self {
                    variant: variant,
                    shape: GenericShape::new(
                        ShapeVariant::Square{size: TILE_SIZE},
                        ERROR_COLOR, 
                        map_idx_to_point2(idx)
                    ),
                }
            }
        }
        
    }
}