use crate::common::utils::{to_indeg_edges, vertices};
use crate::week1::types;
use std::cmp::min;
use std::collections::HashMap;
use std::i64::MAX;

type ShortestPaths = HashMap<u64, i64>;

/// Computes the optimization function for the Bellman-Ford
/// algorithm.
fn opt(prev: i64, indeg_prev: &[i64], indeg: &[&types::Edge]) -> i64 {
    let n_indeg = indeg_prev.len();
    let mut min_indeg = prev;
    for i in 0..n_indeg {
        let new_cost = indeg[i].weight.saturating_add(indeg_prev[i]);
        min_indeg = min(min_indeg, new_cost);
    }
    return min_indeg;
}

/// Computes a table with the shortest-path distances from vertex `s`
/// to all other vertices in the graph `g` using the Bellman-Ford algorithm.
/// The table is indexed by destination vertex **ID**, and the value
/// associated to it is the length of the sortest path from the
/// vertex `s` to said destination vertex.
pub fn solve(s: u64, g: Vec<types::Edge>) -> Option<ShortestPaths> {
    let n = g.len();
    let vs = vertices(&g);
    let indeg = to_indeg_edges(&g);
    let mut result: ShortestPaths = vs
        .iter()
        .map(|&v| if v == s { (v, 0) } else { (v, MAX) })
        .collect();

    for _ in 1..n {
        let old_result: ShortestPaths = result.clone();
        result = result
            .iter()
            .map(|(k, v)| {
                let indeg = indeg[k].as_slice();
                let indeg_prev: Vec<i64> = indeg.iter().map(|&v| result[&v.tail]).collect();
                return (*k, opt(*v, &indeg_prev, indeg));
            })
            .collect();

        if old_result == result {
            break;
        }
    }

    return Some(result);
}
