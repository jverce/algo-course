use std::collections::HashMap;

pub type VertexId = usize;

#[derive(Debug)]
pub struct Edge {
    pub tail: VertexId,
    pub head: VertexId,
    pub weight: Weight,
}

pub type Weight = f64;

pub type Graph = Vec<Edge>;

pub type ShortestPathsBF = HashMap<VertexId, Weight>;
pub type ShortestPathsFW = HashMap<(VertexId, VertexId), Weight>;
