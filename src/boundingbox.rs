// Community library imports
use spade::delaunay::VertexHandle;

// Crate imports
use crate::point::Point;

pub struct BoundingBox {
    pub xmin: f32,
    pub xmax: f32,
    pub ymin: f32,
    pub ymax: f32,
}

impl BoundingBox {
    pub fn from_triangle(triangle: &[VertexHandle<Point>; 3]) -> BoundingBox {
        let mut xmin = 100000.;
        let mut xmax = -100000.;
        let mut ymin = 100000.;
        let mut ymax = -100000.;

        for point in triangle.iter() {
            if point.x < xmin {
                xmin = point.x;
            }
            if point.x > xmax {
                xmax = point.x;
            }
            if point.y < ymin {
                ymin = point.y;
            }
            if point.y > ymax {
                ymax = point.y;
            }
        }
        BoundingBox {
            xmin,
            xmax,
            ymin,
            ymax,
        }
    }
}

#[cfg(test)]
mod boundingbox_tests {
    use image::Rgba;
    use spade::delaunay::{DelaunayTriangulation, FloatDelaunayTriangulation, DelaunayWalkLocate};
    use spade::kernels::FloatKernel;
    use super::*;

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
    fn test_creation() {
        let (xmin, xmax, ymin, ymax) = (0.0, 1.0, 2.0, 3.0);
        let bbox = BoundingBox {
            xmin,
            xmax,
            ymin,
            ymax
        };

        assert_eq!(bbox.xmin, xmin);
        assert_eq!(bbox.xmax, xmax);
        assert_eq!(bbox.ymin, ymin);
        assert_eq!(bbox.ymax, ymax);
    }

    #[test]
    fn test_creation_from_triangle() {
        let mesh = create_simple_triangulation();

        for face in mesh.triangles() {
            let triangle = face.as_triangle();
            let bbox = BoundingBox::from_triangle(&triangle);

            assert_eq!(bbox.xmin, 0.0);
            assert_eq!(bbox.xmax, 10.0);
            assert_eq!(bbox.ymin, 0.0);
            assert_eq!(bbox.ymax, 10.0);
        }
    }
}