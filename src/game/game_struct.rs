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
use piston::input::{RenderEvent, MouseCursorEvent, MouseScrollEvent};
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
        let offset = Point2{x: -width / 2.0, y: -height / 2.0};

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

        let mut v1 = GenericShape::new(ShapeVariant::Rect{width: WINDOW_WIDTH, height: 3.0}, [0.0,0.0,0.0,1.0], s1.get_position());
        let mut n1 = GenericShape::new(ShapeVariant::Rect{width: WINDOW_WIDTH, height: 3.0}, [0.0,0.0,0.0,1.0], s1.get_position());
        v1.set_offset(Point2{x: -WINDOW_WIDTH/ 2.0, y: 0.0} + offset);
        n1.set_offset(Point2{x: -WINDOW_WIDTH/ 2.0, y: 0.0} + Point2{x: offset.y, y: -offset.x});

        let rot = -PI/3.56;
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

        let mut v2 = GenericShape::new(ShapeVariant::Rect{width: WINDOW_WIDTH, height: 3.0}, [0.5,0.5,0.5,1.0], s2.get_position());
        let mut n2 = GenericShape::new(ShapeVariant::Rect{width: WINDOW_WIDTH, height: 3.0}, [0.5,0.5,0.5,1.0], s2.get_position());
        v2.set_offset(Point2{x: -WINDOW_WIDTH/ 2.0, y: 0.0});
        n2.set_offset(Point2{x: -WINDOW_WIDTH/ 2.0, y: 0.0});

        let mut p1 = GenericShape::new(ShapeVariant::Rect{width: 5.0, height: 5.0}, c2, s1.get_position());
        let mut p2 = GenericShape::new(ShapeVariant::Rect{width: 5.0, height: 5.0}, c2, s1.get_position());
        let mut p3 = GenericShape::new(ShapeVariant::Rect{width: 5.0, height: 5.0}, [0.0,1.0,0.0,1.0], s1.get_position());
        let mut p4 = GenericShape::new(ShapeVariant::Rect{width: 5.0, height: 5.0}, [0.0,1.0,0.0,1.0], s1.get_position());
        
        if let Some(rot) = s1.get_rotation() {
            v1.set_rotation(rot);
            n1.set_rotation(rot + PI/2.0)
        }

        if let Some(rot) = s2.get_rotation() {
            v2.set_rotation(rot);
            n2.set_rotation(rot + PI/2.0)
        }

        while let Some(e) = events.next(&mut window) {
            // if !self.controller.check_state() {
            //     break;
            // }
            // self.controller.handle_event(&e);
            
            if let Some(args) = e.mouse_cursor_args() {
                s2.set_position(Point2{x: args[0], y: args[1]});
                v2.set_position(s2.get_position());
                n2.set_position(s2.get_position());
                if check_collision(s1,s2,&mut p1, &mut p2, &mut p3, &mut p4) {
                    s2.set_color(c2);
                } else {
                    s2.set_color(c1);
                }

            }

            if let Some(args) = e.mouse_scroll_args() {

                s1.update(Point2{x: 0.0, y: 0.0}, Some(PI/12.0 * -args[1]));

                if let Some(rot) = s1.get_rotation() {
                    v1.set_rotation(rot);
                    n1.set_rotation(rot + PI/2.0)
                }

            }

            if let Some(args) = e.render_args() {
                gl.draw(args.viewport(), |c, g| {
                    use graphics::{clear};
                    
                    clear([1.0; 4], g);
                    // self.controller.view.draw(&self.controller.model, &c, g);
                    v1.draw(&c, g);
                    n1.draw(&c, g);
                    s1.draw(&c, g);
                    s2.draw(&c, g);
                    v2.draw(&c, g);
                    n2.draw(&c, g);
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
            
            let mut proj_x1: Vec<Point2> = Vec::new();
            let mut proj_y1: Vec<Point2> = Vec::new();
            let mut proj_x2: Vec<Point2> = Vec::new();
            let mut proj_y2: Vec<Point2> = Vec::new();

            let mut rad = 0.0;
            if let Some(rot) = s1.get_rotation(){
                rad = rot; 
            }
            let line = Vec2::new(rad.cos(), rad.sin());
            let norm = line.normal_unit();
            for point in s2_corners.iter() {
                let v = Vec2::new_from_point(s1.get_position() - *point);
                let py = s1.get_position() + project(v, line);
                let px = s1.get_position() + project(v, norm);
                proj_x1.push(px);
                proj_y1.push(py);
            }

            let mut rad = 0.0;
            if let Some(rot) = s2.get_rotation(){
                rad = rot; 
            }
            let line = Vec2::new(rad.cos(), rad.sin());
            let norm = line.normal_unit();
            for point in s1_corners.iter() {
                let v = Vec2::new_from_point(s2.get_position() - *point);
                let py = s2.get_position() + project(v, line);
                let px = s2.get_position() + project(v, norm);
                proj_x2.push(px);
                proj_y2.push(py);
            }

            let mut ls_x1 = [proj_x1[0], proj_x1[0]];
            let mut ls_y1 = [proj_y1[0], proj_y1[0]];
            let mut ls_x2 = [proj_x1[0], proj_x1[0]];
            let mut ls_y2 = [proj_y1[0], proj_y1[0]];
            for i in 0..4 {
                for j in i+1..4 {

                    let d = proj_x1[i] - proj_x1[j];
                    let d = Vec2::new_from_point(d);

                    let ls = ls_x1[0] - ls_x1[1];
                    let ls = Vec2::new_from_point(ls);

                    if Vec2::dot_product(d,d) > Vec2::dot_product(ls, ls) {
                        ls_x1[0] = proj_x1[i];
                        ls_x1[1] = proj_x1[j];
                    }

                    let d = proj_y1[i] - proj_y1[j];
                    let d = Vec2::new_from_point(d);

                    let ls = ls_y1[0] - ls_y1[1];
                    let ls = Vec2::new_from_point(ls);

                    if Vec2::dot_product(d,d) > Vec2::dot_product(ls, ls) {
                        ls_y1[0] = proj_y1[i];
                        ls_y1[1] = proj_y1[j];
                    }

                    let d = proj_x2[i] - proj_x2[j];
                    let d = Vec2::new_from_point(d);

                    let ls = ls_x2[0] - ls_x2[1];
                    let ls = Vec2::new_from_point(ls);

                    if Vec2::dot_product(d,d) > Vec2::dot_product(ls, ls) {
                        ls_x2[0] = proj_x2[i];
                        ls_x2[1] = proj_x2[j];
                    }

                    let d = proj_y2[i] - proj_y2[j];
                    let d = Vec2::new_from_point(d);

                    let ls = ls_y2[0] - ls_y2[1];
                    let ls = Vec2::new_from_point(ls);

                    if Vec2::dot_product(d,d) > Vec2::dot_product(ls, ls) {
                        ls_y2[0] = proj_y2[i];
                        ls_y2[1] = proj_y2[j];
                    }
                }
            }

            let q1 = ls_x1[0];
            let s1 = ls_x1[1];

            let q2 = ls_y1[0];
            let s2 = ls_y1[1];

            let q3 = ls_x2[0];
            let s3 = ls_x2[1];

            let q4 = ls_y2[0];
            let s4 = ls_y2[1];

            p1.set_position(q3);
            p2.set_position(s3);
            p3.set_position(q4);
            p4.set_position(s4);

            let dist_ls_x1 = Vec2::new_from_point(s2 - q2);
            let dist_ls_x1 = Vec2::dot_product(dist_ls_x1, dist_ls_x1);
            let dist_p_r11 = Vec2::new_from_point(s1_corners[1] - s1_corners[0]);
            let dist_p_r11 = Vec2::dot_product(dist_p_r11, dist_p_r11);
            
            let dist_ls_y1 = Vec2::new_from_point(s1 - q1);
            let dist_ls_y1 = Vec2::dot_product(dist_ls_y1, dist_ls_y1);
            let dist_p_r12 = Vec2::new_from_point(s1_corners[2] - s1_corners[0]);
            let dist_p_r12 = Vec2::dot_product(dist_p_r12, dist_p_r12);

            let dist_ls_x2 = Vec2::new_from_point(s4 - q4);
            let dist_ls_x2 = Vec2::dot_product(dist_ls_x2, dist_ls_x2);
            let dist_p_r21 = Vec2::new_from_point(s2_corners[1] - s2_corners[0]);
            let dist_p_r21 = Vec2::dot_product(dist_p_r21, dist_p_r21);
            
            let dist_ls_y2 = Vec2::new_from_point(s3 - q3);
            let dist_ls_y2 = Vec2::dot_product(dist_ls_y2, dist_ls_y2);
            let dist_p_r22 = Vec2::new_from_point(s2_corners[2] - s2_corners[0]);
            let dist_p_r22 = Vec2::dot_product(dist_p_r22, dist_p_r22);
            
            let max_dist_x1 = find_max_dist(s1_corners[0], s1_corners[1], q2, s2);
            let max_dist_y1 = find_max_dist(s1_corners[0], s1_corners[2], q1, s1);
            let max_dist_x2 = find_max_dist(s2_corners[0], s2_corners[1], q4, s4);
            let max_dist_y2 = find_max_dist(s2_corners[0], s2_corners[2], q3, s3);


            if max_dist_x1.sqrt() <= dist_ls_x1.sqrt() + dist_p_r11.sqrt() &&
               max_dist_y1.sqrt() <= dist_ls_y1.sqrt() + dist_p_r12.sqrt() &&
               max_dist_x2.sqrt() <= dist_ls_x2.sqrt() + dist_p_r21.sqrt() &&
               max_dist_y2.sqrt() <= dist_ls_y2.sqrt() + dist_p_r22.sqrt() {
                true
            } else {
                false
            }

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

fn find_max_dist(p1: Point2, p2: Point2, q1: Point2, q2: Point2) -> f64 {

    let p = vec![p1,p2];
    let q = vec![q1,q2];

    let mut max_dist = 0.0;
    for i in 0..2 {
        for j in 0..2 {

            let v = Vec2::new_from_point(p[i] - q[j]);
            let d = Vec2::dot_product(v, v);
            if d > max_dist {
                max_dist = d;
            }

        }
    }

    max_dist
}
