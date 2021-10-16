use std::ops::Bound;
use std::path::PathBuf;
use image::io::Reader as ImageReader;
use image::{Rgb, Rgba, DynamicImage, GenericImageView, ImageBuffer, RgbImage, Pixel, RgbaImage};
use spade::delaunay::{DelaunayTriangulation, DelaunayWalkLocate, FloatDelaunayTriangulation, VertexHandle};
use spade::{PointN, TwoDimensional};
use spade::kernels::{FloatKernel, TrivialKernel};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    c: Rgba<u8>
}

impl Point {
    fn new(x: i32, y: i32, c: Rgba<u8>) -> Point {
        Point {
            x,
            y,
            c
        }
    }
}

impl TwoDimensional for Point {

}

impl PointN for Point {
    type Scalar = i32;

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


fn main() {
    println!("Hello, world!");

    let path = PathBuf::from("/home/minerva/Programming/Trimage/data/2017_China_Chongqing_Boats.jpg");
    let path_out = PathBuf::from("/home/minerva/Programming/Trimage/temp/2017_China_Chongqing_Boats.jpg");

    let img = ImageReader::open(&path).unwrap().decode().unwrap();
    println!("{:?}", img.get_pixel(0, 0));
    println!("{:?}", img.get_pixel(0, 0).to_luma());

    let (width, height) = img.dimensions();
    println!("img dimensions: {:?}", img.dimensions());

    let mut delaunay = DelaunayTriangulation::with_walk_locate();

    for y in (0..height).step_by(10) {
        for x in (0..width).step_by(10) {
            delaunay.insert(
                Point {
                    x: x as i32,
                    y: y as i32,
                    c: img.get_pixel(x, y)
                }
            );
        }
    }

    for face in delaunay.triangles() {
        let triangle = face.as_triangle();
        println!("Found triangle: {:?} -> {:?} -> {:?}", *triangle[0], *triangle[1], *triangle[2]);
    }

    let img = rasterize_mesh(&delaunay, width as i32, height as i32);

    img.save(path_out).unwrap();
}


type Mesh = DelaunayTriangulation<Point, TrivialKernel, DelaunayWalkLocate>; // TODO: Check if it would be better to use the AdaptiveIntKernel
// type Element = [VertexHandle<Point>; 3];

fn rasterize_mesh(mesh: &Mesh, width: i32, height: i32) -> RgbaImage {
    let mut img = ImageBuffer::new(width as u32, height as u32);
    for face in mesh.triangles() {
        let triangle = face.as_triangle();
        let bbox = BoundingBox::from_triangle(&triangle);

        for row in bbox.ymin..bbox.ymax {
            for col in bbox.xmin..bbox.xmax {
                let point = Point::new(col, row, Rgba([255, 255, 255, 255]));
                if is_point_in_triangle(&point, &triangle) {
                    let color = interpolate_rgba_in_triangle(&point, &triangle);
                    img.put_pixel(col as u32, row as u32, color);
                }
            }
        }
    }
    RgbaImage::from(img)
}

struct BoundingBox {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32
}

impl BoundingBox {
    pub fn from_triangle(triangle: &[VertexHandle<Point>; 3]) -> BoundingBox {
        let mut xmin = 100000;
        let mut xmax = 0;
        let mut ymin = 100000;
        let mut ymax = 0;

        for point in triangle.iter() {
            if point.x < xmin { xmin = point.x; }
            else if point.x > xmax { xmax = point.x; }
            if point.y < ymin { ymin = point.y; }
            else if point.y > ymax { ymax = point.y; }
        }
        BoundingBox { xmin, xmax, ymin, ymax }
    }
}

fn is_ccw(p1: &Point, p2: &Point, p3: &Point) -> bool {
    (p3.y - p1.y) * (p2.x - p1.x) >= (p2.y - p1.y) * (p3.x - p1.x)
}

fn is_point_in_triangle(point: &Point, triangle: &[VertexHandle<Point>; 3]) -> bool {
    let mut ccw_count = 0;

    for i in 0..3 {
        let i1 = i;
        let i2 = (i+1) % 3;
        match is_ccw(&point, &triangle[i1], &triangle[i2]) {
            true => { ccw_count += 1; }
            false => {}
        }
    }

    if ccw_count == 0 || ccw_count == 3 {
        true
    } else {
        false
    }
}

fn interpolate_rgba_in_triangle(point: &Point, triangle: &[VertexHandle<Point>; 3]) -> Rgba<u8> {
    Rgba([0,0,0,0])
}

fn calc_barycentric_interpolatin_weights(p: &Point, triangle: &[VertexHandle<Point>; 3]) -> [f32; 3] {
    let v1 = &triangle[0];
    let v2 = &triangle[1];
    let v3 = &triangle[2];
    let w1 = (((v2.y - v3.y) * (p.x - v3.x) + (v3.x - v2.x) * (p.y - v3.y)) as f32) /
        (((v2.y - v3.y) * (v1.x - v3.x) + (v3.x - v2.x) * (v1.y - v3.y)) as f32);
    let w2 = (((v3.y - v1.y) * (p.x - v3.x) + (v1.x - v3.x) * (p.y - v3.y)) as f32) /
        (((v2.y - v3.y) * (v1.x - v3.x) + (v3.x - v2.x) * (v1.y - v3.y)) as f32);
    let w3 = 1.0 - w1 - w2;
    [w1, w2, w3]
}
