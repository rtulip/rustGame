use graphics::types::Color;
use graphics::{Context, Graphics};
use crate::level::LevelController;

pub const WINDOW_WIDTH: f64 = 1024.0;
pub const WINDOW_HEIGHT: f64 = 812.0;

pub struct LevelViewSettings {

    pub graphics_position: [f64; 2],
    pub tile_size: f64,
    pub background_color: Color,

}

impl LevelViewSettings {

    
    pub fn new() -> LevelViewSettings {

        LevelViewSettings{
            graphics_position: [0.0; 2],
            tile_size: 10.0,
            background_color: [0.8, 0.8, 1.0, 1.0],

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
        let bounding_box = [settings.graphics_position[0],
                            settings.graphics_position[1],
                            WINDOW_WIDTH,
                            WINDOW_HEIGHT];

        Rectangle::new(settings.background_color).draw(bounding_box, &c.draw_state, c.transform, g);

    }

}