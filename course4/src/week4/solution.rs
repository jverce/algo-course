use itertools::Itertools;

use crate::common::utils::read_lines;
use crate::week4::types::{BitSet, BitVec};

pub fn solve_for_file(filename: &str) -> bool {
    let file_contents: Vec<Vec<i64>> = read_lines(filename);
    let n_bits = file_contents[0][0].clone() as usize;
    let candidate: BitVec = BitSet::new(n_bits);

    return false;
}

pub fn solve() {
    let filenames = (1..=6).map(|i| format!("resources/week4/2sat{}.txt", i));
    let result = filenames
        .map(|f| solve_for_file(&f))
        .map(|r| r.into())
        .map(|r: i32| r.to_string())
        .join("");
    println!("{}", result);
}
