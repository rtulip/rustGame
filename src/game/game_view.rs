use crate::game::GameModel;
struct GameViewSettings {

}
impl GameViewSettings {
    fn new() -> Self {
        Self { }
    }
}

pub struct GameView {
    settings: GameViewSettings,
}

impl GameView {
    
    pub fn new() -> Self {
        Self { settings: GameViewSettings::new() }
    }

    pub fn draw(&self, model: &GameModel) {
        // TODO
    }
}