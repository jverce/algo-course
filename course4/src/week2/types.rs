use crate::common::types::VertexId;
use std::collections::HashSet;

/// Wrapper that represents operations on a set of enumerated items.
pub trait EnumSet<T> {
    fn new() -> Self;
    fn add(&self, x: &T) -> Self;
    fn remove(&self, x: &T) -> Self;
}

type BitVec = usize;

/// Data type to use to refer to a subset of vertices.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct VertexSubset {
    vertex_mask: BitVec,
}

impl From<&HashSet<VertexId>> for VertexSubset {
    fn from(vs: &HashSet<VertexId>) -> VertexSubset {
        let mut vertex_mask: BitVec = 0;
        for &v in vs {
            vertex_mask |= 1 << v;
        }
        VertexSubset { vertex_mask }
    }
}

impl From<&Vec<&VertexId>> for VertexSubset {
    fn from(vs: &Vec<&VertexId>) -> VertexSubset {
        let mut vertex_mask: BitVec = 0;
        for &v in vs {
            vertex_mask |= 1 << *v;
        }
        VertexSubset { vertex_mask }
    }
}

impl From<&VertexId> for VertexSubset {
    fn from(v: &VertexId) -> VertexSubset {
        let vertex_mask: BitVec = 1 << *v;
        VertexSubset { vertex_mask }
    }
}

impl EnumSet<VertexId> for VertexSubset {
    fn new() -> VertexSubset {
        let vertex_mask = 0;
        VertexSubset { vertex_mask }
    }

    fn add(&self, v: &VertexId) -> VertexSubset {
        let mut vertex_mask = self.vertex_mask;
        vertex_mask |= 1 << *v;
        VertexSubset { vertex_mask }
    }

    fn remove(&self, v: &VertexId) -> VertexSubset {
        let mut vertex_mask = self.vertex_mask;
        vertex_mask |= 1 << *v;
        vertex_mask ^= 1 << *v;
        VertexSubset { vertex_mask }
    }
}
