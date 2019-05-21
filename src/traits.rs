use piston::input::GenericEvent;
use graphics::{Context, Graphics};

pub trait Model {
    fn new() -> Self;
}

pub trait Controller<M: Model> {

    fn new(model: M) -> Self;
    fn event<E: GenericEvent>(&mut self, e: &E);
}

pub trait View<M: Model, C: Controller<M>> {

    fn draw<G: Graphics>(&self, controller: &C, c: &Context, g: &mut G)

}