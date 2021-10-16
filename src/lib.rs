use std::ops::Bound;
use std::path::PathBuf;
use image::io::Reader as ImageReader;
use image::{Rgb, Rgba, DynamicImage, GenericImageView, ImageBuffer, RgbImage, Pixel, RgbaImage};
use rand::Rng;
use spade::delaunay::{DelaunayTriangulation, DelaunayWalkLocate, FloatDelaunayTriangulation, VertexHandle};
use spade::{PointN, TwoDimensional};
use spade::kernels::{FloatKernel, TrivialKernel};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: f32,
    y: f32,
    c: Rgba<u8>
}

impl Point {
    fn new(x: f32, y: f32, c: Rgba<u8>) -> Point {
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


pub fn run() {
    println!("Hello, world!");

    let path = PathBuf::from("/home/minerva/Programming/Trimage/data/2017_China_Chongqing_Boats.jpg");
    let path_out = PathBuf::from("/home/minerva/Programming/Trimage/temp/2017_China_Chongqing_Boats.jpg");

    let img = ImageReader::open(&path).unwrap().decode().unwrap();
    println!("{:?}", img.get_pixel(0, 0));
    println!("{:?}", img.get_pixel(0, 0).to_luma());

    let (width, height) = img.dimensions();
    println!("img dimensions: {:?}", img.dimensions());

    let mut delaunay = FloatDelaunayTriangulation::with_walk_locate();

    let mut rng = rand::thread_rng();
    for _ in 0..5000 {
        let rnd_x = rng.gen::<f32>() * (width as f32);
        let rnd_y = rng.gen::<f32>() * (height as f32);

        delaunay.insert(
            Point {
                x: rnd_x,
                y: rnd_y,
                c: img.get_pixel(rnd_x as u32, rnd_y as u32)
            }
        );
    }

    // for y in (0..height).step_by(20) {
    //     for x in (0..width).step_by(20) {
    //         delaunay.insert(
    //             Point {
    //                 x: x as f32,
    //                 y: y as f32,
    //                 c: img.get_pixel(x, y)
    //             }
    //         );
    //     }
    // }

    let img = rasterize_mesh(&delaunay, width as i32, height as i32);

    img.save(path_out).unwrap();
}


type Mesh = DelaunayTriangulation<Point, FloatKernel, DelaunayWalkLocate>;
// type Element = [VertexHandle<Point>; 3];

fn rasterize_mesh(mesh: &Mesh, width: i32, height: i32) -> RgbaImage {
    let mut img = ImageBuffer::new(width as u32, height as u32);
    for face in mesh.triangles() {
        let triangle = face.as_triangle();
        let bbox = BoundingBox::from_triangle(&triangle);

        for row in (bbox.ymin.floor() as u32)..(bbox.ymax.ceil() as u32) {
            for col in (bbox.xmin.floor() as u32)..(bbox.xmax.ceil() as u32) {
                let point = Point::new(col as f32, row as f32, Rgba([255, 255, 255, 255]));

                if is_point_in_triangle(&point, &triangle) {
                    // let color = interpolate_rgba_in_triangle(&point, &triangle);
                    let color = interpolate_triangle_average_color(&triangle);
                    img.put_pixel(col, row, color);
                }
            }
        }
    }
    RgbaImage::from(img)
}

struct BoundingBox {
    xmin: f32,
    xmax: f32,
    ymin: f32,
    ymax: f32
}

impl BoundingBox {
    pub fn from_triangle(triangle: &[VertexHandle<Point>; 3]) -> BoundingBox {
        let mut xmin = 100000.;
        let mut xmax = -100000.;
        let mut ymin = 100000.;
        let mut ymax = -100000.;

        for point in triangle.iter() {
            if point.x < xmin { xmin = point.x; }
            if point.x > xmax { xmax = point.x; }
            if point.y < ymin { ymin = point.y; }
            if point.y > ymax { ymax = point.y; }
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

fn interpolate_triangle_average_color(triangle: &[VertexHandle<Point>; 3]) -> Rgba<u8> {
    let mut color = [0.0; 3];
    for i in 0..3 {
        color[0] += triangle[i].c[0] as f32;
        color[1] += triangle[i].c[1] as f32;
        color[2] += triangle[i].c[2] as f32;
    }
    Rgba([(color[0]/3.0) as u8, (color[1]/3.0) as u8, (color[2]/3.0) as u8, 255])
}

fn interpolate_rgba_in_triangle(point: &Point, triangle: &[VertexHandle<Point>; 3]) -> Rgba<u8> {
    let weights = calc_barycentric_interpolatin_weights(point, triangle);

    let mut color = [0.0; 3];
    for i in 0..3 {
        color[0] += weights[i] * (triangle[i].c[0] as f32);
        color[1] += weights[i] * (triangle[i].c[1] as f32);
        color[2] += weights[i] * (triangle[i].c[2] as f32);
    }

    Rgba([color[0] as u8, color[1] as u8, color[2] as u8, 255])
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_in_triangle() {
        let point_in = Point::new(5.0, 5.0, Rgba([255, 255, 255, 255]));
        let point_out = Point::new(15.0, 15.0, Rgba([255, 255, 255, 255]));

        let mut delaunay = FloatDelaunayTriangulation::with_walk_locate();
        delaunay.insert(
            Point {
                x: 0.0,
                y: 0.0,
                c: Rgba([255, 0, 0, 255])
            }
        );
        delaunay.insert(
            Point {
                x: 10.0,
                y: 0.0,
                c: Rgba([0, 255, 0, 255])
            }
        );
        delaunay.insert(
            Point {
                x: 10.0,
                y: 10.0,
                c: Rgba([0, 0, 255, 255])
            }
        );

        for face in delaunay.triangles() {
            let triangle = face.as_triangle();
            println!("Found triangle: {:?} -> {:?} -> {:?}", *triangle[0], *triangle[1], *triangle[2]);
            assert_eq!(is_point_in_triangle(&point_in, &triangle), true);
            assert_eq!(is_point_in_triangle(&point_out, &triangle), false);

            let bbox = BoundingBox::from_triangle(&triangle);
            assert_eq!(bbox.xmin, 0.0);
            assert_eq!(bbox.xmax, 10.0);
            assert_eq!(bbox.ymin, 0.0);
            assert_eq!(bbox.ymax, 10.0);
        }
    }
}