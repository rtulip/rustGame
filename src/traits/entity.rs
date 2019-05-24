use crate::traits::shape::Shape;

/// Entity Trait
/// 
/// Shape is a supertrait of Entity
/// tick() is a method to update the Entity once per game loop
pub trait Entity: Shape {

    fn tick(&mut self);

}