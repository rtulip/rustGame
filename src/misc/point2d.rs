pub struct Point2 {
    pub x: f64,
    pub y: f64,
}

impl std::ops::Add for Point2 {
    fn add(self, other: Point2) {
        Point2 { x: self.x + other.x, y: self.y + other.y}
    }
}

impl std::ops::Sub for Point2 {
    fn sub(self, other: Point2) {
        Point2 { x: self.x - other.x, y: self.y + other.y}
    }
}