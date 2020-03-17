use rayon::prelude::*;
use std::cmp::min_by;
use std::f64::MAX;

use crate::common::types::Weight;
use crate::common::utils::{cmp, read_lines, to_edges, vertices};
use crate::week1::types::ShortestPathsBF;
use crate::week1::{bellman_ford, floyd_warshall};

/// Computes the solution to the problem for the file
/// located at `filename` using the Bellman-Ford algorithm.
pub fn solve_for_file_bf(filename: &str) -> Option<Weight> {
    let file_content = read_lines(filename);
    let edges = to_edges(file_content);
    let sources = vertices(&edges);
    let results: Vec<Option<ShortestPathsBF>> = sources
        .par_iter()
        .map(|s| bellman_ford::solve(*s, &edges))
        .collect();

    let mut result = None;
    for r in results {
        let partial_result = match r {
            Some(t) => t.values().into_iter().map(|i| *i).min_by(cmp),
            None => break,
        };

        result = if result.is_none() {
            partial_result
        } else {
            min_by(result, partial_result, cmp)
        };
    }

    return result;
}

/// Computes the solution to the problem for the file
/// located at `filename` using the Floyd-Warshall algorithm.
pub fn solve_for_file_fw(filename: &str) -> Option<Weight> {
    let file_content = read_lines(filename);
    let edges = to_edges(file_content);
    let result = floyd_warshall::solve(&edges);
    let n = vertices(&edges).len();

    for i in 1..=n {
        let key = (i, i);
        let val = result.get(&key).unwrap();
        if val.ne(&0f64) {
            return None;
        }
    }

    return result.values().min_by(cmp).or(Some(&MAX)).cloned();
}

pub fn solve() {
    let results = vec![
        solve_for_file_bf("resources/week1/g1.txt"),
        solve_for_file_bf("resources/week1/g2.txt"),
        solve_for_file_bf("resources/week1/g3.txt"),
    ];
    let result = results
        .iter()
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .min_by(cmp);

    match result {
        Some(r) => println!("{}", r),
        None => println!("NULL"),
    };
}
