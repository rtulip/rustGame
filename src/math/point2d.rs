/// A structure to represent a point in 2d space in a usable manner by the
/// GameView.
#[derive(Clone, Copy, Debug)]
pub struct Point2 {
    pub x: f64,
    pub y: f64,
}

impl std::ops::Add for Point2 {
    type Output = Self;

    fn add(self, other: Point2) -> Self {
        Point2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point2 {
    type Output = Self;

    fn sub(self, other: Point2) -> Self {
        Point2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<f64> for Point2 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Mul<Point2> for f64 {
    type Output = Point2;

    fn mul(self, rhs: Point2) -> Self::Output {
        Point2 {
            x: rhs.x * self,
            y: rhs.y * self,
        }
    }
}

#[cfg(test)]
mod point2d_tests {

    use super::Point2;

    #[test]
    fn test_add_point() {
        let p1 = Point2 { x: 3.0, y: 5.0 };
        let p2 = Point2 { x: -2.5, y: 0.7 };

        assert_eq!(0.5, (p1 + p2).x);
        assert_eq!(5.7, (p1 + p2).y);
    }

    #[test]
    fn test_sub_point() {
        let p1 = Point2 { x: 3.0, y: 5.0 };
        let p2 = Point2 { x: -2.5, y: 0.7 };

        assert_eq!(5.5, (p1 - p2).x);
        assert_eq!(4.3, (p1 - p2).y);
    }

    #[test]
    fn test_mul_point() {
        let c = 1.5;
        let p = Point2 { x: 3.0, y: 5.0 };

        assert_eq!(4.5, (c * p).x);
        assert_eq!(7.5, (c * p).y);
        assert_eq!(4.5, (p * c).x);
        assert_eq!(7.5, (p * c).y);
    }
}
