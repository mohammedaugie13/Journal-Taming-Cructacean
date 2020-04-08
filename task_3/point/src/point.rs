#[derive(Debug)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    #[allow(dead_code)]
    pub fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    #[allow(dead_code)]
    pub fn zero(&self) -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    #[allow(dead_code)]
    pub fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
    #[allow(dead_code)]
    pub fn add(&self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    #[allow(dead_code)]
    pub fn sub(&self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
    #[allow(dead_code)]
    pub fn mul(&self, other: Point) -> Point {
        Point {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
    #[allow(dead_code)]
    pub fn dot(p1: Point, p2: Point) -> f64 {
        p1.x * p2.x + p1.y * p2.y
    }
    #[allow(dead_code)]
    pub fn clone(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}
