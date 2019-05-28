use graphics::{Rectangle, Context, Graphics, Transformed};
use graphics::types::Color;

/// A struct representing a Circle Shape
pub struct EllipseType {}
/// A struct representing a Rectangular Shape
pub struct RectangleType {}

/// Implementation of EllipseType reveals how to draw the type
impl EllipseType {
    
    /// Draws an Ellipse in the specified Color at the specified location 
    pub fn draw<G: Graphics>(&self, color: Color, radius: f64, x: f64, y: f64, width: f64, height: f64, c: &Context, g: &mut G) {
        Rectangle::new_round(color, radius).draw([x, y, width, height], &c.draw_state, c.transform, g);
    }

}

/// Implementation of EllipseType reveals how to draw the type
impl RectangleType {
    
    /// Draws an Rectangle in the specified Color at the specified location
    pub fn draw<G: Graphics>(&self, color: Color, x: f64, y: f64, width: f64, height: f64, rotation: f64, c: &Context, g: &mut G) {
        let mut transform = c.transform;
        if rotation == 0.0 {
            transform = transform.trans(x,y);
        } else {
            transform = transform.trans(x,y).rot_rad(rotation).trans(-width/2.0,-height/2.0);
        }
        Rectangle::new(color).draw([0.0, 0.0, width, height], &c.draw_state, transform, g);
    }

}

/// Defines the shape, and how to draw an implementing object. Intention is to
/// decouple drawing from the object itself.
/// 
/// type ShapeVariant must be either EllipseType or RectangleType
/// get_shape() is to return an instance of the ShapeVariant
pub trait Shape {
    type ShapeVairant;
    fn get_shape(&self) -> Self::ShapeVairant;
}