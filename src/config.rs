// Standard library imports
use std::path::PathBuf;

pub struct Config {
    pub path_in: PathBuf,
    pub path_out: PathBuf,
    pub n_initial_points: usize,
    pub n_iterations: i32,
    pub max_diff: i32,
}
