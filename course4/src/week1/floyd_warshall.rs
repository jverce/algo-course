use rayon::prelude::*;
use std::collections::HashMap;
use std::i64::MAX;

use crate::common::utils::{to_outdeg_edges, vertices};
use crate::week1::types::{Graph, VertexId};

/// Computes the shortest-path distances of all the pairs
/// of vertices in the graph `g` using the Floyd-Warshall algorithm
/// The table is indexed by the source and destination
/// vertex **IDs** as a tuple `(s, t)`, and the value
/// associated to it is the length of the shortest path from the
/// vertex `s` to the destination vertex `t`.
pub fn solve(g: &Graph) -> HashMap<(VertexId, VertexId), i64> {
    let vs = vertices(&g);
    let n = vs.len();
    let outdeg = to_outdeg_edges(&g);
    let mut path_weights: HashMap<(VertexId, VertexId), i64> = HashMap::new();

    for i in 1..=n {
        let key = (i, i);
        path_weights.insert(key, 0);
    }

    for (i, edges) in outdeg {
        for e in edges {
            let key = (i, e.head);
            let curr_weight = path_weights.get(&key).or(Some(&MAX)).unwrap();
            if curr_weight > &e.weight {
                path_weights.insert(key, e.weight);
            }
        }
    }

    for k in 1..=n {
        for i in 1..=n {
            for j in 1..=n {
                let key = (i, j);
                let curr_weight = path_weights.get(&key).or(Some(&MAX)).unwrap();

                let rhs_key = (i, k);
                let rhs_weight = path_weights.get(&rhs_key).or(Some(&MAX)).unwrap();

                let lhs_key = (k, j);
                let lhs_weight = path_weights.get(&lhs_key).or(Some(&MAX)).unwrap();

                let new_weight = rhs_weight.saturating_add(*lhs_weight);
                if curr_weight > &new_weight {
                    path_weights.insert(key, new_weight);
                }
            }
        }
    }

    return path_weights;
}
