use crate::levels::map::MapIdx;
use crate::math::Point2;
use graphics::types::Color;
use opengl_graphics::OpenGL;
use std::f64;

pub const PI: f64 = f64::consts::PI;
pub const INF: f64 = f64::INFINITY;
pub const MIN: f64 = f64::MIN;

pub const WINDOW_WIDTH: f64 = 1000.0;
pub const WINDOW_HEIGHT: f64 = 1000.0;
pub const OPEN_GL_VERSION: OpenGL = OpenGL::V3_2;

pub const LEVEL_WIDTH: i32 = 50;
pub const LEVEL_HEIGHT: i32 = 50;
pub const LEVEL_GEN_ITERS: i32 = 5;

pub const TILE_SIZE: f64 = 20.0;
pub const FLOOR_COLOR: Color = [0.2, 0.13, 0.08, 1.0];
pub const WALL_COLOR: Color = [0.3, 0.3, 0.2, 1.0];
pub const SPAWNER_COLOR: Color = [0.4, 0.06, 0.0, 1.0];
pub const ERROR_COLOR: Color = [1.0, 0.0, 0.0, 1.0];

pub const PLAYER_SIZE: f64 = 16.0;
pub const PLAYER_RADIUS: f64 = PLAYER_SIZE / 2.0;
pub const PLAYER_STARTING_HEALTH: i32 = 10;
pub const PLAYER_SPEED: f64 = 7.5 * TILE_SIZE;
pub const PLAYER_COLOR: Color = [0.75, 0.12, 0.08, 1.0];

pub const PLAYER_ATTACK_WIDTH: f64 = PLAYER_SIZE * 1.5;
pub const PLAYER_ATTACK_HEIGHT: f64 = PLAYER_SIZE / 3.0;
pub const PLAYER_ATTACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

pub const ENEMY_SIZE: f64 = 16.0;
pub const ENEMY_RADIUS: f64 = ENEMY_SIZE / 2.0;
pub const ENEMY_COLOR: Color = [0.04, 0.13, 0.27, 1.0];
pub const ENEMY_SPEED: f64 = 4.0 * TILE_SIZE;

pub const DROP_SIZE: f64 = TILE_SIZE / 2.0;
pub const DROP_ROTATION_SPEED: f64 = -2.0 * PI;
pub const RESOURCE_COLOR: Color = BEACON_COLOR;

pub const BEACON_SIZE: f64 = 18.0;
pub const BEACON_COLOR: Color = [0.88, 0.68, 0.1, 1.0];
pub const BEACON_ROTATION_SPEED: f64 = 2.0 * PI;
pub const BEACON_STARTING_HEALTH: i32 = 10;

pub const TOWER_COLOR: Color = [0.33, 0.33, 0.33, 1.0];
pub const TOWER_SIZE: f64 = PLAYER_SIZE;
pub const TOWER_RADIUS: f64 = TOWER_SIZE / 2.0;
pub const TOWER_RANGE: f64 = TILE_SIZE * 4.0;
pub const TOWER_CANNON_COLOR: Color = PLAYER_ATTACK_COLOR;
pub const TOWER_CANNON_WIDTH: f64 = TOWER_SIZE;
pub const TOWER_CANNON_HEIGHT: f64 = TOWER_SIZE / 3.0;
pub const TOWER_ATTACK_COOLDOWN: f64 = 0.5;
pub const BULLET_WIDTH: f64 = TOWER_SIZE / 2.0;
pub const BULLET_HEIGHT: f64 = TOWER_CANNON_HEIGHT * 0.75;
pub const BULLET_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
pub const BULLET_SPEED: f64 = ENEMY_SPEED * 2.0;

pub const HEALTH_BAR_HEIGHT: f64 = 5.0;
pub const HEALTH_COLOR: Color = [0.0, 1.0, 0.0, 1.0];
pub const DAMAGE_COLOR: Color = [1.0, 0.0, 0.0, 1.0];

pub fn map_idx_to_point2(idx: MapIdx) -> Point2 {
    Point2 {
        x: idx.x as f64 * TILE_SIZE,
        y: idx.y as f64 * TILE_SIZE,
    }
}

pub fn point2_to_map_idx(p: Point2) -> MapIdx {
    MapIdx::new(
        (p.x / TILE_SIZE).floor() as i32,
        (p.y / TILE_SIZE).floor() as i32,
    )
}
