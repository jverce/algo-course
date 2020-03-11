use std::collections::HashMap;

#[derive(Debug)]
pub struct Edge {
    pub tail: u64,
    pub head: u64,
    pub weight: i64,
}

pub type Graph = Vec<Edge>;

pub type ShortestPaths = HashMap<u64, i64>;
