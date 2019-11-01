extern crate rust_game;
use rust_game::levels::Level;
use rust_game::math::random::create_seed;

pub fn setup_level() -> Level {
    let seed = create_seed(true);
    Level::new(seed)
}
