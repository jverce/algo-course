use crate::week2::types::TspResult;

/// Computes the solution to the problem for the file
/// located at `filename` using the Floyd-Warshall algorithm.
pub fn solve_for_file(filename: &str) -> TspResult {
    return 0;
}

pub fn solve() {
    let result = solve_for_file("resources/week2/tsp.txt");
    println!("{}", result);
}
