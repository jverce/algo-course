use crate::common::utils::{read_lines, to_edges};
use crate::week1::bellman_ford;

/// Computes the solution to the problem for the file
/// located at `filename`.
pub fn solve_for_file(filename: &str) -> Option<i64> {
    let file_content = read_lines(filename);
    let edges = to_edges(file_content);
    let source = 1;
    let result = bellman_ford::solve(source, edges);

    return match result {
        Some(t) => t.values().into_iter().map(|i| *i).min(),
        None => None,
    };
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
