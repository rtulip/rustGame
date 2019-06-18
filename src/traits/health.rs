use crate::traits::draw::{GenericShape,ShapeVariant, Draw, Context, Graphics};
use crate::math::Point2;
use crate::game::consts::{
    HEALTH_BAR_HEIGHT,
    HEALTH_BAR_WIDTH,
    HEALTH_COLOR,
    DAMAGE_COLOR,
};

pub struct HealthBar {

    max_health: i32,
    health: i32,
    health_bar: GenericShape,
    damage_bar: GenericShape

}

impl HealthBar {

    pub fn new(max_health: i32, entity_point: Point2, entity_width: f64, entity_height: f64) -> Self {

        Self {
            max_health: max_health,
            health: max_health,
            health_bar: GenericShape::new(
                ShapeVariant::Rect{
                    width: HEALTH_BAR_WIDTH,
                    height: HEALTH_BAR_HEIGHT,
                }, 
                HEALTH_COLOR,
                entity_point + Point2{x: entity_width / 2.0 - HEALTH_BAR_WIDTH / 2.0, y: entity_height + HEALTH_BAR_HEIGHT * 1.2}
            ),
            damage_bar: GenericShape::new(
                ShapeVariant::Rect {
                    width: 0.0,
                    height: HEALTH_BAR_HEIGHT,
                }, 
                DAMAGE_COLOR,
                entity_point + Point2{x: entity_width / 2.0 - HEALTH_BAR_WIDTH / 2.0, y: entity_height + HEALTH_BAR_HEIGHT * 1.2}
            )
        }

    }

    pub fn damage(&mut self) {
        self.health -= 1;
        self.health_bar = GenericShape::new(
            ShapeVariant::Rect{
                width: HEALTH_BAR_WIDTH * self.health as f64 / self.max_health as f64,
                height: HEALTH_BAR_HEIGHT 
            }, 
            HEALTH_COLOR,
            self.health_bar.get_position(),
        );
        self.damage_bar = GenericShape::new(
            ShapeVariant::Rect{
                width: HEALTH_BAR_WIDTH * (self.max_health - self.health) as f64 / self.max_health as f64,
                height: HEALTH_BAR_HEIGHT 
            }, 
            DAMAGE_COLOR,
            self.health_bar.get_position() + Point2{x: HEALTH_BAR_WIDTH * self.health as f64 / self.max_health as f64, y: 0.0}
        );
    }

    pub fn update(&mut self, delta_pos: Point2, delta_rot: Option<f64>) {

        self.health_bar.update(delta_pos, delta_rot);
        self.damage_bar.update(delta_pos, delta_rot);

    }

}

impl Draw for HealthBar {

    fn draw<G: Graphics>(&self, c: &Context, g: &mut G) {

        self.health_bar.draw(c, g);
        self.damage_bar.draw(c, g);

    }

}