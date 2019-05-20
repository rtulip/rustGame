use graphics::types::Color;
use graphics::{Context, Graphics};
use crate::level::{LevelController, MapIdx};
use crate::entity::tile::Tile;

pub const WINDOW_WIDTH: f64 = 1000.0;
pub const WINDOW_HEIGHT: f64 = 1000.0;

pub struct LevelViewSettings {

    pub tile_size: f64,
    pub background_color: Color,
    pub error_color: Color,
    pub floor_color: Color,
    pub wall_color: Color,

}

impl LevelViewSettings {
    
    pub fn new() -> LevelViewSettings {
        LevelViewSettings{
            tile_size: 20.0,
            background_color: [0.0, 0.0, 0.0, 1.0],
            error_color: [1.0, 0.0, 0.0, 1.0],
            wall_color: [0.3, 0.3, 0.2, 1.0],
            floor_color: [0.2, 0.13, 0.08, 1.0],
        }
    }

}

pub struct LevelView {

    pub settings: LevelViewSettings,

}

impl LevelView {

    pub fn new(settings: LevelViewSettings) -> LevelView {
        LevelView {settings: settings}
    }
    
    pub fn draw<G: Graphics>(&self, controller: &LevelController, c: &Context, g: &mut G) {
        use graphics::{Line, Rectangle};

        let settings = &self.settings;
        
        // Traverse the controller's map.
        for h in 0..controller.get_height(){
            for w in 0..controller.get_width(){
                // Draw a different colors for walls floors, and errors.
                match controller.get_map().get(&(w,h)){
                    Some(Tile::Floor) => {
                        Rectangle::new(settings.floor_color)
                            .draw([self.convert_idx((w,h))[0], self.convert_idx((w,h))[1], settings.tile_size,settings.tile_size], 
                                  &c.draw_state, c.transform, g);
                    },
                    Some(Tile::Wall) => {
                        Rectangle::new(settings.wall_color)
                            .draw([self.convert_idx((w,h))[0], self.convert_idx((w,h))[1], settings.tile_size,settings.tile_size], 
                                  &c.draw_state, c.transform, g);
                    },
                    _ => {
                        Rectangle::new(settings.error_color)
                            .draw([self.convert_idx((w,h))[0], self.convert_idx((w,h))[1], settings.tile_size,settings.tile_size], 
                                  &c.draw_state, c.transform, g);
                    },
                }; 
            }
        }

    }

    pub fn convert_idx(&self, idx: MapIdx) -> [f64;2] {
        [idx.0 as f64 * self.settings.tile_size, idx.1 as f64 * self.settings.tile_size]
    }

}

