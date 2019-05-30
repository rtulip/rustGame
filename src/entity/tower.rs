use crate::misc::point2d::Point2;
use crate::misc::vector2d::Vec2;
use crate::traits::draw::{Draw, Context, Graphics, GenericShape, ShapeVariant};
use crate::traits::state::State;
use crate::traits::entity::Entity;
use crate::game::consts::{
    TOWER_COLOR,
    TOWER_SIZE,
    TOWER_RADIUS,
    TOWER_CANNON_WIDTH,
    TOWER_CANNON_HEIGHT,
    TOWER_CANNON_COLOR,
    TOWER_RANGE,
    BULLET_WIDTH,
    BULLET_HEIGHT,
    BULLET_COLOR,
    BULLET_SPEED,
};

pub enum TowerState{
    Ready,
    Attacking,
}

pub struct Tower {
    pub base_shape: GenericShape,
    pub cannon_shape: GenericShape,
    pub range: f64,
    pub state: TowerState,
    pub bullet: Bullet
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
            range: TOWER_RANGE,
            state: TowerState::Ready,
            bullet: Bullet::new(
                Point2{x: 0.0, y: 0.0}, 
                Vec2::new(0.0, 0.0)
            ),
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
        match self.state {
            TowerState::Attacking => {
                self.bullet.shape.draw(c, g);
            },
            _ => (),
        }
    }
}

impl State for Tower {

    type StateEnum = TowerState;
    fn change_state(&mut self, new_state: Self::StateEnum) {
        match new_state {
            TowerState::Attacking => {
                if let Some(rot) = self.cannon_shape.get_rotation() {
                    self.bullet = Bullet::new(
                        self.base_shape.center_point(),
                        Vec2::new(rot.cos(), rot.sin())
                    );
                    self.bullet.shape.set_offset(Point2{
                        x: 0.0,
                        y: -BULLET_HEIGHT / 2.0
                    });
                }
                
            },
            _ => ()
        }
        self.state = new_state;

    }
}

impl Entity for Tower {
    fn tick(&mut self) {
        match self.state {
            TowerState::Attacking => {
                self.bullet.tick();
            }
            _ => (),
        }
    }
}

pub struct Bullet {
    pub shape: GenericShape,
    direction: Vec2,
}

impl Bullet {

    fn new(position: Point2, direction: Vec2) -> Self {
        Self {
            shape: GenericShape::new(
                ShapeVariant::Rect{
                    width: BULLET_WIDTH,
                    height: BULLET_HEIGHT,
                }, 
                BULLET_COLOR, 
                position,
            ),
            direction: direction,
        }
    }

}

impl Entity for Bullet {
    fn tick(&mut self) {
        let delta = Point2{
            x: self.direction.x * BULLET_SPEED,
            y: self.direction.y * BULLET_SPEED,
        };
        self.shape.update(delta,None);
        // println!("BULLET POSITION: {:?}", self.shape.get_position());

    }
}