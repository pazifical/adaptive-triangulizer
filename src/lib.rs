// Modules
mod boundingbox;
mod config;
mod interpolation;
mod io;
mod point;
mod rasterization;
mod refinement;
mod topology;

// Standard library imports
use std::path::PathBuf;

// Community library imports
use image::{DynamicImage, GenericImageView};
use rand::Rng;
use spade::delaunay::{DelaunayTriangulation, DelaunayWalkLocate, FloatDelaunayTriangulation};
use spade::kernels::FloatKernel;

// Crate imports
use crate::config::Config;
use crate::io::import_image_from_path;
use crate::point::Point;
use crate::refinement::refine_mesh_by_centroid;
use crate::rasterization::rasterize_mesh;

// Type definitions
type Mesh = DelaunayTriangulation<Point, FloatKernel, DelaunayWalkLocate>;

// Starting point of command line program
// TODO: Put this into a separate binary. After all, this should be a library
pub fn run() {
    // Step 0: Parsing command line arguments
    // TODO: Parse command line arguments to assign these variables
    let path =
        PathBuf::from("./data/2017_China_Chongqing_Boats.jpg");
    let path_out =
        PathBuf::from("./data/out/2017_China_Chongqing_Boats.jpg");
    let n_initial_points = 100;
    let iterations = 4;
    let max_diff = 15;

    // Step 1: Importing the image from a given path
    let img = import_image_from_path(&path);

    // Step 2: Creating a Mesh of the input image by delaunay triangulation
    let mut delaunay = delaunay_of_random_image_pixels(&img, n_initial_points);

    // Step 3: Refining the Mesh
    for _ in 0..iterations {
        refine_mesh_by_centroid(&mut delaunay, &img, max_diff);
    }

    // Step 4: Creating an image from the mesh by rasterization
    let img_out = rasterize_mesh(&delaunay, img.width() as i32, img.height() as i32);
    img_out.save(path_out).unwrap();
}

fn delaunay_of_random_image_pixels(img: &DynamicImage, n_points: usize) -> Mesh {
    let (width, height) = img.dimensions();
    let mut delaunay = FloatDelaunayTriangulation::with_walk_locate();

    delaunay.insert(Point::new(0.0, 0.0, img.get_pixel(0, 0)));
    delaunay.insert(Point::new((width-1) as f32, 0.0, img.get_pixel(width-1, 0)));
    delaunay.insert(Point::new(0.0, (height-1) as f32, img.get_pixel(0, height-1)));
    delaunay.insert(Point::new((width-1) as f32, (height-1) as f32, img.get_pixel(width-1, height-1)));

    let mut rng = rand::thread_rng();
    for _ in 0..(n_points-4) {
        let rnd_x = rng.gen::<f32>() * (width as f32);
        let rnd_y = rng.gen::<f32>() * (height as f32);

        delaunay.insert(Point {
            x: rnd_x,
            y: rnd_y,
            c: img.get_pixel(rnd_x as u32, rnd_y as u32),
        });
    }
    delaunay
}

// TODO: Add command line parsing here
fn parse_arguments() -> Config {
    Config {
        path_in: PathBuf::from("a"),
    }
}
