use crate::game::GameModel;
use crate::level::MapIdx;
use crate::misc::point2d::Point2;
use crate::traits::shape::Shape;
use crate::traits::state::State;
use crate::traits::draw::Draw;
use crate::entity::attack;
use graphics::{Context, Graphics, Transformed, Image};
use graphics::types::Color;
use graphics::character::CharacterCache;
use std::f64;

/// An enum to describe all the different states of animation. Ready means that
/// the animation is ready to start. Active means the animation is currently in
/// progress, and Finished means that the animation has completed.
pub enum AnimationEnum {
    Active,
    Finished,
    Ready,
}

/// A structure to fully describe an animation.
pub struct MeleeAnimation {
    pub animation_width: f64,
    pub animation_height: f64,
    pub animation_color: Color,
    pub animation_position: Point2,
    pub animation_rotation: f64,
    pub state: AnimationEnum,
}

impl State for MeleeAnimation {
    type StateEnum = AnimationEnum;
    fn change_state(&mut self, new_state: Self::StateEnum) {
        match [&self.state, &new_state] {
            [AnimationEnum::Active, AnimationEnum::Finished] => {
                self.state = new_state;
            },
            [AnimationEnum::Finished, _] => {
                self.state = new_state;
            },
            [AnimationEnum::Ready, _] => {
                self.state = new_state;
            },
            _ => ()
        }
    }
}

const TILE_SIZE: f64 = 20.0;
const PLAYER_SIZE: f64 = 16.0;
const PLAYER_RADIUS: f64 = PLAYER_SIZE/2.0;
const BEACON_SIZE: f64 = 18.0;
const ENEMY_SIZE: f64 = 16.0;
const ENEMY_RADIUS: f64 = ENEMY_SIZE/2.0;
const DROP_SIZE: f64 = TILE_SIZE / 2.0;

const FLOOR_COLOR: Color = [0.2, 0.13, 0.08, 1.0];
const WALL_COLOR: Color = [0.3, 0.3, 0.2, 1.0];
const SPAWNER_COLOR: Color = [0.4, 0.06, 0.0, 1.0];
const BEACON_COLOR: Color = [0.88, 0.68, 0.1, 1.0];
const RESOURCE_COLOR: Color = BEACON_COLOR;
const PLAYER_COLOR: Color = [0.75, 0.12, 0.08,1.0];
const ENEMY_COLOR: Color = [0.04, 0.13, 0.27, 1.0];
const ERROR_COLOR: Color = [1.0, 0.0, 0.0, 1.0];
const TEXT_COLOR: Color = [1.0, 1.0, 1.0, 1.0,];

const ANIMATION_COLOR: Color = [0.5, 0.5, 0.5 ,1.0];
const PLAYER_ATTACK_ANIMATION: MeleeAnimation = MeleeAnimation 
    {
        animation_width: PLAYER_SIZE *1.5,
        animation_height: PLAYER_SIZE / 3.0,
        animation_color: ANIMATION_COLOR,
        animation_position: Point2 {x: 0.0, y: 0.0},
        animation_rotation: 0.0,
        state: AnimationEnum::Ready,
    };


/// A structure to encapsulate all the settings used by the GameView to display
/// the GameModel
pub struct GameViewSettings {

    pub tile_size: f64,
    pub floor_color: Color,
    pub wall_color: Color,
    pub spawner_color: Color,
    pub beacon_color: Color,
    pub beacon_size: f64,
    pub player_size: f64,
    pub player_radius: f64,
    pub player_color: Color,
    pub player_attack_animation: MeleeAnimation,
    pub enemy_color: Color,
    pub enemy_size: f64,
    pub enemy_radius: f64,
    pub drop_size: f64,
    pub resource_color: Color,
    pub error_color: Color,
    
}

impl GameViewSettings {
    
    fn new() -> Self {
        
        Self {  
            tile_size: TILE_SIZE,
            floor_color: FLOOR_COLOR,
            wall_color: WALL_COLOR,
            spawner_color: SPAWNER_COLOR,
            beacon_color: BEACON_COLOR,
            beacon_size: BEACON_SIZE,
            player_size: PLAYER_SIZE,
            player_radius: PLAYER_RADIUS,
            player_color: PLAYER_COLOR,
            player_attack_animation: PLAYER_ATTACK_ANIMATION,
            enemy_color: ENEMY_COLOR,
            enemy_size: ENEMY_SIZE,
            enemy_radius: ENEMY_RADIUS,
            drop_size: DROP_SIZE,
            resource_color: RESOURCE_COLOR,
            error_color: ERROR_COLOR
        }

    }

}

/// A structure responsible for drawing the GameModel.
pub struct GameView {
    pub settings: GameViewSettings,
}

impl GameView {
    
    /// Creates a new GameView
    pub fn new() -> Self {
        Self { settings: GameViewSettings::new() }
    }

    /// Public function to translate a MapIdx to a Point2. This allows for the
    /// size of the tiles to change while leaving all the game logic the same.
    /// Some functions in the GameModel use this function.
    pub fn map_idx_to_point2(idx: MapIdx) -> Point2 {

        Point2 {x: idx.x as f64 * TILE_SIZE, y: idx.y as f64 * TILE_SIZE}

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
    fn draw_player<G: Graphics>(&mut self, model: &GameModel, c: &Context, g: &mut G) {
        // Draw the player
        model.player.shape.draw(c, g);
        // Draw the player's attack anmiation if in Active state. 
        match self.settings.player_attack_animation.state {
            AnimationEnum::Active => {
                model.player.attack.shape.draw(c,g);

            },
            _ => ()
        }
        
    }

    /// Draws the GameModel's Beacon
    fn draw_beacon<G: Graphics>(&mut self, model: &GameModel, c: &Context, g: &mut G) {
        model.beacon.shape.draw(c, g);
    }

    /// Draws each enemy in the GameModel enemy list
    fn draw_enemies<G: Graphics>(&mut self, model: &GameModel, c: &Context, g: &mut G) {
        for enemy in model.enemies.iter() {
            enemy.shape.draw(c,g);
        }
    }

    /// Draws each resource in the GameModels resource list
    fn draw_resources<G: Graphics>(&mut self, model: &GameModel, c: &Context, g: &mut G) {

        for resource in model.resources.iter() {

            resource.shape.draw(c,g);

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
            x: (model.level.width - 1) as f64 * self.settings.tile_size,
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