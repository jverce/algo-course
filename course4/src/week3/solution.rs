use crate::common::types::Weight;
use crate::common::utils::read_lines;
use crate::week2::types::TspResult;

pub fn solve_for_file(filename: &str) -> TspResult {
    println!("reading file...");
    let file_contents: Vec<Vec<Weight>> = read_lines(filename);
    return 0;
}

pub fn solve() {
    let result = solve_for_file("resources/week3/tsp.txt");
    println!("{}", result);
}
