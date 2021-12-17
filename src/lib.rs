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
use clap::{Arg, App};

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
    let config = parse_arguments();

    // Step 1: Importing the image from a given path
    let img = import_image_from_path(&config.path_in);

    // Step 2: Creating a Mesh of the input image by delaunay triangulation
    let mut delaunay = delaunay_of_random_image_pixels(&img, config.n_initial_points);

    // Step 3: Refining the Mesh
    for _ in 0..config.n_iterations {
        refine_mesh_by_centroid(&mut delaunay, &img, config.max_diff);
    }

    // Step 4: Creating an image from the mesh by rasterization
    let img_out = rasterize_mesh(
        &delaunay,
        img.width() as i32,
        img.height() as i32
    );
    img_out.save(config.path_out).unwrap();
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

fn parse_arguments() -> Config {
    let matches = App::new("trimage")
        .version("1.0")
        .author("Paz Vi <paz@twowaysix.com>")
        .about("AdapTri - Adaptive Triangulizer")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .help("The input image path.")
            .takes_value(true))
        .arg(Arg::with_name("points")
            .short("p")
            .long("points")
            .help("Number of initial mesh nodes.")
            .takes_value(true)
            .default_value("100"))
        .arg(Arg::with_name("iterations")
            .short("i")
            .long("iterations")
            .help("Number of iterations.")
            .takes_value(true)
            .default_value("4"))
        .arg(Arg::with_name("max_diff")
            .short("d")
            .long("max_diff")
            .help("Maximum difference for no refinement.")
            .takes_value(true)
            .default_value("15"))
        .get_matches();

    let path_in = PathBuf::from(matches.value_of("file").unwrap());
    let path_out = path_in.parent().unwrap().join(
        PathBuf::from(
            format!(
                "{}_triangulized.{}",
                path_in.file_stem().unwrap().to_str().unwrap(),
                path_in.extension().unwrap().to_str().unwrap()
            )
        )
    );

    // TODO: Handle wrong input better
    let n_initial_points =matches.value_of("points").unwrap().parse::<usize>().unwrap_or(100);
    let n_iterations = matches.value_of("iterations").unwrap().parse::<i32>().unwrap_or(4);
    let max_diff = matches.value_of("max_diff").unwrap().parse::<i32>().unwrap_or(15);

    println!("\nChosen parameters:");
    println!("- Input path:       {}", path_in.to_str().unwrap());
    println!("- Out path:         {}", path_out.to_str().unwrap());
    println!("- Initial nodes:    {}", n_initial_points);
    println!("- Iterations:       {}", n_iterations);
    println!("- Max. difference:  {}", max_diff);

    Config { path_in, path_out, n_initial_points, n_iterations, max_diff }
}
