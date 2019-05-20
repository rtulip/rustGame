#![deny(missing_docs)]

//! A roguelike tower-defense game.
extern crate piston;
extern crate glutin_window;
extern crate rust_game;

use rust_game::level::{Level,LevelController,LevelView, LevelViewSettings};
use rust_game::misc::random;
use rust_game::input;

use piston::window::WindowSettings;
use piston::event_loop::{Events, EventSettings};
use glutin_window::GlutinWindow;

fn main() {

    let config = input::handle_init_input();
    let seed = random::create_seed(config.debug);
    let lvl: Level = Level::new(seed);
    let lvlctr: LevelController = LevelController::new(lvl);
    lvlctr.print_level();

    let settings = LevelViewSettings::new();
    let lvlview = LevelView::new(settings);

    let window_settings = WindowSettings::new("Game", [512;2]).exit_on_esc(true);
    let mut window: GlutinWindow = window_settings.build().expect("Couldn't create window!");
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {

    }
}