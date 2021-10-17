// Community library imports
use image::Rgba;
use spade::{PointN, TwoDimensional};


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub c: Rgba<u8>
}

impl Point {
    pub fn new(x: f32, y: f32, c: Rgba<u8>) -> Point {
        Point {
            x,
            y,
            c
        }
    }
}

impl TwoDimensional for Point {}

impl PointN for Point {
    type Scalar = f32;

    fn dimensions() -> usize {
        2
    }

    fn from_value(value: Self::Scalar) -> Self {
        Point{ x: value, y: value, c: Rgba([255, 255, 255, 255]) }
    }

    fn nth(&self, index: usize) -> &Self::Scalar {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => &self.x // TODO
        }
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => &mut self.x // TODO
        }
    }
}

