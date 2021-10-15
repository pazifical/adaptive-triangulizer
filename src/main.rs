use std::path::PathBuf;
use image::io::Reader as ImageReader;
use image::{Rgb, DynamicImage, GenericImageView, ImageBuffer, RgbImage};
use spade::delaunay::{DelaunayTriangulation, DelaunayWalkLocate, FloatDelaunayTriangulation};
use spade::{PointN, TwoDimensional};
use spade::kernels::FloatKernel;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: f32,
    y: f32,
    z: f32
}

impl Point {
    fn new(x: f32, y: f32, z: f32) -> Point {
        Point {
            x,
            y,
            z
        }
    }
}

impl TwoDimensional for Point {

}

impl PointN for Point {
    type Scalar = f32;

    fn dimensions() -> usize {
        3
    }

    fn from_value(value: Self::Scalar) -> Self {
        Point{ x: value, y: value, z: value }
    }

    fn nth(&self, index: usize) -> &Self::Scalar {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => &self.x // TODO
        }
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => &mut self.x // TODO
        }
    }
}


fn main() {
    println!("Hello, world!");

    let path = PathBuf::from("/home/minerva/Programming/Trimage/data/2017_China_Chongqing_Boats.jpg");

    let img = ImageReader::open(&path).unwrap().decode().unwrap();
    println!("{:?}", img.get_pixel(0, 0));

    let (width, height) = img.dimensions();

    let mut delaunay = FloatDelaunayTriangulation::with_walk_locate();
    delaunay.insert(Point{ x: 0.0, y: 0.0, z: 1.0 });
    delaunay.insert(Point{ x: (width-1) as f32, y: 0.0, z: 2.0 });
    delaunay.insert(Point{ x: (width-1) as f32, y: (height-1) as f32, z: 3.0 });
    delaunay.insert(Point{ x: 0.0, y: (height-1) as f32, z: 4.0 });

    for face in delaunay.triangles() {
        let triangle = face.as_triangle();
        println!("Found triangle: {:?} -> {:?} -> {:?}", *triangle[0], *triangle[1], *triangle[2]);
    }

    let img = rasterize_mesh(&delaunay, width, height);
}


type Mesh = DelaunayTriangulation<Point, FloatKernel, DelaunayWalkLocate>;

fn rasterize_mesh(mesh: &Mesh, width: u32, height: u32) -> RgbImage {
    let mut img = ImageBuffer::new(width, height);
    for face in mesh.triangles() {
        let triangle = face.as_triangle();
        img.put_pixel(0, 0, Rgb([10, 10, 20]));
    }
    RgbImage::from(img)
}

