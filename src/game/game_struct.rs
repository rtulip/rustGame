use crate::game::{GameController, GameState};
use crate::misc::random::Seed;

use piston::window::WindowSettings;
use piston::event_loop::{Events, EventSettings};
use piston::input::{RenderEvent, UpdateEvent};
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics, Filter, GlyphCache, TextureSettings};

const WINDOW_WIDTH: f64 = 1000.0;
const WINDOW_HEIGHT: f64 = 1000.0;
const OPEN_GL_VERSION: OpenGL = OpenGL::V3_2;

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
    
    pub fn new(seed: Seed) -> Self {
        Self { 
            opengl: OPEN_GL_VERSION,
            window_settings: WindowSettings::new("Game", [WINDOW_WIDTH, WINDOW_HEIGHT]).opengl(OPEN_GL_VERSION).exit_on_esc(true),
            controller: GameController::new(seed),
        }
    }

    /// run()
    /// 
    /// A function to start the game loop.
    pub fn run(&mut self) {
        let mut window: GlutinWindow = self.window_settings.build().expect("Couldn't create window!");
        let mut events = Events::new(EventSettings::new());
        let mut gl = GlGraphics::new(self.opengl);
        let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        
        while let Some(e) = events.next(&mut window) {
            match self.controller.state {
                GameState::Finished => break,
                _ => (),
            }
            
            self.controller.handle_event(&e);
            
            if let Some(args) = e.render_args() {
                let ref mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
                        .expect("Could not load font");
                gl.draw(args.viewport(), |c, g| {
                    use graphics::{clear};
                    
                    clear([1.0; 4], g);
                    self.controller.view.draw(&self.controller.model, glyphs, &c, g)
                });
            }
        }
    }

}