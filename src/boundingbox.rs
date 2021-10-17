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
