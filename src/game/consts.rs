use graphics::types::Color;
use crate::level::MapIdx;
use crate::misc::point2d::Point2;
use std::f64;

pub const PI: f64 = f64::consts::PI;
pub const INF: f64 = f64::INFINITY;

pub const TILE_SIZE: f64 = 20.0;
pub const FLOOR_COLOR: Color = [0.2, 0.13, 0.08, 1.0];
pub const WALL_COLOR: Color = [0.3, 0.3, 0.2, 1.0];
pub const SPAWNER_COLOR: Color = [0.4, 0.06, 0.0, 1.0];
pub const ERROR_COLOR: Color = [1.0, 0.0, 0.0, 1.0];

pub const PLAYER_SIZE: f64 = 16.0;
pub const PLAYER_RADIUS: f64 = PLAYER_SIZE/2.0;
pub const PLAYER_STARTING_HEALTH: i32 = 10000;
pub const PLAYER_SPEED: f64 = 0.1;
pub const PLAYER_COLOR: Color = [0.75, 0.12, 0.08,1.0];

pub const PLAYER_ATTACK_WIDTH: f64 = PLAYER_SIZE * 1.5;
pub const PLAYER_ATTACK_HEIGHT: f64 = PLAYER_SIZE / 3.0;
pub const PLAYER_ATTACK_COLOR: Color = [0.5, 0.5, 0.5 ,1.0]; 

pub const ENEMY_SIZE: f64 = 16.0;
pub const ENEMY_RADIUS: f64 = ENEMY_SIZE/2.0;
pub const ENEMY_COLOR: Color = [0.04, 0.13, 0.27, 1.0];
pub const ENEMY_SPEED: f64 = 0.1;

pub const DROP_SIZE: f64 = TILE_SIZE / 2.0;
pub const DROP_ROTATION_SPEED: f64 = -0.01;
pub const RESOURCE_COLOR: Color = BEACON_COLOR;

pub const BEACON_SIZE: f64 = 18.0;
pub const BEACON_COLOR: Color = [0.88, 0.68, 0.1, 1.0];
pub const BEACON_ROTATION_SPEED: f64 = 0.01;
pub const BEACON_STARTING_HEALTH: i32 = 10000;

pub const TOWER_COLOR: Color = [0.33, 0.33, 0.33, 1.0];
pub const TOWER_SIZE: f64 = PLAYER_SIZE;
pub const TOWER_RADIUS: f64 = TOWER_SIZE / 2.0;
pub const TOWER_CANNON_COLOR: Color = PLAYER_ATTACK_COLOR;
pub const TOWER_CANNON_WIDTH: f64 = TOWER_SIZE;
pub const TOWER_CANNON_HEIGHT: f64 = TOWER_SIZE / 3.0;

pub fn map_idx_to_point2(idx: MapIdx) -> Point2 {

    Point2 {x: idx.x as f64 * TILE_SIZE, y: idx.y as f64 * TILE_SIZE}

}