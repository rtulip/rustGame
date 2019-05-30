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
        Self {
            base_shape: GenericShape::new(
                ShapeVariant::Circle{
                    size: TOWER_SIZE, 
                    radius: TOWER_RADIUS
                },
                TOWER_COLOR,
                position),
            cannon_shape: GenericShape::new(
                ShapeVariant::Rect{
                    width: TOWER_CANNON_WIDTH,
                    height: TOWER_CANNON_HEIGHT,
                }, 
                TOWER_CANNON_COLOR, 
                position)
        }
    }
}

impl Draw for Tower {
    fn draw<G: Graphics>(&self, c: &Context, g: &mut G){
        self.base_shape.draw(c, g);
        self.cannon_shape.draw(c, g);
    }
}