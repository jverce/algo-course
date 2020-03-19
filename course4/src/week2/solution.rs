use crate::common::utils::{read_lines, to_edges_from_xy_position, vertices};
use crate::week2::types::{TspResult, VertexSubset};

/// Computes the solution to the TSP problem for the file
/// located at `filename` using dynamic programming.
pub fn solve_for_file(filename: &str) -> TspResult {
    let file_contents: Vec<Vec<f64>> = read_lines(filename);
    let g = to_edges_from_xy_position(file_contents);
    let vs = vertices(&g);
    let s = VertexSubset::from(&vs);
    return 0;
}

pub fn solve() {
    let result = solve_for_file("resources/week2/tsp.txt");
    println!("{}", result);
}
