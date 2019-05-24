use crate::traits::shape::Shape;

pub trait Entity: Shape {

    fn tick(&mut self);

}