/// A trait to model state-based Entities. 
/// Entity is a supertrait of State. (i.e. Implementing State requies Entity 
/// to have been implemented)
/// 
/// type StateEnum: 
///     The enum containing the different states
/// 
/// fn change_state()
/// 
///     Defines how to changes the state of the Entity to a new state.
pub trait State {
    type StateEnum;
    fn change_state(&mut self, new_state: Self::StateEnum);
}

