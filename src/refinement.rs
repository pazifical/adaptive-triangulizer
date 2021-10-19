// Community library imports
use spade::kernels::FloatKernel;
use spade::delaunay::{DelaunayTriangulation, DelaunayWalkLocate, VertexHandle};
use image::{DynamicImage, GenericImageView};

// Crate imports
use crate::point::Point;
use crate::interpolation::interpolate_triangle_centroid;

// Type definitions
type Mesh = DelaunayTriangulation<Point, FloatKernel, DelaunayWalkLocate>;


pub fn refine_mesh_by_centroid(mesh: &mut Mesh, img: &DynamicImage, max_diff: i32) {
    let mut new_points = Vec::new();

    for face in mesh.triangles() {
        let triangle = face.as_triangle();
        let mut centroid = interpolate_triangle_centroid(&triangle);
        let orig_color = img.get_pixel(centroid.x as u32, centroid.y as u32);

        let diff_r = ((centroid.c[0] as i32) - (orig_color[0] as i32)).abs();
        let diff_g = ((centroid.c[1] as i32) - (orig_color[1] as i32)).abs();
        let diff_b = ((centroid.c[2] as i32) - (orig_color[2] as i32)).abs();

        if diff_r > max_diff || diff_b > max_diff || diff_b > max_diff {
            centroid.c = orig_color;
            new_points.push(centroid);
        }
    }

    for point in new_points {
        mesh.insert(point);
    }
}

