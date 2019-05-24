use crate::game::GameModel;
use crate::traits::shape::Shape;
use crate::entity::tile::Tile;
use graphics::{Context, Graphics};
use graphics::types::Color;

const TILE_SIZE: f64 = 20.0;
const PLAYER_SIZE: f64 = 16.0;
const PLAYER_RADIUS: f64 = 8.0;
const FLOOR_COLOR: Color = [0.2, 0.13, 0.08, 1.0];
const WALL_COLOR: Color = [0.3, 0.3, 0.2, 1.0];
const PLAYER_COLOR: Color = [0.75, 0.12, 0.08,1.0];
const ERROR_COLOR: Color = [1.0, 0.0, 0.0, 1.0];


pub struct GameViewSettings {
    pub tile_size: f64,
    pub floor_color: Color,
    pub wall_color: Color,
    pub player_size: f64,
    pub player_radius: f64,
    pub player_color: Color,
    pub error_color: Color,
    
}
impl GameViewSettings {
    fn new() -> Self {
        Self {  
            tile_size: TILE_SIZE,
            floor_color: FLOOR_COLOR,
            wall_color: WALL_COLOR,
            player_size: PLAYER_SIZE,
            player_radius: PLAYER_RADIUS,
            player_color: PLAYER_COLOR,
            error_color: ERROR_COLOR
        }
    }
}

pub struct GameView {
    pub settings: GameViewSettings,
}

impl GameView {
    
    pub fn new() -> Self {
        Self { settings: GameViewSettings::new() }
    }

    pub fn draw<G: Graphics>(&self, model: &GameModel, c: &Context, g: &mut G) {
        self.draw_level(model, c, g);
        self.draw_player(model, c, g);
    }

    fn draw_level<G: Graphics>(&self, model: &GameModel, c: &Context, g: &mut G) {
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

    fn draw_player<G: Graphics>(&self, model: &GameModel, c: &Context, g: &mut G) {
        model.player.get_shape().draw(
            PLAYER_COLOR,
            PLAYER_RADIUS,
            model.player.position[0],
            model.player.position[1],
            PLAYER_SIZE,
            PLAYER_SIZE,
            c,
            g
        )
    }
}