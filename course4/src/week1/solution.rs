use crate::common::utils::{read_lines, to_edges};
use crate::week1::bellman_ford;

/// Computes the solution to the problem for the file
/// located at `filename`.
fn solve(filename: &str) {
    let file_content = read_lines(filename);
    let edges = to_edges(file_content);
    let source = 1;
    bellman_ford::solve(source, edges);
}

pub fn graph1() {
    const FILENAME: &str = "resources/week1/g1.txt";
    solve(FILENAME);
}

pub fn graph2() {
    const FILENAME: &str = "resources/week1/g2.txt";
    solve(FILENAME);
}

pub fn graph3() {
    const FILENAME: &str = "resources/week1/g3.txt";
    solve(FILENAME);
}
