/// Shape is a supertrait of Entity
/// tick() is a method to update the Entity once per game loop
pub trait Entity {
    fn tick(&mut self, dt: f64);
}
