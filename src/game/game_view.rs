use crate::game::GameModel;
use crate::level::{MapIdx, Level};
use crate::misc::point2d::Point2;
use crate::traits::shape::Shape;
use crate::traits::state::State;
use crate::entity::{tile, attack};
use graphics::{Context, Graphics,Transformed};
use graphics::types::Color;
use std::f64;

pub enum AnimationEnum {
    Active,
    Finished,
    Ready,
}

pub struct MeleeAnimation {
    pub animation_width: f64,
    pub animation_height: f64,
    pub animation_color: Color,
    pub animation_position: [f64;2],
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

const FLOOR_COLOR: Color = [0.2, 0.13, 0.08, 1.0];
const WALL_COLOR: Color = [0.3, 0.3, 0.2, 1.0];
const BEACON_COLOR: Color = [0.88, 0.68, 0.1, 1.0];
const PLAYER_COLOR: Color = [0.75, 0.12, 0.08,1.0];
const ENEMY_COLOR: Color = [0.04, 0.13, 0.27, 1.0];
const ERROR_COLOR: Color = [1.0, 0.0, 0.0, 1.0];

const ANIMATION_COLOR: Color = [0.5, 0.5, 0.5 ,1.0];
const PLAYER_ATTACK_ANIMATION: MeleeAnimation = MeleeAnimation 
    {
        animation_width: PLAYER_SIZE,
        animation_height: PLAYER_SIZE / 3.0,
        animation_color: ANIMATION_COLOR,
        animation_position: [0.0, 0.0],
        animation_rotation: 0.0,
        state: AnimationEnum::Ready,
    };


/// GameViewSettings 
/// 
/// A structure containing the needed information to draw each Entity and Shape
/// in the game.
pub struct GameViewSettings {

    pub tile_size: f64,
    pub floor_color: Color,
    pub wall_color: Color,
    pub beacon_color: Color,
    pub beacon_size: f64,
    pub player_size: f64,
    pub player_radius: f64,
    pub player_color: Color,
    pub player_attack_animation: MeleeAnimation,
    pub enemy_color: Color,
    pub enemy_size: f64,
    pub enemy_radius: f64,
    pub error_color: Color,
    
}

impl GameViewSettings {
    
    fn new() -> Self {
        
        Self {  
            tile_size: TILE_SIZE,
            floor_color: FLOOR_COLOR,
            wall_color: WALL_COLOR,
            beacon_color: BEACON_COLOR,
            beacon_size: BEACON_SIZE,
            player_size: PLAYER_SIZE,
            player_radius: PLAYER_RADIUS,
            player_color: PLAYER_COLOR,
            player_attack_animation: PLAYER_ATTACK_ANIMATION,
            enemy_color: ENEMY_COLOR,
            enemy_size: ENEMY_SIZE,
            enemy_radius: ENEMY_RADIUS,

            error_color: ERROR_COLOR
        }

    }

}

/// GameView
/// 
/// A structure to handle all the graphics in the game
pub struct GameView {
    pub settings: GameViewSettings,
}

impl GameView {
    
    pub fn new() -> Self {
        Self { settings: GameViewSettings::new() }
    }

    pub fn MapIdx_to_Point2(&self, idx: MapIdx) -> Point2 {

        Point2 {x: idx.x as f64 * self.settings.tile_size, y: idx.y as f64 * self.settings.tile_size}

    }

    /// draw()
    /// 
    /// args:
    ///     model: &GameModel: A reference to the GameModel to draw
    ///     c: &Context: The graphics Context
    ///     g: &mut Graphics: A mutable reference to the Graphics
    /// 
    /// Draws the GameModel
    pub fn draw<G: Graphics>(&mut self, model: &GameModel, c: &Context, g: &mut G) {
        self.draw_level(model, c, g);
        self.draw_player(model, c, g);
        self.draw_beacon(model, c, g);
        self.draw_enemies(model, c, g);
    }

    /// draw_level()
    /// 
    /// args:
    ///     model: &GameModel: A reference to the GameModel for which the level
    ///         is to be drawn
    ///     c: &Context: The graphics Context
    ///     g: &mut Graphics: A mutable reference to the Graphics
    /// 
    /// Draws the Level of the GameModel by looping through each tile in the 
    /// Map.
    fn draw_level<G: Graphics>(&self, model: &GameModel, c: &Context, g: &mut G) {
        let settings = &self.settings;
        for h in 0..model.level.height {
            for w in 0..model.level.width {
                match model.level.map.get(&MapIdx::new(w,h)){
                    Some(tile::Tile::Floor) => {
                        let p = self.MapIdx_to_Point2(MapIdx::new(w, h));
                        tile::Tile::Floor.get_shape().draw(settings.floor_color,
                                                     p.x, 
                                                     p.y, 
                                                     settings.tile_size,
                                                     settings.tile_size,
                                                     0.0, 
                                                     c, 
                                                     g)
                    },
                    Some(tile::Tile::Wall) => {
                        let p = self.MapIdx_to_Point2(MapIdx::new(w, h));
                        tile::Tile::Floor.get_shape().draw(settings.wall_color,
                                                     p.x, 
                                                     p.y, 
                                                     settings.tile_size,
                                                     settings.tile_size,
                                                     0.0, 
                                                     c, 
                                                     g)
                    },
                    _ => {
                        let p = self.MapIdx_to_Point2(MapIdx::new(w, h));
                        tile::Tile::Floor.get_shape().draw(settings.error_color,
                                                     p.x, 
                                                     p.y, 
                                                     settings.tile_size,
                                                     settings.tile_size,
                                                     0.0,
                                                     c, 
                                                     g)
                    },
                };
            }
        }

    }

    /// draw_player()
    /// 
    /// args:
    ///     model: &GameModel: A reference to the GameModel for which the 
    ///         player is to be drawn
    ///     c: &Context: The graphics Context
    ///     g: &mut Graphics: A mutable reference to the Graphics
    /// 
    /// Draws the Player of the GameModel. If the player is attacking, the 
    /// Player's sword is drawn as well.
    fn draw_player<G: Graphics>(&mut self, model: &GameModel, c: &Context, g: &mut G) {
        model.player.get_shape().draw(
            self.settings.player_color,
            self.settings.player_radius,
            model.player.position.x,
            model.player.position.y,
            self.settings.player_size,
            self.settings.player_size,
            c,
            g
        );
        match self.settings.player_attack_animation.state {
            AnimationEnum::Active => {
                let shape = attack::Attack {};
                let shape = shape.get_shape();
                let pi = f64::consts::PI;
                let dir = &model.player.direction;
                let mut rad = model.player.direction.y / model.player.direction.x;
                rad = rad.atan();
                
                match [dir.x < 0.0, dir.y < 0.0] {
                    [true, true] => rad = pi * 2.0 - rad,
                    [true, false] => rad = rad * -1.0,
                    [false, true] => rad = pi + rad * -1.0,
                    [false, false] => rad = pi - rad
                }

                rad = pi - rad;

                let player_center = [model.player.position.x + self.settings.player_size / 2.0,
                                     model.player.position.y + self.settings.player_size / 2.0];
                
                let tangent_point = [player_center[0] + self.settings.player_radius * rad.cos(),
                                     player_center[1] + self.settings.player_radius * rad.sin()];

                let animation_corner = [tangent_point[0] - self.settings.player_attack_animation.animation_height / 2.0,
                                        tangent_point[1] - self.settings.player_attack_animation.animation_height / 2.0];
                
                shape.draw( self.settings.player_attack_animation.animation_color,
                            animation_corner[0],
                            animation_corner[1],
                            self.settings.player_attack_animation.animation_width,
                            self.settings.player_attack_animation.animation_height,
                            rad, 
                            c, 
                            g);                
                self.settings.player_attack_animation.animation_position = animation_corner;
                self.settings.player_attack_animation.animation_rotation = rad;
            },
            _ => ()
        }
        
    }

    fn draw_beacon<G: Graphics>(&mut self, model: &GameModel, c: &Context, g: &mut G) {
        let p = self.MapIdx_to_Point2(MapIdx::new(model.beacon.position.x, model.beacon.position.y));
        model.beacon.get_shape().draw(
            self.settings.beacon_color,
            p.x, 
            p.y, 
            self.settings.beacon_size,
            self.settings.beacon_size,
            model.beacon.rotation, 
            c, 
            g);
    }

    fn draw_enemies<G: Graphics>(&mut self, model: &GameModel, c: &Context, g: &mut G) {
        for enemy in model.enemies.iter() {
            enemy.get_shape().draw(
                self.settings.enemy_color,
                self.settings.enemy_radius,
                enemy.position.x,
                enemy.position.y,
                self.settings.enemy_size,
                self.settings.enemy_size,
                c,
                g
            )
        }
    }
}