// Community library imports
use image::Rgba;
use spade::{PointN, TwoDimensional};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub c: Rgba<u8>,
}

impl Point {
    pub fn new(x: f32, y: f32, c: Rgba<u8>) -> Point {
        Point { x, y, c }
    }
}

impl TwoDimensional for Point {}

impl PointN for Point {
    type Scalar = f32;

    fn dimensions() -> usize {
        2
    }

    fn from_value(value: Self::Scalar) -> Self {
        Point {
            x: value,
            y: value,
            c: Rgba([255, 255, 255, 255]),
        }
    }

    fn nth(&self, index: usize) -> &Self::Scalar {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => &self.x, // TODO
        }
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => &mut self.x, // TODO
        }
    }
}


#[cfg(test)]
mod point_tests {
    use super::*;

    #[test]
    fn test_construction() {
        let x = 10.0;
        let y = 20.0;
        let c = Rgba([1, 2, 3, 4]);

        let p = Point::new(x, y, c);
        
        assert_eq!(p.x, x);
        assert_eq!(p.y, y);
        assert_eq!(p.c, c);
    }

    #[test]
    fn test_from_value() {
        let value = 23.0;
        let p = Point::from_value(value);

        assert_eq!(p.x, value);
        assert_eq!(p.y, value);
        assert_eq!(p.c, Rgba([255; 4]));
    }

    #[test]
    fn test_nth() {
        let x = 10.0;
        let y = 20.0;
        let c = Rgba([1, 2, 3, 4]);

        let mut p = Point::new(x, y, c);
        
        assert_eq!(*p.nth(0), p.x);
        assert_eq!(*p.nth(1), p.y);
        assert_eq!(*p.nth(1337), p.x);
        assert_eq!(*p.nth_mut(0), x);
        assert_eq!(*p.nth_mut(1), y);
        assert_eq!(*p.nth_mut(1337), x);
    }
}