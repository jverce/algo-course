use spade::{PointN, SpadeNum, TwoDimensional};
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

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointVertex<T> {
    pub point: Point<T>,
    pub id: VertexId,
}

impl<T> PointN for PointVertex<T>
where
    T: SpadeNum,
{
    type Scalar = T;

    fn dimensions() -> usize {
        // I know Rust will someday have non-type template parameters.
        // In the mean-time, I'm hard-coding this.
        2
    }

    fn from_value(value: Self::Scalar) -> Self {
        // I know Rust will someday have non-type template parameters.
        // In the mean-time, I have to build a fixed-size vector like this :(
        let point = vec![value.clone(), value.clone()];
        let id = 0;
        PointVertex { id, point }
    }

    fn nth(&self, index: usize) -> &Self::Scalar {
        self.point.iter().nth(index).unwrap()
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        self.point.iter_mut().nth(index).unwrap()
    }
}

impl<T> TwoDimensional for PointVertex<T> where T: SpadeNum {}
