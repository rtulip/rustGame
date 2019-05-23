use crate::game::{GameController};
use crate::misc::random::Seed;
use piston::window::WindowSettings;
use piston::event_loop::{Events, EventSettings};
use piston::input::RenderEvent;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};

const WINDOW_WIDTH: f64 = 1000.0;
const WINDOW_HEIGHT: f64 = 1000.0;
const OPEN_GL_VERSION: OpenGL = OpenGL::V3_2;

pub struct Game {
    opengl: OpenGL,
    window_settings: WindowSettings,
    controller: GameController,
    // view: GameView,
}

impl Game {
    
    pub fn new(seed: Seed) -> Self {
        Self { opengl: OPEN_GL_VERSION,
               window_settings: WindowSettings::new("Game", [WINDOW_WIDTH, WINDOW_HEIGHT]).opengl(OPEN_GL_VERSION).exit_on_esc(true),
               controller: GameController::new(seed)
             }
    }

    pub fn run(&self) {
        let mut window: GlutinWindow = self.window_settings.build().expect("Couldn't create window!");
        let mut events = Events::new(EventSettings::new());
        let mut gl = GlGraphics::new(self.opengl);
        
        while let Some(e) = events.next(&mut window) {
            if let Some(args) = e.render_args(){
                gl.draw(args.viewport(), |c, g| {
                    use graphics::{clear};

                    clear([1.0; 4], g);
                });
            }
        }
    }

}