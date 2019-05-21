use crate::entity::player::{Player, PlayerController};
use crate::traits;
use graphics::types::Color;
use graphics::{Context, Graphics};

const PLAYER_SIZE: f64 = 16.0;
const PLAYER_RADIUS: f64 = 8.0;

pub struct PlayerViewSettings {
    pub size: f64,
    pub radius: f64,
    pub player_color: Color,
    pub clear_color: Color,
}

impl PlayerViewSettings {

    pub fn new() -> PlayerViewSettings {
        PlayerViewSettings { size: PLAYER_SIZE,
                             radius: PLAYER_RADIUS,
                             player_color: [0.75, 0.12, 0.08,1.0],
                             clear_color: [0.0, 0.0, 0.0, 0.0] }
    }

}

pub struct PlayerView {

    settings: PlayerViewSettings,

}

impl PlayerView {

    pub fn new(settings: PlayerViewSettings) -> PlayerView {
        PlayerView { settings: settings }
    }
    
}

impl traits::View<[f64; 2], Player, PlayerController> for PlayerView {

    fn draw<G: Graphics>(&self, controller: &PlayerController, c: &Context, g: &mut G) {
        use graphics::{Rectangle};
        let settings = &self.settings;

        Rectangle::new_round(settings.player_color, settings.radius)
            .draw([controller.get_position()[0], controller.get_position()[1], settings.size, settings.size],
                  &c.draw_state, c.transform, g);
        
    }
}