// Standard library imports
use std::path::Path;

// Community library imports
use image::DynamicImage;
use image::io::Reader as ImageReader;


pub fn import_image_from_path(path: &Path) -> DynamicImage {
    let img_reader = match ImageReader::open(path) {
        Ok(reader) => reader,
        Err(err) => {
            eprintln!("ERROR: {}.", err);
            eprintln!("Exiting program.");
            std::process::exit(1);
        }
    };

    match img_reader.decode() {
        Ok(img) => img,
        Err(err) => {
            eprintln!("ERROR: {}", err);
            eprintln!("Exiting program.");
            std::process::exit(1);
        }
    }
}