#![deny(missing_docs)]

//! A roguelike tower-defense game.
extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate rust_game;

use rust_game::game::Game;
use rust_game::input;
use rust_game::misc::random;

fn main() {

    let config = input::handle_init_input();
    let seed = random::create_seed(config.debug);

    let mut game = Game::new(seed);
    game.run();

}