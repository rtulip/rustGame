use crate::traits::entity::Entity;

pub trait State: Entity {
    type StateEnum;
    fn change_state(&mut self, new_state: Self::StateEnum);
}

