use std::collections::HashMap;

pub type VertexId = usize;

#[derive(Debug)]
pub struct Edge {
    pub tail: VertexId,
    pub head: VertexId,
    pub weight: i64,
}

pub type Graph = Vec<Edge>;

pub type ShortestPathsBF = HashMap<VertexId, i64>;
pub type ShortestPathsFW = HashMap<(VertexId, VertexId), i64>;
