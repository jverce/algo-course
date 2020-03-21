use crate::common::types::{VertexId, Weight};
use crate::common::utils::{cmp, read_lines, to_edges_from_xy_position, vertices};
use crate::week2::types::{EnumSet, TspResult, VertexSubset};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use std::f64::MAX;

/// Computes the solution to the TSP problem for the file
/// located at `filename` using dynamic programming.
pub fn solve_for_file(filename: &str) -> TspResult {
    let file_contents: Vec<Vec<Weight>> = read_lines(filename);
    let g = to_edges_from_xy_position(file_contents);
    let mut vs = vertices(&g);
    let vertex_set = VertexSubset::from(&vs);

    let home_vertex = 0;
    let mut cost_accum: HashMap<(VertexSubset, VertexId), Weight> = HashMap::new();
    cost_accum.insert((vertex_set.add(&home_vertex), 0), 0f64);

    // We need to remove the home vertex, since it won't be
    // accounted for in the rest of the path.
    vs.remove(&home_vertex);

    let n = vs.len();
    for i in 1..=n {
        // We compute all the vertex subsets of size `i - 1`, and iterate over each one of them.
        let subsets = vs.iter().permutations(i - 1);
        println!("i={}", i);
        for subset in subsets {
            // We build an instance of `VertexSubset` which contains all the vertices
            // for this particular vertex subset `subset`, and also includes the home vertex.
            let s = subset
                .iter()
                .fold(vertex_set.clear_all(), |s, &v| s.add(v))
                .add(&home_vertex);

            // We iterate over all the vertices in this particular vertex subset `subset` to
            // compute the minimum cost from the home vertex up to these.
            for &j in &subset {
                let cost = subset
                    .par_iter()
                    .filter(|&i| *i != j)
                    .map(|&i| cost_accum.get(&(s.remove(j), *i)).or(Some(&MAX)).unwrap() + -1f64)
                    .min_by(cmp)
                    .or(Some(-1f64))
                    .unwrap();
                cost_accum.insert((s.clone(), *j), cost);
            }
        }
    }

    let all_vertices = vertex_set.set_all();
    let min_cost = cost_accum.get(&(all_vertices, home_vertex));
    match min_cost {
        Some(c) => c.round() as TspResult,
        None => 0,
    }
}

pub fn solve() {
    let result = solve_for_file("resources/week2/test_cases/input_float_13_5.txt");
    println!("{}", result);
}
