use crate::common::types::{VertexId, Weight};
use crate::common::utils::{
    cmp, into_undirected_graph_tab, read_lines, to_edges_from_xy_position, vertices,
};
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
    let g = into_undirected_graph_tab(&g);
    let vertex_set = VertexSubset::from(&vs);

    // Initialize the cost table `C` with the cost of the home vertex to itself (which is 0).
    let home_vertex = 0;
    let mut cost_accum: HashMap<(VertexSubset, VertexId), Weight> = HashMap::new();
    cost_accum.insert((vertex_set.add(&home_vertex), home_vertex), 0f64);

    // We need to remove the home vertex, since it won't be
    // accounted for in the rest of the path.
    vs.remove(&home_vertex);

    let n = vs.len();
    for i in 1..=n {
        // We compute all the vertex subsets of size `i` (excluding the home vertex),
        // and iterate over each one of them.
        let subsets = vs.iter().permutations(i);
        for subset in subsets {
            // We build an instance of `VertexSubset` which contains all the vertices
            // for this particular vertex subset `subset`, and also includes the home vertex.
            let mut subset_with_home = subset.clone();
            subset_with_home.push(&home_vertex);
            let subset_key = subset_with_home
                .iter()
                .fold(vertex_set.clear_all(), |s, &v| s.add(v));

            // We iterate over all the vertices in this particular vertex subset `subset` to
            // compute the minimum cost from the home vertex up to these.
            for &j in &subset {
                let cost = subset_with_home
                    .par_iter()
                    .filter(|&i| *i != j)
                    .map(|&i| {
                        let key = (subset_key.remove(j), *i);
                        let vertices = (*i, *j);
                        let weight = g.get(&vertices).unwrap();
                        let current_cost = cost_accum.get(&key).or(Some(&MAX)).unwrap();
                        return weight + current_cost;
                    })
                    .min_by(cmp)
                    .or(Some(MAX))
                    .unwrap();
                cost_accum.insert((subset_key.clone(), *j), cost);
            }
        }
    }

    let all_vertices = vertex_set.set_all();
    let min_cost = cost_accum
        .iter()
        .filter(|((s, _), _)| *s == all_vertices)
        .map(|((_, j), cost)| {
            let vertices = (home_vertex, *j);
            let weight = g.get(&vertices).or(Some(&MAX)).unwrap();
            return cost + weight;
        })
        .min_by(cmp);
    match min_cost {
        Some(c) => c.floor() as TspResult,
        None => 0,
    }
}

pub fn solve() {
    let result = solve_for_file("resources/week2/test_cases/input_float_7_3.txt");
    println!("{}", result);
}
