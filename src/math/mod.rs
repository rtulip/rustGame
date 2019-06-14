pub mod random;
mod vector2d;
mod point2d;

pub use self::vector2d::{Vec2};
pub use self::point2d::{Point2};

/// Function to see if a point lies on a line. The line is defined by two 
/// points, a start and an end, l1 and l2. 
fn point_on_line(p: Point2, l1: Point2, l2: Point2) -> bool {

    // If the x values aren't the same, see if the point's x value lies within
    // the range created by the line's x values.
    if l1.x != l2.x {
        within(p.x, l1.x, l2.x)
    // if the x values are the same, see if the point's y value lies within the
    // range created by the line's y values.
    } else {
        within(p.y, l1.y, l2.y)
    }

}

/// Function to see if a value p lies within a range [p,q].
fn within(p: f64, q: f64, r: f64) -> bool{
    (q <= p && p <= r) || (r <= p && p <= q)
}

/// Function to see if two lines intersect. Lines are defined by two points, a
/// start and an end point. Assumes that the lines are colinear.
pub fn line_intersection(p1: Point2, p2: Point2, q1: Point2, q2: Point2) -> bool {

    let p = vec![p1,p2];
    let q = vec![q1,q2];

    // Finds the largest distance between points between the two lines.
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
    
    // Finds the length of the first line.
    let dist_p = Vec2::new_from_point(p2-p1);
    let dist_p = Vec2::dot_product(dist_p, dist_p);

    // Finds the length of the second line.
    let dist_q = Vec2::new_from_point(q2-q1);
    let dist_q = Vec2::dot_product(dist_q, dist_q);

    // If the maximum distance is less than the sum of the distance of the two
    // lines, then the lines intersect
    if max_dist.sqrt() <= dist_p.sqrt() + dist_q.sqrt(){
        true
    } else {
        false
    }

}

/// Function to see if a circle is intercecting with a rectangle. The circle is
/// defined by a center point and a radius. The rectangle is defined by a list
/// of four corners.
pub fn circle_rect_intersect(c: Point2, r: f64, corners: Vec<Point2>) -> bool {

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

    (d1 <= r_2 && point_on_line(p1, corners[1], corners[0])) || 
    (d2 <= r_2 && point_on_line(p2, corners[2], corners[0])) ||
    (d3 <= r_2 && point_on_line(p3, corners[3], corners[2])) ||
    (d4 <= r_2 && point_on_line(p4, corners[3], corners[1])) 

}

/// function to project a vector onto a line. The line is defined by another
/// vector. 
pub fn project(vec: Vec2, line: Vec2) -> Point2 {

    let norm = Vec2::new_unit(line.x, line.y);
    let c = Vec2::dot_product(vec, norm);

    Point2{x: norm.x * -c, y: norm.y * -c}

}

/// Finds the two points which are the farthest from each other from a list of
/// points.
pub fn find_extrema(points: Vec<Point2>) -> Vec<Point2>{

    let mut extremes = vec![points[0], points[0]];
    for i in 0..points.len() {
        for j in i..points.len() {
            let d = points[i] - points[j];
            let d = Vec2::new_from_point(d);

            let line_seg = extremes[0] - extremes[1];
            let line_seg = Vec2::new_from_point(line_seg);

            if Vec2::dot_product(d,d) > Vec2::dot_product(line_seg, line_seg) {
                extremes[0] = extremes[i];
                extremes[1] = extremes[j];
            }
        }
    }
    extremes

}