use crate::game::GameModel;
use crate::shape::Shape;
use crate::entity::tile::Tile;
use graphics::{Context, Graphics};
use graphics::types::Color;

struct GameViewSettings {
    pub tile_size: f64,
    pub error_color: Color,
    pub floor_color: Color,
    pub wall_color: Color,
}
impl GameViewSettings {
    fn new() -> Self {
        Self {  tile_size: 20.0,
                error_color: [1.0, 0.0, 0.0, 1.0],
                wall_color: [0.3, 0.3, 0.2, 1.0],
                floor_color: [0.2, 0.13, 0.08, 1.0], }
    }
}

pub struct GameView {
    settings: GameViewSettings,
}

impl GameView {
    
    pub fn new() -> Self {
        Self { settings: GameViewSettings::new() }
    }

    pub fn draw<G: Graphics>(&self, model: &GameModel, c: &Context, g: &mut G) {
        let settings = &self.settings;
        for h in 0..model.level.get_height() {
            for w in 0..model.level.get_width() {
                match model.level.get_map().get(&(w,h)){
                    Some(Tile::Floor) => {
                        Tile::Floor.get_shape().draw(settings.floor_color,
                                                     w as f64 * settings.tile_size, 
                                                     h as f64 * settings.tile_size, 
                                                     settings.tile_size,
                                                     settings.tile_size, 
                                                     c, 
                                                     g)
                    },
                    Some(Tile::Wall) => {
                        Tile::Floor.get_shape().draw(settings.wall_color,
                                                     w as f64 * settings.tile_size, 
                                                     h as f64 * settings.tile_size, 
                                                     settings.tile_size,
                                                     settings.tile_size, 
                                                     c, 
                                                     g)
                    },
                    _ => {
                        Tile::Floor.get_shape().draw(settings.error_color,
                                                     w as f64 * settings.tile_size, 
                                                     h as f64 * settings.tile_size, 
                                                     settings.tile_size,
                                                     settings.tile_size, 
                                                     c, 
                                                     g)
                    },
                };
            }
        }
    }
}