#![deny(missing_docs)]

//! A roguelike tower-defense game.
extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate rust_game;

use rust_game::game::Game;

fn main() {

    let game = Game::new();
    game.run();

}