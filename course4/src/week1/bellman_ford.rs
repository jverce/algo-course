use crate::common::utils::{to_indeg_edges, vertices};
use crate::week1::types;
use std::collections::HashMap;

/// Computes the optimization function for the Bellman-Ford
/// algorithm.
fn opt(prev: i64, indeg: &[&types::Edge]) -> i64 {
    return prev;
}

/// Computes a table with the shortest-path distances from vertex `s`
/// to all other vertices in the graph `g` using the Bellman-Ford algorithm.
/// The table is indexed by destination vertex **ID**, and the value
/// associated to it is the length of the sortest path from the
/// vertex `s` to said destination vertex.
pub fn solve(s: u64, g: Vec<types::Edge>) -> HashMap<u64, i64> {
    let n = g.len();
    let vs = vertices(g.as_slice());
    let indeg = to_indeg_edges(g.as_slice());
    let mut result: HashMap<u64, i64> = vs.iter().map(|v| (*v, std::i64::MAX)).collect();

    for _ in 1..n {
        result
            .iter_mut()
            .for_each(|(k, v)| *v = opt(*v, indeg[k].as_slice()));
    }

    return result;
}
