use generic_array::{ArrayLength, GenericArray};
use spade::{PointN, SpadeNum, TwoDimensional};
use std::collections::HashMap;
use std::fmt::Debug;

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
pub type Point<T, N> = GenericArray<T, N>;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
/// Represents a labeled vertex located at point in the space `T^N`.
pub struct PointVertex<T, N>
where
    N: ArrayLength<T>,
{
    pub point: Point<T, N>,
    pub id: VertexId,
}

impl<T, N> PointN for PointVertex<T, N>
where
    T: SpadeNum,
    N: ArrayLength<T> + Clone + Debug + PartialEq + PartialOrd,
{
    type Scalar = T;

    fn dimensions() -> usize {
        N::USIZE
    }

    fn from_value(value: Self::Scalar) -> Self {
        let id = 0;
        let point = (0..N::USIZE).map(|_| value.clone()).collect();
        PointVertex { id, point }
    }

    fn nth(&self, index: usize) -> &Self::Scalar {
        self.point.iter().nth(index).unwrap()
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        self.point.iter_mut().nth(index).unwrap()
    }
}

impl<T, N> TwoDimensional for PointVertex<T, N>
where
    T: SpadeNum,
    N: ArrayLength<T> + Clone + Debug + PartialEq + PartialOrd,
{
}
