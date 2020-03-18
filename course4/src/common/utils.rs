use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use crate::common::types::{Edge, Graph, VertexId, Weight};

/// Function that compares `PartialOrd` values and returns
/// an `std::cmp::Ordering` result, so that it can be used in a straightforward
/// manner by `std::cmp::min_by` and such.
pub fn cmp<T: PartialOrd>(a: &T, b: &T) -> Ordering {
    return a.partial_cmp(b).unwrap();
}

/// The `str` content in each lines is split in each space
/// character, and each of these components is parsed and put into a vector.
fn convert_to_num_vector<T: FromStr>(line: &str) -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    return line.split(' ').map(|i| i.parse().unwrap()).collect();
}

/// Reads all the lines in the file located at `filename`,
/// and returns these as a collection, where each item
/// represents each line.
/// The `str` content in each lines is split in each space
/// character, and each of these components is put into a vector.
pub fn read_lines<T: FromStr>(filename: &str) -> Vec<Vec<T>>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let fd = File::open(filename).unwrap();
    let reader = BufReader::new(fd);
    let mut res: Vec<Vec<T>> = Vec::new();
    for i in reader.lines() {
        let as_str: &str = &&i.unwrap();
        let as_int_vec = convert_to_num_vector(as_str);
        res.push(as_int_vec);
    }
    return res;
}

/// Takes a file's content as input and produces a list
/// of edges that represent such content.
/// Since the file format specifies these edges, the output
/// representation is transparent and does not perform any
/// significant computation.
pub fn to_edges(file_content: Vec<Vec<i64>>) -> Graph {
    return file_content[1..]
        .iter()
        .map(|v| Edge {
            tail: v[0] as VertexId,
            head: v[1] as VertexId,
            weight: v[2] as Weight,
        })
        .collect::<Vec<_>>();
}

/// Returns the source vertex of an edge.
pub fn edge_head(edge: &Edge) -> VertexId {
    return edge.head;
}

/// Returns the destination vertex of an edge.
pub fn edge_tail(edge: &Edge) -> VertexId {
    return edge.tail;
}

/// Computes and returns a table that maps vertices to a list of edges for which
/// they are associated according to a _key getter_ function.
/// Takes as input a collection of `Edge`s representing a graph, as well as a
/// function (i.e. `key_getter`) that maps an `Edge` to a vertex.
fn to_assoc_edges<'a>(
    edges: &'a [Edge],
    key_getter: &dyn Fn(&Edge) -> VertexId,
) -> HashMap<VertexId, Vec<&'a Edge>> {
    return edges.iter().map(|e| (key_getter(e), e)).into_group_map();
}

/// Computes and returns a table that maps vertices to their _indegree_ edges.
/// Takes as input a collection of `Edge`s representing a graph.
pub fn to_indeg_edges(edges: &[Edge]) -> HashMap<VertexId, Vec<&Edge>> {
    return to_assoc_edges(edges, &edge_head);
}

/// Computes and returns a table that maps vertices to their _outdegree_ edges.
/// Takes as input a list of `Edge`s representing a graph.
pub fn to_outdeg_edges(edges: &[Edge]) -> HashMap<VertexId, Vec<&Edge>> {
    return to_assoc_edges(edges, &edge_tail);
}

/// Returns the vertices of the input graph `g`.
pub fn vertices(g: &[Edge]) -> HashSet<VertexId> {
    let heads: HashSet<VertexId> = g.iter().map(edge_head).collect();
    let tails: HashSet<VertexId> = g.iter().map(edge_tail).collect();
    return heads.union(&tails).cloned().collect();
}
