use itertools::Itertools;
use std::{collections::HashMap, println};

use crate::common::utils;
use crate::week1::bellman_ford;
use crate::week1::types;

fn to_edges(file_content: Vec<Vec<i64>>) -> Vec<types::Edge> {
    return file_content[1..]
        .iter()
        .map(|v| types::Edge {
            tail: v[0] as u64,
            head: v[1] as u64,
            weight: v[2],
        })
        .collect::<Vec<_>>();
}

fn edge_head(edge: &types::Edge) -> u64 {
    return edge.head;
}

fn edge_tail(edge: &types::Edge) -> u64 {
    return edge.tail;
}

fn to_assoc_edges<'a>(
    edges: &'a Vec<types::Edge>,
    key_getter: &dyn Fn(&types::Edge) -> u64,
) -> HashMap<u64, Vec<&'a types::Edge>> {
    return edges.iter().map(|e| (key_getter(e), e)).into_group_map();
}

fn to_indeg_edges(edges: &Vec<types::Edge>) -> HashMap<u64, Vec<&types::Edge>> {
    return to_assoc_edges(edges, &edge_tail);
}

fn solve(filename: &str) {
    let file_content = utils::read_lines(filename);
    let edges = to_edges(file_content);
    let indeg_edges = to_indeg_edges(&edges);
    match indeg_edges.get(&1) {
        Some(v) => println!("{:#?}", v),
        None => println!("nothing"),
    }
}

pub fn graph1() {
    const FILENAME: &str = "resources/week1/g1.txt";
    solve(FILENAME);
}

pub fn graph2() {
    const FILENAME: &str = "resources/week1/g2.txt";
    solve(FILENAME);
}

pub fn graph3() {
    const FILENAME: &str = "resources/week1/g3.txt";
    solve(FILENAME);
}
