use crate::week1::types;
use std::collections::HashMap;

/// Computes a table with the shortest-path distances from vertex `s`
/// to all other vertices in the graph `g` using the Bellman-Ford algorithm.
/// The table is indexed by destination vertex **ID**, and the value
/// associated to it is the length of the sortest path from the
/// vertex `s` to said destination vertex.
pub fn solve(s: u64, g: Vec<types::Edge>) -> HashMap<u64, i64> {
    let res: HashMap<u64, i64> = HashMap::new();
    return res;
}
