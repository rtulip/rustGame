use crate::math::{Point2, collision};
use crate::game::consts::PI;
pub use graphics::{Rectangle, Context, Graphics};
use graphics::Transformed;
use graphics::types::Color;

/// Trait for drawing objects to the screen.
pub trait Draw {
    fn draw<G: Graphics>(&self, c: &Context, g: &mut G);
}

/// Different Types of shapes. More complex shapes can be created by combining
/// ShapeVariants. 
#[derive(Clone, Copy)]
pub enum ShapeVariant {
    Rect { width: f64, height: f64},
    Circle { size: f64, radius: f64 },
}

/// A generic shape which can be used to draw every shape in the game.
#[derive(Clone, Copy)]
pub struct GenericShape {
    pub shape: ShapeVariant,
    color: Color,
    position: Point2,
    rotation: Option<f64>,
    offset: Option<Point2>,
}

impl GenericShape {
    
    /// Creates a new GenericShape
    pub fn new(shape: ShapeVariant, color: Color, position: Point2)-> Self  {
            
        Self {
            shape: shape,
            color: color,
            position: position,
            rotation: None,
            offset: None,
        }
    }

    /// Update function which moves the shape by delta_pos and can optionally
    /// rotate the shape. 
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

    /// Function to set the color.
    pub fn set_color(&mut self, new_color: Color){
        self.color = new_color;
    }

    /// Function to return the private Position field.
    pub fn get_position(&self) -> Point2 { 
        self.position
    }

    /// Function to set the position of the GenericShape.
    pub fn set_position(&mut self, new_pos: Point2) {
        self.position = new_pos;
    }

    /// Function to get the private Rotation field.
    pub fn get_rotation(&self) -> Option<f64> {
        self.rotation
    }

    /// Function to set the rotation of the shape.
    pub fn set_rotation(&mut self, new_rot: f64) {
        self.rotation = Some(new_rot);
    }

    /// Function to return the center point of the GenericShape. The 
    /// calculation depends on ShapeVariant because of the differnet enum 
    /// struct fields.
    pub fn center_point(&self) -> Point2 {
        match self.shape {
            ShapeVariant::Circle{size: val, radius: _rad} => {
                let center_offset = Point2 {
                    x: val / 2.0,
                    y: val / 2.0,
                };
                if let Some(offset) = self.offset {
                    self.position + center_offset + offset
                } else {
                    self.position + center_offset
                } 
            },
            ShapeVariant::Rect{width: w, height: h} => {
                let center_offset = Point2 {
                    x: w / 2.0,
                    y: h / 2.0,
                };
                if let Some(offset) = self.offset {
                    self.position + center_offset + offset
                } else {
                    self.position + center_offset
                }
            }
        }
    }

    /// Function to find the top right corner of the shape as a Point2. Depends
    /// on the ShapeVariant and the Rotation of the GenericShape.
    pub fn top_right(&self) -> Point2 {
        match self.shape {
            ShapeVariant::Circle{size: val, radius: _rad} => {
                match self.rotation {
                    Some(rot) => {
                        let offset = Point2 {
                            x: val * (-1.0 * rot).cos(),
                            y: -val * (-1.0 * rot).sin(),
                        };
                        self.position + offset
                    }
                    None => {
                        let offset = Point2 {
                            x: val,
                            y: 0.0,
                        };
                        self.position + offset
                    }
                }
            },
            ShapeVariant::Rect{width: w, height: _h} => {
                match self.rotation {
                    Some(rot) => {
                        let offset = Point2 {
                            x: w * (-1.0 * rot).cos(),
                            y: -w * (-1.0 * rot).sin(),
                        };
                        self.position + offset
                    }
                    None => {
                        let offset = Point2 {
                            x: w,
                            y: 0.0,
                        };
                        self.position + offset
                    }
                }
            }
        }
    }

    /// Function to find the bottom right corner of the shape as a Point2.
    /// Depends on the ShapeVariant and the Rotation of the GenericShape.
    pub fn bottom_right(&self) -> Point2 {
        match self.shape {
            ShapeVariant::Circle{size: val, radius: _rad} => {
                match self.rotation {
                    Some(rot) => {
                        let offset = Point2 {
                            x: val * (PI / 2.0 + rot).cos(),
                            y: val * (PI / 2.0 + rot).sin(),
                        };
                        self.top_right() + offset
                    }
                    None => {
                        let offset = Point2 {
                            x: val,
                            y: val,
                        };
                        self.position + offset
                    }
                }
            },
            ShapeVariant::Rect{width: w, height: h} => {
                match self.rotation {
                    Some(rot) => {
                        let offset = Point2 {
                            x: h * (PI / 2.0 + rot).cos(),
                            y: h * (PI / 2.0 + rot).sin(),
                        };
                        self.top_right() + offset
                    }
                    None => {
                        let offset = Point2 {
                            x: w,
                            y: h,
                        };
                        self.position + offset
                    }
                }
            }
        }
    }

    pub fn get_corners(&self) -> Option<Vec<Point2>> {

        match self.shape {
            ShapeVariant::Rect{width: w, height: h} => {

                let mut rot: f64 = 0.0;
                if let Some(rotation) = self.rotation {
                    rot = rotation;
                }
                let mut offset = Point2{x: 0.0, y: 0.0};
                if let Some(offs) = self.offset {
                    offset = offs
                }

                let o = Point2{
                    x: self.position.x,
                    y: self.position.y,
                };

                let p1 = Point2{
                    x: self.position.x,
                    y: self.position.y,
                };
                let p1 = p1 + offset;
                let p1 = Point2{
                    x: p1.x * rot.cos() - p1.y * rot.sin() + o.x - o.x * rot.cos() + o.y * rot.sin(),
                    y: p1.x * rot.sin() + p1.y * rot.cos() + o.y - o.x * rot.sin() - o.y * rot.cos()
                };
                
                let p2 = Point2{
                    x: self.position.x + w,
                    y: self.position.y,
                };
                let p2 = p2 + offset;
                let p2 = Point2{
                    x: p2.x * rot.cos() - p2.y * rot.sin() + o.x - o.x * rot.cos() + o.y * rot.sin(),
                    y: p2.x * rot.sin() + p2.y * rot.cos() + o.y - o.x * rot.sin() - o.y * rot.cos()
                };

                let p3 = Point2{
                    x: self.position.x,
                    y: self.position.y + h,
                };
                let p3 = p3 + offset;
                let p3 = Point2{
                    x: p3.x * rot.cos() - p3.y * rot.sin() + o.x - o.x * rot.cos() + o.y * rot.sin(),
                    y: p3.x * rot.sin() + p3.y * rot.cos() + o.y - o.x * rot.sin() - o.y * rot.cos()
                };

                let p4 = Point2{
                    x: self.position.x + w,
                    y: self.position.y + h,
                };
                let p4 = p4 + offset;
                let p4 = Point2{
                    x: p4.x * rot.cos() - p4.y * rot.sin() + o.x - o.x * rot.cos() + o.y * rot.sin(),
                    y: p4.x * rot.sin() + p4.y * rot.cos() + o.y - o.x * rot.sin() - o.y * rot.cos()
                };

                Some(vec![
                    p1, 
                    p2, 
                    p3,
                    p4,
                ])

            },
            ShapeVariant::Circle{size: _s, radius: _r} => {

               None

            }
        }

    }

    /// Function to set the private Offset field.
    pub fn set_offset(&mut self, new_offset: Point2){
        self.offset = Some(new_offset);
    }

    pub fn get_offset(&self) -> Option<Point2> {
        self.offset
    }

}

impl Draw for GenericShape {
    /// Function to draw the shape.
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
            ShapeVariant::Circle{size, radius} => {
                Rectangle::new_round(self.color, radius).draw(
                    [0.0,0.0,size,size],
                    &c.draw_state, 
                    transform, 
                    g
                )
            },
            ShapeVariant::Rect{width, height} => {
                Rectangle::new(self.color).draw(
                    [0.0,0.0,width,height], 
                    &c.draw_state, 
                    transform, 
                    g
                )
            }
        }

    }
}

/// Function which checks if two generic shapes are colliding.
pub fn check_collision(s1: GenericShape, s2: GenericShape) -> bool {
    
    // See if shape1 is a rect
    if let Some(s1_corners) = s1.get_corners() {

        // See if shape2 is a rect
        if let Some(s2_corners) = s2.get_corners() {
            
            // Check rect-rect collision.
            collision::rect_rect_intersect(s1_corners, s2_corners)
            
        } else {

            // Check for circle-rect collision.
            match s2.shape {
                ShapeVariant::Circle{size: _s, radius: r} => {
                    collision::circle_rect_intersect(s2.center_point(), r, s1_corners)
                },
                _ => false
            }

        }

    } else {
        
        if let Some(s2_corners) = s2.get_corners() {

            // Check for circle-rect collision.
            match s1.shape {
                ShapeVariant::Circle{size: _s, radius: r} => {
                    collision::circle_rect_intersect(s1.center_point(), r, s2_corners)
                },
                _ => false
            }

        } else {
            
            // Check for circle-circle collision.
            match [s1.shape, s2.shape] {
                [ShapeVariant::Circle{size: _s1, radius: r1}, ShapeVariant::Circle{size: _s2, radius: r2}] => {
                    collision::circle_circle_intersect(s1.center_point(), r1, s2.center_point(), r2)
                },
                _ => false
            }

        }

    }

}