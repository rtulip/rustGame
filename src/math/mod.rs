pub mod random;
mod vector2d;
mod point2d;

pub use self::vector2d::{Vec2};
pub use self::point2d::{Point2};

pub fn point_on_line(p: Point2, l1: Point2, l2: Point2) -> bool {

    if l1.x != l2.x {
        within(p.x, l1.x, l2.x)
    } else {
        within(p.y, l1.y, l2.y)
    }

}

fn within(p: f64, q: f64, r: f64) -> bool{
    (q <= p && p <= r) || (r <= p && p <= q)
}