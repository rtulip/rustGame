use crate::entity::player::PlayerController;
use graphics::types::Color;
use graphics::{Context, Graphics};

const PLAYER_SIZE: f64 = 16.0;

pub struct PlayerViewSettings {
    pub size: f64,
    pub color: Color,
}

impl PlayerViewSettings {

    pub fn new() -> PlayerViewSettings {
        PlayerViewSettings { size: PLAYER_SIZE, color: [0.75, 0.12, 0.08,1.0] }
    }

}

pub struct PlayerView {

    settings: PlayerViewSettings,

}

impl PlayerView {

    pub fn new(settings: PlayerViewSettings) -> PlayerView {
        PlayerView { settings: settings }
    }

    pub fn draw<G: Graphics>(&self, controller: &PlayerController, c: &Context, g: &mut G) {
        // Todo
    }

}