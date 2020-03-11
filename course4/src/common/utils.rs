use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::week1::types;

/// The `str` content in each lines is split in each space
/// character, and each of these components is parse and
/// transformed into an integer and put into a vector.
fn convert_to_int_vector(line: &str) -> Vec<i64> {
    return line.split(' ').map(|i| i.parse::<i64>().unwrap()).collect();
}

/// Reads all the lines in the file located at `filename`,
/// and returns these as a collection, where each item
/// represents each line.
/// The `str` content in each lines is split in each space
/// character, and each of these components is put into a vector.
pub fn read_lines(filename: &str) -> Vec<Vec<i64>> {
    let fd = File::open(filename).unwrap();
    let reader = BufReader::new(fd);
    let mut res: Vec<Vec<i64>> = Vec::new();
    for i in reader.lines() {
        let as_str: &str = &&i.unwrap();
        let as_int_vec = convert_to_int_vector(as_str);
        res.push(as_int_vec);
    }
    return res;
}

/// Takes a file's content as input and produces a list
/// of edges that represent such content.
/// Since the file format specifies these edges, the output
/// representation is transparent and does not perform any
/// significant computation.
pub fn to_edges(file_content: Vec<Vec<i64>>) -> types::Graph {
    return file_content[1..]
        .iter()
        .map(|v| types::Edge {
            tail: v[0] as u64,
            head: v[1] as u64,
            weight: v[2],
        })
        .collect::<Vec<_>>();
}

/// Returns the source vertex of an edge.
pub fn edge_head(edge: &types::Edge) -> u64 {
    return edge.head;
}

/// Returns the destination vertex of an edge.
pub fn edge_tail(edge: &types::Edge) -> u64 {
    return edge.tail;
}

/// Computes and returns a table that maps vertices to a list of edges for which
/// they are associated according to a _key getter_ function.
/// Takes as input a collection of `types::Edge`s representing a graph, as well as a
/// function (i.e. `key_getter`) that maps an `types::Edge` to a vertex.
fn to_assoc_edges<'a>(
    edges: &'a [types::Edge],
    key_getter: &dyn Fn(&types::Edge) -> u64,
) -> HashMap<u64, Vec<&'a types::Edge>> {
    return edges.iter().map(|e| (key_getter(e), e)).into_group_map();
}

/// Computes and returns a table that maps vertices to their _indegree_ edges.
/// Takes as input a collection of `types::Edge`s representing a graph.
pub fn to_indeg_edges(edges: &[types::Edge]) -> HashMap<u64, Vec<&types::Edge>> {
    return to_assoc_edges(edges, &edge_head);
}

/// Computes and returns a table that maps vertices to their _outdegree_ edges.
/// Takes as input a list of `types::Edge`s representing a graph.
pub fn to_outdeg_edges(edges: &[types::Edge]) -> HashMap<u64, Vec<&types::Edge>> {
    return to_assoc_edges(edges, &edge_tail);
}

/// Returns the vertices of the input graph `g`.
pub fn vertices(g: &[types::Edge]) -> HashSet<u64> {
    let heads: HashSet<u64> = g.iter().map(edge_head).collect();
    let tails: HashSet<u64> = g.iter().map(edge_tail).collect();
    return heads.union(&tails).cloned().collect();
}
