use crate::entity::player::{Player, PlayerController};
use crate::traits;
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
    
}

impl traits::View<[f64; 2], Player, PlayerController> for PlayerView {

    fn draw<G: Graphics>(&self, controller: &PlayerController, c: &Context, g: &mut G) {
        // Todo
    }

}