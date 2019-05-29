use graphics::types::Color;
pub const PLAYER_SIZE: f64 = 16.0;
pub const PLAYER_RADIUS: f64 = PLAYER_SIZE/2.0;
pub const PLAYER_STARTING_HEALTH: i32 = 10000;
pub const PLAYER_SPEED: f64 = 0.1;
pub const PLAYER_COLOR: Color = [0.75, 0.12, 0.08,1.0];

pub const ENEMY_SIZE: f64 = 16.0;
pub const ENEMY_RADIUS: f64 = ENEMY_SIZE/2.0;
pub const ENEMY_COLOR: Color = [0.04, 0.13, 0.27, 1.0];
pub const ENEMY_SPEED: f64 = 0.1;

pub const DROP_SIZE: f64 = TILE_SIZE / 2.0;
pub const DROP_ROTATION_SPEED: f64 = -0.01;
pub const RESOURCE_COLOR: Color = BEACON_COLOR;

pub const BEACON_COLOR: Color = [0.88, 0.68, 0.1, 1.0];
pub const TILE_SIZE: f64 = 20.0;


