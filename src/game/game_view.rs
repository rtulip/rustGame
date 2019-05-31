use crate::game::GameModel;
use crate::entity::player::PlayerState;
use crate::level::MapIdx;
use crate::misc::point2d::Point2;
use crate::traits::draw::Draw;
use graphics::{Context, Graphics, Transformed, Image};
use graphics::character::CharacterCache;
use crate::game::consts::{
    ERROR_COLOR,
    TILE_SIZE,
};

/// A structure responsible for drawing the GameModel.
pub struct GameView {}

impl GameView {
    
    /// Creates a new GameView
    pub fn new() -> Self {
        Self { }
    }

    /// Draws the GameModel by first drawing the level, then the player, then
    /// the beacon, and finally all the enemies.
    pub fn draw<G: Graphics, C>(
        &mut self, 
        model: &GameModel,
        glyphs: &mut C, 
        c: &Context, 
        g: &mut G
    ) 
        where C: CharacterCache<Texture = G::Texture> {
        
        self.draw_level(model, c, g);
        self.draw_text(model, glyphs, c, g);
        self.draw_beacon(model, c, g);
        self.draw_resources(model, c, g);
        self.draw_towers(model, c, g);
        self.draw_enemies(model, c, g);
        self.draw_player(model, c, g);
        
    }

    /// Draws the Level of the GameModel by looping through each tile in the 
    /// Map.
    fn draw_level<G: Graphics>(&self, model: &GameModel, c: &Context, g: &mut G) {
        for h in 0..model.level.height {
            for w in 0..model.level.width {
                if let Some(tile) = model.level.map.get(&MapIdx::new(w, h)) {
                    tile.shape.draw(c,g);
                }
            }
        }

    }

    /// Draws the Player of the GameModel. If the player is attacking, the 
    /// Player's sword is drawn as well.
    fn draw_player<G: Graphics>(&self, model: &GameModel, c: &Context, g: &mut G) {
        // Draw the player
        model.player.shape.draw(c, g);
        // Draw the player's attack anmiation if in Active state. 
        match model.player.state {
            PlayerState::Attacking => {
                model.player.attack.shape.draw(c,g);
            },
            _ => (),
        }
        
    }

    /// Draws the GameModel's Beacon
    fn draw_beacon<G: Graphics>(&self, model: &GameModel, c: &Context, g: &mut G) {
        model.beacon.shape.draw(c, g);
    }

    /// Draws each enemy in the GameModel enemy list
    fn draw_enemies<G: Graphics>(&self, model: &GameModel, c: &Context, g: &mut G) {
        for enemy in model.enemies.iter() {
            enemy.shape.draw(c,g);
        }
    }

    /// Draws each resource in the GameModels resource list
    fn draw_resources<G: Graphics>(&self, model: &GameModel, c: &Context, g: &mut G) {

        for resource in model.resources.iter() {

            resource.shape.draw(c,g);

        }

    }

    fn draw_towers<G: Graphics>(&self, model: &GameModel, c: &Context, g: &mut G){

        for tower in model.towers.iter() {
            tower.draw(c, g);
        }

    }

    /// Draws all text. 
    fn draw_text<G: Graphics, C>(
        &mut self, 
        model: &GameModel,
        glyphs: &mut C, 
        c: &Context, 
        g: &mut G
    ) 
        where C: CharacterCache<Texture = G::Texture> {

        let text_img = Image::new_color(ERROR_COLOR);
        let score_string = model.player.resources.to_string();
        let char_point = Point2 {
            x: (model.level.width - 1) as f64 * TILE_SIZE,
            y: 0.0
        };
        
        for ch in score_string.chars() {
            if let Ok(character) = glyphs.character(34, ch) {
                text_img.draw(
                    character.texture, 
                    &c.draw_state,
                    c.transform.trans(char_point.x, char_point.y),
                    g);
            };
        }
    }
}