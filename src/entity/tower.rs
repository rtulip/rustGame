use crate::misc::point2d::Point2;
use crate::traits::draw::{Draw, Context, Graphics, GenericShape, ShapeVariant};
use crate::game::consts::{
    TOWER_COLOR,
    TOWER_SIZE,
    TOWER_RADIUS,
    TOWER_CANNON_WIDTH,
    TOWER_CANNON_HEIGHT,
    TOWER_CANNON_COLOR,
};

pub struct Tower {
    pub base_shape: GenericShape,
    pub cannon_shape: GenericShape,
}

impl Tower {
    pub fn new(position: Point2) -> Self {
        let base_shape = GenericShape::new(
            ShapeVariant::Circle{
                size: TOWER_SIZE, 
                radius: TOWER_RADIUS
            },
            TOWER_COLOR,
            position
        );

        let mut cannon_shape = GenericShape::new(
            ShapeVariant::Rect{
                width: TOWER_CANNON_WIDTH,
                height: TOWER_CANNON_HEIGHT,
            }, 
            TOWER_CANNON_COLOR, 
            base_shape.center_point()
        );
        cannon_shape.set_offset(Point2{
            x: 0.0,
            y: - TOWER_CANNON_HEIGHT / 2.0
        });

        Self {
            cannon_shape:cannon_shape,
            base_shape: base_shape, 
        }
    }

    pub fn set_rotation(&mut self, new_roation: f64) {
        self.cannon_shape.set_rotation(new_roation);
    }
}

impl Draw for Tower {
    fn draw<G: Graphics>(&self, c: &Context, g: &mut G){
        self.base_shape.draw(c, g);
        self.cannon_shape.draw(c, g);
    }
}