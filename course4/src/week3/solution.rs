use crate::common::types::{PointVertex, Weight};
use crate::common::utils::{read_lines, to_points};
use crate::week2::types::TspResult;
use typenum::U2;

pub fn solve_for_file(filename: &str) -> TspResult {
    let file_contents: Vec<Vec<f64>> = read_lines(filename);
    let points: Vec<PointVertex<Weight, U2>> = to_points(file_contents);
    return points.len() as TspResult;
}

pub fn solve() {
    let result = solve_for_file("resources/week3/tsp.txt");
    println!("{}", result);
}
