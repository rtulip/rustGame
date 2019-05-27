pub struct Point2 {
    pub x: f64,
    pub y: f64,
}

impl std::ops::Add for Point2 {
    type Output = Self;

    fn add(self, other: Point2) -> Self {
        Point2 { x: self.x + other.x, y: self.y + other.y}
    }
}

impl std::ops::Sub for Point2 {
    type Output = Self;
    
    fn sub(self, other: Point2) -> Self {
        Point2 { x: self.x - other.x, y: self.y + other.y}
    }
}