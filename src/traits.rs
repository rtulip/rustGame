use piston::input::GenericEvent;
use graphics::{Context, Graphics};

pub trait Model<T> {
    fn new(init: T) -> Self;
}

pub trait Controller<T, M: Model<T>> {

    fn new(model: M) -> Self;
    fn event<E: GenericEvent>(&mut self, e: &E);
}

pub trait View<T, M: Model<T>, C: Controller<T,M>> {
    
    fn draw<G: Graphics>(&self, controller: &C, c: &Context, g: &mut G);

}