use std::collections::HashMap;

pub type VertexId = usize;
pub type Weight = f64;

#[derive(Debug)]
pub struct Edge {
    pub tail: VertexId,
    pub head: VertexId,
    pub weight: Weight,
}

/// Canonical graph representation, as a set of edges.
pub type Graph = Vec<Edge>;

/// Tabular graph representation, that for each pair of
/// vertices it returns the weight of the edge between them.
pub type GraphTab = HashMap<(VertexId, VertexId), Weight>;

/// Represents a point in the space `T^N`.
pub type Point<T> = Vec<T>;

pub struct PointVertex<T> {
    pub point: Point<T>,
    pub id: VertexId,
}
