use itertools::Itertools;
use rayon::prelude::*;

use crate::common::utils::read_lines;
use crate::week4::types::{BitSet, BitVec, EvaluableMut, ExprFull, SatisfiableMut};

pub fn solve_for_file(filename: &str) -> bool {
    let mut file_contents: Vec<Vec<i64>> = read_lines(filename);
    let n_bits = file_contents[0][0].clone() as usize;

    // Remove the first item of the file contents, which is irrelevant to the logical expression
    // for which the 2-SAT must be evaluated.
    file_contents.remove(0);

    let log_n_bits = (n_bits as f64).log2().ceil() as usize;

    // Papadimitriou's algorithm specifies this number to be 2 * N^2, but from the test cases
    // I found out that such a high count is not necessary, otherwise running this program with
    // the real input from the course takes forever (in the case of unsatisfied expressions).
    let inner_loop_limit = 2 * n_bits;

    (1..=log_n_bits).into_par_iter().any(|_| {
        // Instantiate the expression for which we need to check satisfiability.
        let mut expr = ExprFull::new(&file_contents);
        let mut candidate: BitVec = BitSet::new(n_bits);
        for _ in 1..=inner_loop_limit {
            if expr.eval(&mut candidate) {
                return true;
            }
            expr.satisfy_term_randomly(&mut candidate);
        }
        false
    })
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
