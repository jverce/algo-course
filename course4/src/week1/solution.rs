use std::cmp::min;

use crate::common::utils::{read_lines, to_edges, vertices};
use crate::week1::bellman_ford;
use crate::week1::types::ShortestPaths;

/// Computes the solution to the problem for the file
/// located at `filename`.
pub fn solve_for_file(filename: &str) -> Option<i64> {
    let file_content = read_lines(filename);
    let edges = to_edges(file_content);
    let sources = vertices(&edges);
    let results: Vec<Option<ShortestPaths>> = sources.iter().map(|s| bellman_ford::solve(*s, &edges)).collect();

    let mut result = None;
    for r in results {
        let partial_result = match r {
            Some(t) => t.values().into_iter().map(|i| *i).min(),
            None => break,
        };

        result = if result.is_none() { partial_result } else { min(result, partial_result) };
    };

    return result;
}

pub fn solve() {
    let results = vec![
        solve_for_file("resources/week1/g1.txt"),
        solve_for_file("resources/week1/g2.txt"),
        solve_for_file("resources/week1/g3.txt"),
    ];
    let has_none = results.iter().find(|o| match o {
        None => true,
        _ => false,
    });

    let result = match has_none {
        None => results.iter().map(|o| o.unwrap()).min(),
        _ => None,
    };
    match result {
        Some(r) => println!("{}", r),
        None => println!("NULL"),
    };
}
