use std::cmp::min_by;
use std::f64::MAX;

use crate::common::types::{Edge, Graph, VertexId, Weight};
use crate::common::utils::{cmp, to_indeg_edges, vertices};
use crate::week1::types::ShortestPathsBF;

/// Computes the optimization function for the Bellman-Ford
/// algorithm.
fn opt(prev: Weight, indeg_prev: &[Weight], indeg: &[&Edge]) -> Weight {
    let n_indeg = indeg_prev.len();
    let mut min_indeg = prev;
    for i in 0..n_indeg {
        let new_cost = indeg[i].weight + indeg_prev[i];
        min_indeg = min_by(min_indeg, new_cost, cmp);
    }
    return min_indeg;
}

/// Computes a table with the shortest-path distances from vertex `s`
/// to all other vertices in the graph `g` using the Bellman-Ford algorithm.
/// The table is indexed by destination vertex **ID**, and the value
/// associated to it is the length of the shortest path from the
/// vertex `s` to said destination vertex.
pub fn solve(s: VertexId, g: &Graph) -> Option<ShortestPathsBF> {
    let vs = vertices(&g);
    let n = vs.len();
    let empty: Vec<&Edge> = Vec::new();
    let indeg = to_indeg_edges(&g);
    let mut result: ShortestPathsBF = vs
        .iter()
        .map(|&v| if v == s { (v, 0f64) } else { (v, MAX) })
        .collect();

    let mut not_finished = true;
    for _ in 0..n {
        not_finished = false;
        result = result
            .iter()
            .map(|(k, v)| {
                let indeg = indeg.get(k).unwrap_or(&empty);
                let indeg_prev: Vec<Weight> = indeg.iter().map(|&v| result[&v.tail]).collect();
                let new_val = opt(*v, &indeg_prev, indeg);
                not_finished = not_finished || new_val.ne(v);
                return (*k, new_val);
            })
            .collect();

        if !not_finished {
            break;
        }
    }

    return if not_finished { None } else { Some(result) };
}
