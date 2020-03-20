use crate::common::types::{VertexId, Weight};
use crate::common::utils::{cmp, read_lines, to_edges_from_xy_position, vertices};
use crate::week2::types::{EnumSet, TspResult, VertexSubset};
use itertools::Itertools;
use std::collections::HashMap;

/// Computes the solution to the TSP problem for the file
/// located at `filename` using dynamic programming.
pub fn solve_for_file(filename: &str) -> TspResult {
    let file_contents: Vec<Vec<Weight>> = read_lines(filename);
    let g = to_edges_from_xy_position(file_contents);
    let mut vs = vertices(&g);
    let s = VertexSubset::from(&vs);

    let home_vertex = 0;
    let mut cost_accum: HashMap<(VertexSubset, VertexId), Weight> = HashMap::new();
    cost_accum.insert((s.add(&home_vertex), 0), 0f64);

    // Insert TSP algorithm here
    //        |
    //        |
    //        V
    let n = vs.len();
    vs.remove(&home_vertex);
    for i in 2..=n {
        let subsets = vs.iter().permutations(i);
        for subset in subsets {
            let vertex_subset = subset
                .iter()
                .fold(s.clear_all(), |s, &v| s.add(v))
                .add(&home_vertex);
            if i == 7 {
                println!("i={}, {:?}", i, vertex_subset);
            }
        }
    }

    let all_vertices = s.set_all();
    let min_cost = cost_accum
        .iter()
        .filter(|((s, _), _)| *s == all_vertices)
        .min_by(|(_, c1), (_, c2)| cmp(c1, c2));

    match min_cost {
        Some((_, c)) => c.round() as TspResult,
        None => 0,
    }
}

pub fn solve() {
    let result = solve_for_file("resources/week2/tsp.txt");
    println!("{}", result);
}
