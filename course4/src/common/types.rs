use generic_array::{ArrayLength, GenericArray};
use spade::{PointN, SpadeNum, TwoDimensional};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

pub type VertexId = usize;
pub type Weight = f64;
pub type Real = f64;

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

#[derive(Clone, Debug)]
/// Represents a labeled vertex located at point in the space `T^N`.
pub struct PointVertex<T, N>
where
    T: SpadeNum,
    N: ArrayLength<T>,
{
    pub point: Point<T, N>,
    pub id: VertexId,
}

impl<T, N> PartialEq for PointVertex<T, N>
where
    T: SpadeNum,
    N: ArrayLength<T>,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T, N> Eq for PointVertex<T, N>
where
    T: SpadeNum,
    N: ArrayLength<T>,
{
}

impl<T, N> PartialOrd for PointVertex<T, N>
where
    T: SpadeNum,
    N: ArrayLength<T>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl<T, N> Hash for PointVertex<T, N>
where
    T: SpadeNum,
    N: ArrayLength<T>,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl<T, N> PointN for PointVertex<T, N>
where
    T: SpadeNum,
    N: ArrayLength<T> + Copy + Debug + PartialEq + PartialOrd,
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
    N: ArrayLength<T> + Copy + Debug + PartialEq + PartialOrd,
{
}

/// Data type to represent the expected output of this week's assignment.
pub type TspResult = i64;
