use crate::game::consts::{OPEN_GL_VERSION, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::game::GameController;
use crate::input;
use crate::math::random;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::WindowSettings;

/// Game
///
/// A structure to enclose the entirety of the Game Logic
/// The Game struct starts the game loop, which keeps the game going
/// A game has a GameController which controlls all the game Logic and graphics
pub struct Game {
    opengl: OpenGL,
    window_settings: WindowSettings,
    controller: GameController,
}

impl Game {
    pub fn new() -> Self {
        // Parse command line for input commands
        let config = input::handle_init_input();
        // Create the seed used for the run
        let mut seed = random::create_seed(config.debug);
        let controller: GameController;
        loop {
            if let Some(c) = GameController::new(seed) {
                controller = c;
                break;
            } else if config.debug {
                panic!("Failed to create game controller with debug flag");
            } else {
                seed = random::create_seed(false);
                println!("Had to reroll seed");
            }
        }

        Self {
            opengl: OPEN_GL_VERSION,
            window_settings: WindowSettings::new("Rust Game", [WINDOW_WIDTH, WINDOW_HEIGHT])
                .graphics_api(OPEN_GL_VERSION)
                .exit_on_esc(true),
            controller: controller,
        }
    }

    /// A function to start the game loop.
    pub fn run(&mut self) {
        let mut window: GlutinWindow = self
            .window_settings
            .build()
            .expect("Couldn't create window!");
        let mut events = Events::new(EventSettings::new());
        let mut gl = GlGraphics::new(self.opengl);

        while let Some(e) = events.next(&mut window) {
            if !self.controller.check_state() {
                break;
            }
            self.controller.handle_event(&e);

            if let Some(args) = e.render_args() {
                gl.draw(args.viewport(), |c, g| {
                    use graphics::clear;

                    clear([1.0; 4], g);
                    self.controller.view.draw(&self.controller.model, &c, g);
                })
            }
        }
    }
}
