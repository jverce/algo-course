use crate::common::utils;
use crate::week1::bellman_ford;

fn solve(filename: &str) {
    let file_content = utils::read_lines(filename);
    let edges = utils::to_edges(file_content);
    let indeg_edges = utils::to_indeg_edges(&edges);
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
