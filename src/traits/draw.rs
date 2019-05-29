use crate::misc::point2d::Point2;
pub use graphics::{Rectangle, Context, Graphics};
use graphics::Transformed;
use graphics::types::Color;
use std::f64::consts::PI;

pub trait Draw {
    fn draw<G: Graphics>(&self, c: &Context, g: &mut G);
}

pub enum ShapeVariant {
    Square { size: f64 },
    Circle { size: f64, radius: f64 },
}

pub struct GenericShape {
    shape: ShapeVariant,
    color: Color,
    position: Point2,
    rotation: Option<f64>,
    offset: Option<Point2>,
}

impl GenericShape {
    
    pub fn new(shape: ShapeVariant, color: Color, position: Point2)-> Self  {
            
        Self {
            shape: shape,
            color: color,
            position: position,
            rotation: None,
            offset: None,
        }
    }

    pub fn update(&mut self, delta_pos: Point2, delta_rad: Option<f64>){

        self.position = self.position + delta_pos;
        match [delta_rad,self.rotation] {
            [Some(delta), Some(val)] => {
                if delta + val > 2.0 * PI || delta + val < -2.0 * PI {
                    self.rotation = Some(0.0);
                } else {
                    self.rotation = Some(delta+val);
                } 
                
            },
            [Some(delta), None] => {
                self.rotation = Some(delta);
            },
            _ => (),
        }

    }

    pub fn get_position(&self) -> Point2 {
        self.position
    }

    pub fn set_position(&mut self, new_pos: Point2) {
        self.position = new_pos;
    }

    pub fn set_color(&mut self, new_color: Color){
        self.color = new_color;
    }

    pub fn center_point(&self) -> Point2 {
        match self.shape {
            ShapeVariant::Square{size: val} => {
                let offset = Point2 {
                    x: val / 2.0,
                    y: val / 2.0,
                };
                self.position + offset
            }
            ShapeVariant::Circle{size: val, radius: _rad} => {
                let offset = Point2 {
                    x: val / 2.0,
                    y: val / 2.0,
                };
                self.position + offset
            }
        }
    }

    pub fn resize(&mut self, new_size: f64) {
        match self.shape {
            ShapeVariant::Square{size: _size} => {
                self.shape = ShapeVariant::Square{size: new_size};
            },
            ShapeVariant::Circle{size: _size, radius: rad} => {
                self.shape = ShapeVariant::Circle{size: new_size, radius: rad};
            }
        }
    }

    pub fn set_radius(&mut self, new_radius: f64){
        match self.shape {
            ShapeVariant::Circle{size: val, radius: _} => {
                self.shape = ShapeVariant::Circle{size: val, radius: new_radius};
            },
            _ => (),
        }
    }

    pub fn set_offset(&mut self, new_offset: Point2){
        self.offset = Some(new_offset);
    }

}

impl Draw for GenericShape {
    fn draw<G: Graphics>(&self, c: &Context, g: &mut G){
        let mut transform = c.transform;
        transform = transform.trans(self.position.x, self.position.y);
        match self.rotation {
            Some(rad) => {
                transform = transform.rot_rad(rad);
            },
            None => (),
        }
        match self.offset {
            Some(offset) => {
                transform = transform.trans(offset.x, offset.y);
            },
            None => (),
        }

        match self.shape {
            ShapeVariant::Square{ size } => {
                Rectangle::new(self.color).draw(
                    [0.0,0.0,size, size],
                    &c.draw_state,
                    transform,
                    g
                );
            },
            ShapeVariant::Circle{size, radius} => {
                Rectangle::new_round(self.color, radius).draw(
                    [0.0,0.0,size,size],
                    &c.draw_state, 
                    transform, 
                    g
                )
            }
        }

    }
}



