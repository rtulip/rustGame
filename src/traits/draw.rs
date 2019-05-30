use crate::misc::point2d::Point2;
use crate::game::consts::PI;
pub use graphics::{Rectangle, Context, Graphics};
use graphics::Transformed;
use graphics::types::Color;

pub trait Draw {
    fn draw<G: Graphics>(&self, c: &Context, g: &mut G);
}

pub enum ShapeVariant {
    Square { size: f64 },
    Rect { width: f64, height: f64},
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

    pub fn set_color(&mut self, new_color: Color){
        self.color = new_color;
    }

    pub fn get_position(&self) -> Point2 {
        self.position
    }

    pub fn set_position(&mut self, new_pos: Point2) {
        self.position = new_pos;
    }

    pub fn set_rotation(&mut self, new_rot: f64) {
        self.rotation = Some(new_rot);
    }

    pub fn center_point(&self) -> Point2 {
        match self.shape {
            ShapeVariant::Square{size: val} => {
                let center_offset = Point2 {
                    x: val / 2.0,
                    y: val / 2.0,
                };
                if let Some(offset) = self.offset {
                    self.position + center_offset + offset
                } else {
                    self.position + center_offset
                }
                
            }
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

    pub fn top_right(&self) -> Point2 {
        match self.shape {
            ShapeVariant::Square{size: val} => {
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
            }
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

    pub fn bottom_right(&self) -> Point2 {
        match self.shape {
            ShapeVariant::Square{size: val} => {
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



