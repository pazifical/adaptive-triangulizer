// Community library imports
use image::Rgba;
use spade::delaunay::VertexHandle;

// Crate imports
use crate::point::Point;

pub fn interpolate_triangle_average_color(triangle: &[VertexHandle<Point>; 3]) -> Rgba<u8> {
    let mut color = [0.0; 3];
    for i in 0..3 {
        color[0] += triangle[i].c[0] as f32;
        color[1] += triangle[i].c[1] as f32;
        color[2] += triangle[i].c[2] as f32;
    }
    Rgba([
        (color[0] / 3.0) as u8,
        (color[1] / 3.0) as u8,
        (color[2] / 3.0) as u8,
        255,
    ])
}

pub fn interpolate_rgba_in_triangle(
    point: &Point,
    triangle: &[VertexHandle<Point>; 3],
) -> Rgba<u8> {
    let weights = calc_barycentric_interpolation_weights(point, triangle);

    let mut color = [0.0; 3];
    for i in 0..3 {
        color[0] += weights[i] * (triangle[i].c[0] as f32);
        color[1] += weights[i] * (triangle[i].c[1] as f32);
        color[2] += weights[i] * (triangle[i].c[2] as f32);
    }

    Rgba([color[0] as u8, color[1] as u8, color[2] as u8, 255])
}

fn calc_barycentric_interpolation_weights(
    p: &Point,
    triangle: &[VertexHandle<Point>; 3],
) -> [f32; 3] {
    let v1 = &triangle[0];
    let v2 = &triangle[1];
    let v3 = &triangle[2];
    let w1 = (((v2.y - v3.y) * (p.x - v3.x) + (v3.x - v2.x) * (p.y - v3.y)) as f32)
        / (((v2.y - v3.y) * (v1.x - v3.x) + (v3.x - v2.x) * (v1.y - v3.y)) as f32);
    let w2 = (((v3.y - v1.y) * (p.x - v3.x) + (v1.x - v3.x) * (p.y - v3.y)) as f32)
        / (((v2.y - v3.y) * (v1.x - v3.x) + (v3.x - v2.x) * (v1.y - v3.y)) as f32);
    let w3 = 1.0 - w1 - w2;
    [w1, w2, w3]
}


#[cfg(test)]
mod interpolation_tests {
    use super::*;
    use spade::delaunay::{DelaunayTriangulation, FloatDelaunayTriangulation, DelaunayWalkLocate};
    use spade::kernels::FloatKernel;
    
    type Mesh = DelaunayTriangulation<Point, FloatKernel, DelaunayWalkLocate>;

    fn create_simple_triangulation() -> Mesh {
        let mut delaunay = FloatDelaunayTriangulation::with_walk_locate();
        delaunay.insert(Point {
            x: 0.0,
            y: 0.0,
            c: Rgba([255, 0, 0, 255]),
        });
        delaunay.insert(Point {
            x: 10.0,
            y: 0.0,
            c: Rgba([0, 255, 0, 255]),
        });
        delaunay.insert(Point {
            x: 0.0,
            y: 10.0,
            c: Rgba([0, 0, 255, 255]),
        });
        delaunay
    }

    #[test]
    fn test_calc_barycentric_interpolation_weights() {
        let mesh = create_simple_triangulation();

        let p1 = Point::new(0.0, 0.0, Rgba([255, 0, 0, 255]));
        let p2 = Point::new(10.0, 0.0, Rgba([0, 255, 0, 255]));
        let p3 = Point::new(0.0, 10.0, Rgba([0, 0, 255, 255]));
        let p4 = Point::new(
            (p1.x + p2.x + p3.x)/3.0, 
            (p1.y + p2.y + p3.y)/3.0,
            Rgba([255/3, 255/3, 255/3, 255])    
        );

        for face in mesh.triangles() {
            let triangle = face.as_triangle();

            let weights1 = calc_barycentric_interpolation_weights(&p1, &triangle);
            let weights2 = calc_barycentric_interpolation_weights(&p2, &triangle);
            let weights3 = calc_barycentric_interpolation_weights(&p3, &triangle);
            let weights4 = calc_barycentric_interpolation_weights(&p4, &triangle);

            // The order is not what expected, but nevermind
            // If these first 3 asserts pass, the other tests are valuable! 
            assert_eq!(weights1, [0.0, 0.0, 1.0]);
            assert_eq!(weights2, [1.0, 0.0, 0.0]);
            assert_eq!(weights3, [0.0, 1.0, 0.0]);
            
            assert_eq!(format!("{:.4}", weights4[0]), format!("{:.4}", weights4[1]));
            assert_eq!(format!("{:.4}", weights4[0]), format!("{:.4}", weights4[2]));

            // TODO: Add test for non center non identical point
        } 
    }

    #[test]
    fn test_interpolate_triangle_avg_color() {
        let mesh = create_simple_triangulation();
        for face in mesh.triangles() {
            let triangle = face.as_triangle();
            let c = interpolate_triangle_average_color(&triangle);

            assert_eq!(c, Rgba([255/3, 255/3, 255/3, 255]));
        }
    }
}