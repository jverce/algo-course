use rayon::prelude::*;
use std::cmp::min_by;
use std::collections::HashMap;
use std::f64::MAX;

use crate::common::utils::{cmp, to_outdeg_edges, vertices};
use crate::week1::types::{Graph, ShortestPathsFW, VertexId, Weight};

fn opt(path_weights: &ShortestPathsFW, i: VertexId, j: VertexId, k: VertexId) -> Weight {
    let key = (i, j);
    let curr_weight = path_weights.get(&key).or(Some(&MAX)).unwrap();

    let rhs_key = (i, k);
    let rhs_weight = path_weights.get(&rhs_key).or(Some(&MAX)).unwrap();

    let lhs_key = (k, j);
    let lhs_weight = path_weights.get(&lhs_key).or(Some(&MAX)).unwrap();

    let new_weight = rhs_weight + lhs_weight;
    return min_by(*curr_weight, new_weight, cmp);
}

/// Computes the shortest-path distances of all the pairs
/// of vertices in the graph `g` using the Floyd-Warshall algorithm
/// The table is indexed by the source and destination
/// vertex **IDs** as a tuple `(s, t)`, and the value
/// associated to it is the length of the shortest path from the
/// vertex `s` to the destination vertex `t`.
pub fn solve(g: &Graph) -> ShortestPathsFW {
    let vs = vertices(&g);
    let n = vs.len();
    let outdeg = to_outdeg_edges(&g);
    let mut path_weights: ShortestPathsFW = HashMap::new();

    // Initialize all vertex paths to themselves, set them to 0.
    for i in 1..=n {
        let key = (i, i);
        path_weights.insert(key, 0f64);
    }

    // Initialize all existing direct paths from `i` to `j`.
    // This will override all preset self-loops from above (which is good).
    for (i, edges) in outdeg {
        for e in edges {
            let key = (i, e.head);
            let curr_weight = path_weights.get(&key).or(Some(&MAX)).unwrap();
            if curr_weight > &e.weight {
                path_weights.insert(key, e.weight);
            }
        }
    }

    // Compute shortest-paths for k = 1, 2, ..., n.
    for k in 1..=n {
        for i in 1..=n {
            let new_weights: Vec<Weight> = (1..=n)
                .into_par_iter()
                .map(|j| opt(&path_weights, i, j, k))
                .collect();

            for j in 1..=n {
                let key = (i, j);
                path_weights.insert(key, new_weights[j - 1]);
            }
        }
    }

    return path_weights;
}
