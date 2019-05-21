#![deny(missing_docs)]

//! A roguelike tower-defense game.
extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate rust_game;

use rust_game::traits::{Model, View, Controller};
use rust_game::level::{Level,LevelController,LevelView, LevelViewSettings};
use rust_game::level::{WINDOW_WIDTH, WINDOW_HEIGHT};
use rust_game::entity::player::{Player, PlayerController, PlayerView, PlayerViewSettings};
use rust_game::misc::random;
use rust_game::input;

use piston::window::WindowSettings;
use piston::event_loop::{Events, EventSettings};
use piston::input::RenderEvent;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};

fn main() {

    let config = input::handle_init_input();
    let seed = random::create_seed(config.debug);
    let lvl: Level = Level::new(seed);
    let mut lvlctrl: LevelController = LevelController::new(lvl);
    lvlctrl.print_level();
    let settings = LevelViewSettings::new();
    let lvlview = LevelView::new(settings);

    let player_spawn = lvlctrl.find_player_spawn();
    let player_spawn = lvlview.convert_idx(player_spawn);
    let player = Player::new(player_spawn);
    let player_controller = PlayerController::new(player);

    let player_settings = PlayerViewSettings::new();
    let player_view = PlayerView::new(player_settings);

    let opengl = OpenGL::V3_2;

    let window_settings = WindowSettings::new("Game", [WINDOW_WIDTH, WINDOW_HEIGHT]).opengl(opengl).exit_on_esc(true);
    let mut window: GlutinWindow = window_settings.build().expect("Couldn't create window!");
    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args(){
            gl.draw(args.viewport(), |c, g| {
                use graphics::{clear};

                clear([1.0; 4], g);
                lvlview.draw(&lvlctrl, &c, g);
                player_view.draw(&player_controller, &c, g)
            });
        }
    }
}