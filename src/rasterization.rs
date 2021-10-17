// Community library imports
use image::{ImageBuffer, Rgba, RgbaImage};
use spade::delaunay::{DelaunayTriangulation, DelaunayWalkLocate};
use spade::kernels::FloatKernel;

// Crate imports
use crate::boundingbox::BoundingBox;
use crate::interpolation::interpolate_triangle_average_color;
use crate::point::Point;
use crate::topology::is_point_in_triangle;

// Type definitions
type Mesh = DelaunayTriangulation<Point, FloatKernel, DelaunayWalkLocate>;
// type Element = [VertexHandle<Point>; 3];


pub fn rasterize_mesh(mesh: &Mesh, width: i32, height: i32) -> RgbaImage {
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
