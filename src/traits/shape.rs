use graphics::{Rectangle, Context, Graphics};
use graphics::types::Color;

/// CircleType
/// 
/// A struct representing a Circle Shape
pub struct EllipseType {}
/// RectangleType
/// 
/// A struct representing a Rectangular Shape
pub struct RectangleType {}

/// Implementation of EllipseType reveals how to draw the type
impl EllipseType {
    /// draw()
    /// 
    /// args:
    ///     color: Color: The color to draw the Ellipse
    ///     radius: f64: The radius of the Ellipse in pixels
    ///     x: f64: The x position of the upper right hand corner of the 
    ///         ellipse bounding box
    ///     y: f64: The y position of the upper right hand corner of the 
    ///         ellipse bounding box 
    ///     width: f64: The width of the Ellipse in pixels
    ///     height: f64: The height of the Ellipse in pixels
    ///     c: &Context: The graphics context
    ///     g: &mut Graphics: A mutable Graphics
    /// 
    /// Draws an Ellipse in the specified Color at the specified location 
    pub fn draw<G: Graphics>(&self, color: Color, radius: f64, x: f64, y: f64, width: f64, height: f64, c: &Context, g: &mut G){
        Rectangle::new_round(color, radius).draw([x, y, width, height], &c.draw_state, c.transform, g);
    }
}

/// Implementation of EllipseType reveals how to draw the type
impl RectangleType{
    /// draw()
    /// 
    /// args:
    ///     color: Color: The color to draw the Rectangle
    ///     x: f64: The x position of the upper right hand corner of the 
    ///         ellipse bounding box
    ///     y: f64: The y position of the upper right hand corner of the 
    ///         ellipse bounding box 
    ///     width: f64: The width of the Rectangle in pixels
    ///     height: f64: The height of the Rectangle in pixels
    ///     c: &Context: The graphics context
    ///     g: &mut Graphics: A mutable Graphics
    /// 
    /// Draws an Rectangle in the specified Color at the specified location
    pub fn draw<G: Graphics>(&self, color: Color, x: f64, y: f64, width: f64, height: f64, c: &Context, g: &mut G){
        Rectangle::new(color).draw([x, y, width, height], &c.draw_state, c.transform, g);
    }
}

/// Shape Trait
/// 
/// Defines the shape, and how to draw an implementing object. Intention is to
/// decouple drawing from the object itself.
/// 
/// type ShapeVariant must be either EllipseType or RectangleType
/// get_shape() is to return an instance of the ShapeVariant
pub trait Shape {
    type ShapeVairant;
    fn get_shape(&self) -> Self::ShapeVairant;
}