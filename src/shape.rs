use graphics::{Rectangle, Context, Graphics};
use graphics::types::Color;

pub enum ShapeType {
    Circle,
    Rectangle,
}

impl ShapeType {
    fn draw<G: Graphics>(&self, color: Color, radius: f64, x: f64, y: f64, width: f64, height: f64, c: &Context, g: &mut G){
        
        match self {
            ShapeType::Circle => {
                Rectangle::new_round(color, radius).draw([x, y, width, height], &c.draw_state, c.transform, g);
            },
            ShapeType::Rectangle => {
                Rectangle::new(color).draw([x, y, width, height], &c.draw_state, c.transform, g);
            }
        }
    }
}

pub trait Shape {
    type ShapeVairant;
}