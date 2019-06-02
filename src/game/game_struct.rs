use crate::game::GameController;
use crate::math::random::Seed;
use crate::game::consts::{
    OPEN_GL_VERSION,
    WINDOW_HEIGHT,
    WINDOW_WIDTH,
    PI,
    INF,
    MIN
};

use piston::window::WindowSettings;
use piston::event_loop::{Events, EventSettings};
use piston::input::{RenderEvent, MouseCursorEvent};
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};

use crate::traits::draw::{Draw, GenericShape, ShapeVariant};
use crate::math::{Point2, Vec2};

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

    /// A function to start the game loop.
    pub fn run(&mut self) {
        let mut window: GlutinWindow = self.window_settings.build().expect("Couldn't create window!");
        let mut events = Events::new(EventSettings::new());
        let mut gl = GlGraphics::new(self.opengl);
        
        let c1 = [0.0, 0.0, 1.0, 1.0];
        let c2 = [1.0, 0.0, 0.0, 1.0];

        let width = 100.0;
        let height = width/2.0;
        let pos = Point2{x: WINDOW_WIDTH / 2.0, y: WINDOW_HEIGHT / 2.0};
        let rot = 0.0;
        let offset = Point2{x: 0.0, y: 0.0};

        let mut s1 = GenericShape::new(
            ShapeVariant::Rect{
                width: width,
                height: height,
            }, 
            c1, 
            pos
        );
        s1.set_rotation(rot);
        s1.set_offset(offset);

        let rot = PI/2.0;
        let offset = Point2{x: 0.0, y: 0.0};

        let mut s2 = GenericShape::new(
            ShapeVariant::Rect{
                width: width,
                height: height,
            }, 
            c1, 
            pos
        );
        s2.set_rotation(rot);
        s2.set_offset(offset);

        let mut v1 = GenericShape::new(ShapeVariant::Rect{width: WINDOW_WIDTH, height: 3.0}, [0.0,0.0,0.0,1.0], s1.get_position());
        let mut n1 = GenericShape::new(ShapeVariant::Rect{width: WINDOW_WIDTH, height: 3.0}, [0.0,0.0,0.0,1.0], s1.get_position());

        v1.set_offset(Point2{x: -WINDOW_WIDTH/ 2.0, y: 0.0});
        n1.set_offset(Point2{x: -WINDOW_WIDTH/ 2.0, y: 0.0});

        let mut p1 = GenericShape::new(ShapeVariant::Rect{width: 5.0, height: 5.0}, c2, s1.get_position());
        let mut p2 = GenericShape::new(ShapeVariant::Rect{width: 5.0, height: 5.0}, c2, s1.get_position());
        let mut p3 = GenericShape::new(ShapeVariant::Rect{width: 5.0, height: 5.0}, c2, s1.get_position());
        let mut p4 = GenericShape::new(ShapeVariant::Rect{width: 5.0, height: 5.0}, c2, s1.get_position());
        
        if let Some(rot) = s1.get_rotation() {
            v1.set_rotation(rot);
            n1.set_rotation(rot + PI/2.0)
        }

        while let Some(e) = events.next(&mut window) {
            // if !self.controller.check_state() {
            //     break;
            // }
            // self.controller.handle_event(&e);
            
            if let Some(args) = e.mouse_cursor_args() {
                s2.set_position(Point2{x: args[0], y: args[1]});
                if check_collision(s1,s2,&mut p1, &mut p2, &mut p3, &mut p4) {
                    s2.set_color(c2);
                } else {
                    s2.set_color(c1);
                }

                // if let Some(rot) = s1.get_rotation(){
                //     let line = Vec2::new(rot.cos(), rot.sin());
                //     let cursor_point = Point2{x: args[0], y: args[1]};
                //     let proj = project(Vec2::new_from_point(s1.get_position() - cursor_point), line);
                //     p1.set_position(proj + s1.get_position());

                //     let normal_line = line.normal_unit();
                //     let proj = project(Vec2::new_from_point(s1.get_position() - cursor_point), normal_line);
                //     p2.set_position(proj + s1.get_position()); 
                // }

            }

            if let Some(args) = e.render_args() {
                gl.draw(args.viewport(), |c, g| {
                    use graphics::{clear};
                    
                    clear([1.0; 4], g);
                    // self.controller.view.draw(&self.controller.model, &c, g);
                    s1.draw(&c, g);
                    s2.draw(&c, g);
                    v1.draw(&c, g);
                    n1.draw(&c, g);
                    p1.draw(&c, g);
                    p2.draw(&c, g);
                    p3.draw(&c, g);
                    p4.draw(&c, g);
                })
            }
        }
    }

}

fn check_collision(s1: GenericShape, s2: GenericShape, p1: &mut GenericShape, p2: &mut GenericShape, p3: &mut GenericShape, p4: &mut GenericShape) -> bool {
    
    if let Some(s1_corners) = s1.get_corners() {

        if let Some(s2_corners) = s2.get_corners() {
            
            let mut min1_x = Point2 {
                x: INF, 
                y: INF,
            };

            let mut max1_x = Point2 {
                x: MIN,
                y: MIN,
            };

            let mut min1_y = Point2 {
                x: INF, 
                y: INF,
            };

            let mut max1_y = Point2 {
                x: MIN,
                y: MIN,
            };

            let mut rad = 0.0;
            if let Some(rot) = s1.get_rotation(){
                rad = rot; 
            }
            let line = Vec2::new(rad.cos(), rad.sin());
            let norm = line.normal_unit();
            
            for point in s2_corners {
                let v = Vec2::new_from_point(s1.get_position() - point);
                let py = s1.get_position() + project(v, line);
                let px = s1.get_position() + project(v, norm);

                if py.x < min1_y.x {
                    min1_y = py;
                } else if py.x > max1_y.x {
                    max1_y = py;
                }

                if px.y < min1_x.y {
                    min1_x = px;
                } else if px.y > max1_x.y {
                    max1_x = px;
                }

            }

            p1.set_position(max1_y);
            p2.set_position(min1_y);
            p3.set_position(max1_x);
            p4.set_position(min1_x);

            false

        } else {
            false 
        }

    } else {
        false
    }


}

fn project(vec: Vec2, line: Vec2) -> Point2 {

    let norm = Vec2::new_unit(line.x, line.y);
    let c = Vec2::dot_product(vec, norm) / Vec2::dot_product(norm, norm);

    Point2{x: norm.x * -c, y: norm.y * -c}

}