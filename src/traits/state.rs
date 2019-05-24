use crate::traits::entity::Entity;

pub trait State: Entity {
    type StateEnum;
    fn change_state(&self, new_state: Self::StateEnum);
}

