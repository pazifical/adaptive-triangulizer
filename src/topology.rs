// Community library imports
use spade::delaunay::VertexHandle;

// Crate imports
use crate::point::Point;


fn is_ccw(p1: &Point, p2: &Point, p3: &Point) -> bool {
    (p3.y - p1.y) * (p2.x - p1.x) >= (p2.y - p1.y) * (p3.x - p1.x)
}

pub fn is_point_in_triangle(point: &Point, triangle: &[VertexHandle<Point>; 3]) -> bool {
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


#[cfg(test)]
mod topology_tests {
    use image::Rgba;
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
        }
    }
}