use std::println;
use std::collections::HashMap;
use crate::common::utils::read_lines;

pub fn bellman_ford(t: u64, g: HashMap<u64, i64>) -> HashMap<u64, i64> {
    return HashMap::new();
}

fn solve(filename: &str) {
    let file_content = read_lines(filename);
    let file_size = file_content.len();
    println!("{}", file_size);
    println!("{}", file_content[2][2]);
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
