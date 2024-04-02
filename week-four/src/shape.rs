pub trait Shape {
    fn area(&self) -> f64;
}

pub fn area<T>(shape: T) -> f64
where
    T: Shape,
{
    shape.area()
}

pub struct Circle {
    pub radius: f64,
}
impl Circle {
    pub fn new(radius: f64) -> Self {
        Circle { radius }
    }
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

pub struct Triangle {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Triangle {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Triangle { a, b, c }
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        let p = (self.a + self.b + self.c) / 2.0;
        (p * (p - self.a) * (p - self.b) * (p - self.c)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        let c = Circle::new(1.0);
        assert_eq!(c.area(), std::f64::consts::PI);
    }

    #[test]
    fn test_rectangle() {
        let r = Rectangle::new(2.0, 3.0);
        assert_eq!(r.area(), 6.0);
    }

    #[test]
    fn test_triangle() {
        let t = Triangle::new(3.0, 4.0, 5.0);
        assert_eq!(t.area(), 6.0);
    }
}
