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
    Rgba([(color[0]/3.0) as u8, (color[1]/3.0) as u8, (color[2]/3.0) as u8, 255])
}

pub fn interpolate_rgba_in_triangle(point: &Point, triangle: &[VertexHandle<Point>; 3]) -> Rgba<u8> {
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
