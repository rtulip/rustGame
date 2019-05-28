use crate::traits::shape;
/// A structure representing the player attack. Attacks are of a RectangleType
pub struct Attack {}

impl shape::Shape for Attack {
    type ShapeVairant = shape::RectangleType;
    fn get_shape(&self) -> Self::ShapeVairant {
        shape::RectangleType {}
    }
}