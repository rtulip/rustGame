use crate::math::Point2;
use crate::math::Vec2;
use crate::traits::draw::{Draw, Context, Graphics, GenericShape, ShapeVariant};
use crate::traits::state::State;
use crate::traits::entity::Entity;
use crate::entity::towers::bullet::Bullet;
use crate::game::consts::{
    TOWER_COLOR,
    TOWER_SIZE,
    TOWER_RADIUS,
    TOWER_CANNON_WIDTH,
    TOWER_CANNON_HEIGHT,
    TOWER_CANNON_COLOR,
    TOWER_RANGE,
    BULLET_HEIGHT,
};

/// Enumeration describing the states of the Tower. While Ready, if an Enemy is
/// in range, the Tower will start shooting and change it's state to Attacking.
/// While Attacking, the Tower won't be able to fire any more bullets.
pub enum TowerState {
    Ready,
    Attacking,
    Waiting(f64),
}

/// A struct describing the towers in the game. 
pub struct Tower {
    pub base_shape: GenericShape,
    pub cannon_shape: GenericShape,
    pub range: f64,
    pub state: TowerState,
    pub bullet: Bullet
}

impl Tower {
    
    /// Creates a new Tower. 
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

    /// Sets the rotation of the Tower.
    pub fn set_rotation(&mut self, new_roation: f64) {
        self.cannon_shape.set_rotation(new_roation);
    }
}


impl Draw for Tower {
    
    /// Function to draw the Tower.
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

    /// Function to change state. When setting the state to attacking, the 
    /// bullet position is reset, and the rotation is set to match the Tower.
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
                    self.bullet.shape.set_rotation(rot);
                }
                
            },
            _ => ()
        }
        self.state = new_state;

    }
}

impl Entity for Tower {
    
    /// Function to move the Bullet each game Tick if Tower is Attacking.
    fn tick(&mut self, dt: f64) {
        match self.state {
            TowerState::Attacking => {
                self.bullet.tick(dt);
            },
            TowerState::Waiting(t) => {
                if t-dt <= 0.0 {
                    self.change_state(TowerState::Ready);
                } else {
                    self.change_state(TowerState::Waiting(t-dt));
                }
               
            }
            _ => (),
        }
    }

}

