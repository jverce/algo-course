use itertools::Itertools;
use std::collections::HashMap;

use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::week1::types;

fn split_line(line: &str) -> Vec<&str> {
    return line.split(' ').collect();
}

fn convert_to_int(line: &str) -> Vec<i64> {
    let mut res: Vec<i64> = Vec::new();
    let split: Vec<&str> = split_line(line);
    for i in split.iter() {
        let as_int: i64 = i.parse::<i64>().unwrap();
        res.push(as_int);
    }
    return res;
}

pub fn read_lines(filename: &str) -> Vec<Vec<i64>> {
    let fd = File::open(filename).unwrap();
    let reader = BufReader::new(fd);
    let mut res: Vec<Vec<i64>> = Vec::new();
    for i in reader.lines() {
        let as_str: &str = &&i.unwrap();
        let as_int_vec = convert_to_int(as_str);
        res.push(as_int_vec);
    }
    return res;
}

pub fn to_edges(file_content: Vec<Vec<i64>>) -> Vec<types::Edge> {
    return file_content[1..]
        .iter()
        .map(|v| types::Edge {
            tail: v[0] as u64,
            head: v[1] as u64,
            weight: v[2],
        })
        .collect::<Vec<_>>();
}

pub fn edge_head(edge: &types::Edge) -> u64 {
    return edge.head;
}

pub fn edge_tail(edge: &types::Edge) -> u64 {
    return edge.tail;
}

/// Computes and returns a table that maps vertices to a list of edges for which
/// they are associated according to a _key getter_ function.
/// Takes as input a list of `types::Edge`s representing a graph, as well as a
/// function (i.e. `key_getter`) that maps an `types::Edge` to a vertex.
fn to_assoc_edges<'a>(
    edges: &'a Vec<types::Edge>,
    key_getter: &dyn Fn(&types::Edge) -> u64,
) -> HashMap<u64, Vec<&'a types::Edge>> {
    return edges.iter().map(|e| (key_getter(e), e)).into_group_map();
}

/// Computes and returns a table that maps vertices to their _indegree_ edges.
/// Takes as input a list of `types::Edge`s representing a graph.
pub fn to_indeg_edges(edges: &Vec<types::Edge>) -> HashMap<u64, Vec<&types::Edge>> {
    return to_assoc_edges(edges, &edge_tail);
}

/// Computes and returns a table that maps vertices to their _outdegree_ edges.
/// Takes as input a list of `types::Edge`s representing a graph.
pub fn to_outdeg_edges(edges: &Vec<types::Edge>) -> HashMap<u64, Vec<&types::Edge>> {
    return to_assoc_edges(edges, &edge_head);
}
