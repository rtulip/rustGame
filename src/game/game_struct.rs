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
            ShapeVariant::Circle{
                size: height,
                radius: height/2.0
            },
            c1, 
            pos
        );
        s1.set_rotation(rot);
        s1.set_offset(offset);

        let rot = -PI/3.56;
        let offset = Point2{x: -width / 2.0, y: -height / 2.0};

        let mut s2 = GenericShape::new(
            ShapeVariant::Rect{
                width: width,
                height: height
            },
            c1, 
            pos
        );
        s2.set_rotation(rot);
        s2.set_offset(offset);

        let mut point1 = GenericShape::new(
            ShapeVariant::Circle{
                size: 5.0,
                radius: 1.5,
            }, 
            [0.0,0.0,0.0,1.0],
            s1.get_position()
        );
        let mut point2 = GenericShape::new(
            ShapeVariant::Circle{
                size: 5.0,
                radius: 1.5,
            }, 
            [0.0,0.0,0.0,1.0],
            s1.get_position()
        );
        let mut point3 = GenericShape::new(
            ShapeVariant::Circle{
                size: 5.0,
                radius: 1.5,
            }, 
            [0.0,0.0,0.0,1.0],
            s1.get_position()
        );
        let mut point4 = GenericShape::new(
            ShapeVariant::Circle{
                size: 5.0,
                radius: 1.5,
            }, 
            [0.0,0.0,0.0,1.0],
            s1.get_position()
        );

        while let Some(e) = events.next(&mut window) {
            // if !self.controller.check_state() {
            //     break;
            // }
            // self.controller.handle_event(&e);
            
            if let Some(args) = e.mouse_cursor_args() {
                s2.set_position(Point2{x: args[0], y: args[1]});
                if check_collision(s1,s2, &mut point1, &mut point2, &mut point3, &mut point4) {
                    s2.set_color(c2);
                } else {
                    s2.set_color(c1);
                }

            }

            if let Some(args) = e.mouse_scroll_args() {

                s1.update(Point2{x: 0.0, y: 0.0}, Some(PI/12.0 * -args[1]));

            }

            if let Some(args) = e.render_args() {
                gl.draw(args.viewport(), |c, g| {
                    use graphics::{clear};
                    
                    clear([1.0; 4], g);
                    // self.controller.view.draw(&self.controller.model, &c, g);
                    s1.draw(&c, g);
                    s2.draw(&c, g);
                    point1.draw(&c, g);
                    point2.draw(&c, g);
                    point3.draw(&c, g);
                    point4.draw(&c, g);

                })
            }
        }
    }

}

fn check_collision(s1: GenericShape, s2: GenericShape, point1: &mut GenericShape,point2: &mut GenericShape,point3: &mut GenericShape,point4: &mut GenericShape) -> bool {
    
    if let Some(s1_corners) = s1.get_corners() {

        if let Some(s2_corners) = s2.get_corners() {
            
            let mut proj_x1: Vec<Point2> = Vec::new();
            let mut proj_y1: Vec<Point2> = Vec::new();
            let mut proj_x2: Vec<Point2> = Vec::new();
            let mut proj_y2: Vec<Point2> = Vec::new();

            let line = Vec2::new(s1_corners[1].x - s1_corners[0].x, s1_corners[1].y - s1_corners[0].y);
            let norm = line.normal_unit();
            for point in s2_corners.iter() {
                let v = Vec2::new_from_point(s1_corners[0] - *point);
                let py = s1_corners[0] + project(v, line);
                let px = s1_corners[0] + project(v, norm);
                proj_y1.push(px);
                proj_x1.push(py);
            }

            let line = Vec2::new(s2_corners[2].x - s2_corners[0].x, s2_corners[2].y - s2_corners[0].y);
            let norm = line.normal_unit();
            for point in s1_corners.iter() {
                let v = Vec2::new_from_point(s2_corners[0] - *point);
                let py = s2_corners[0] + project(v, line);
                let px = s2_corners[0] + project(v, norm);
                proj_y2.push(px);
                proj_x2.push(py);
            }

            let mut ls_x1 = [proj_x1[0], proj_x1[0]];
            let mut ls_y1 = [proj_y1[0], proj_y1[0]];
            let mut ls_x2 = [proj_x2[0], proj_x2[0]];
            let mut ls_y2 = [proj_y2[0], proj_y2[0]];
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

            let a1 = ls_x1[0];
            let b1 = ls_x1[1];

            let a2 = ls_y1[0];
            let b2 = ls_y1[1];

            let a3 = ls_x2[0];
            let b3 = ls_x2[1];

            let a4 = ls_y2[0];
            let b4 = ls_y2[1];

            let i1 = line_intersection(s1_corners[0], s1_corners[1], a1, b1);
            let i2 = line_intersection(s1_corners[0], s1_corners[2], a2, b2);
            let i3 = line_intersection(s2_corners[0], s2_corners[2], a3, b3);
            let i4 = line_intersection(s2_corners[0], s2_corners[1], a4, b4);
            if i1 && i2 && i3 && i4 {
                true
            } else {
                false
            }

        } else {

            match s2.shape {
                ShapeVariant::Circle{size: _s, radius: r} => {
                    circle_intersection(s2.center_point(), r, s1_corners, point1, point2, point3, point4)
                },
                _ => false
            }

        }

    } else {
        
        if let Some(s2_corners) = s2.get_corners() {

            match s1.shape {
                ShapeVariant::Circle{size: _s, radius: r} => {
                    circle_intersection(s1.center_point(), r, s2_corners, point1, point2, point3, point4)
                },
                _ => false
            }

        } else {
            false
        }

    }


}

fn project(vec: Vec2, line: Vec2) -> Point2 {

    let norm = Vec2::new_unit(line.x, line.y);
    let c = Vec2::dot_product(vec, norm) / Vec2::dot_product(norm, norm);

    Point2{x: norm.x * -c, y: norm.y * -c}

}

fn circle_intersection(c: Point2, r: f64, corners: Vec<Point2>, point1: &mut GenericShape,point2: &mut GenericShape,point3: &mut GenericShape,point4: &mut GenericShape,) -> bool {

    let p1 = corners[0] + project(Vec2::new_from_point(corners[0] - c), Vec2::new(corners[1].x - corners[0].x, corners[1].y - corners[0].y));
    let d1 = Vec2::new_from_point(p1 - c);
    let d1 = Vec2::dot_product(d1, d1);

    let p2 = corners[0] + project(Vec2::new_from_point(corners[0] - c), Vec2::new(corners[2].x - corners[0].x, corners[2].y - corners[0].y));     
    let d2 = Vec2::new_from_point(p2 - c);
    let d2 = Vec2::dot_product(d2, d2);

    let p3 = corners[2] + project(Vec2::new_from_point(corners[0] - c), Vec2::new(corners[3].x - corners[2].x, corners[3].y - corners[2].y));
    let d3 = Vec2::new_from_point(p3- c);
    let d3 = Vec2::dot_product(d3, d3);

    let p4 = corners[1] + project(Vec2::new_from_point(corners[0] - c), Vec2::new(corners[3].x - corners[1].x, corners[3].y - corners[1].y));
    let d4 = Vec2::new_from_point(p4 - c);
    let d4 = Vec2::dot_product(d4, d4);

    let r_2 = r.powi(2);

    // println!("{}, {}", d2 <= r_2, point_on_line(p2, corners[0], corners[2]));
    point1.set_position(p1);
    point2.set_position(p2);
    point3.set_position(p3);
    point4.set_position(p4);
    (d1 <= r_2 && point_on_line(p1, corners[1], corners[0])) || 
    (d2 <= r_2 && point_on_line(p2, corners[2], corners[0])) ||
    (d3 <= r_2 && point_on_line(p3, corners[3], corners[2])) ||
    (d4 <= r_2 && point_on_line(p4, corners[3], corners[1])) 

}

fn point_on_line(p: Point2, l1: Point2, l2: Point2) -> bool {

    if l1.x != l2.x {
        within(p.x, l1.x, l2.x)
    } else {
        within(p.y, l1.y, l2.y)
    }

}

fn within(p: f64, q: f64, r: f64) -> bool{
    (q <= p && p <= r) || (r <= p && p <= q)
}

fn line_intersection(p1: Point2, p2: Point2, q1: Point2, q2: Point2) -> bool {

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
    
    let dist_p = Vec2::new_from_point(p2-p1);
    let dist_p = Vec2::dot_product(dist_p, dist_p);

    let dist_q = Vec2::new_from_point(q2-q1);
    let dist_q = Vec2::dot_product(dist_q, dist_q);

    if max_dist.sqrt() <= dist_p.sqrt() + dist_q.sqrt(){
        true
    } else {
        false
    }

}
