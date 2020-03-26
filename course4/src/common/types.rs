use kdtree::kdtree::KdtreePointTrait;
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

/// Represents a point in the space `T^2`.
#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct PointVertex<T> {
    pub id: VertexId,
    pub point: [T; 2],
}

pub type Real = f64;

impl KdtreePointTrait for PointVertex<Real> {
    #[inline]
    fn dims(&self) -> &[Real] {
        &self.point
    }
}
