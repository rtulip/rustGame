use crate::traits::shape::Shape;

pub trait Entity<S: Shape> {

    fn tick(&self);

}