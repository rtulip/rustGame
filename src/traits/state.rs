/// A trait to model state-based Entities. 
/// Entity is a supertrait of State. (i.e. Implementing State requies Entity 
/// to have been implemented)
pub trait State {
    type StateEnum;
    fn change_state(&mut self, new_state: Self::StateEnum);
}

