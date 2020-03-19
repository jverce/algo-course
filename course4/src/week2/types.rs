use crate::common::types::VertexId;
use bit_vec::BitVec;
use std::collections::HashSet;

/// Data type to represent the expected output of this week's assignment.
pub type TspResult = i64;

/// Wrapper that represents operations on a set of enumerated items.
pub trait EnumSet<T> {
    fn add(&self, x: &T) -> Self;
    fn remove(&self, x: &T) -> Self;
    fn diff(&self, other: &Self) -> Self;
    fn set_all(&self) -> Self;
    fn all(&self) -> bool;
    fn contains(&self, x: &T) -> Option<bool>;
}

/// Data type to use to refer to a subset of vertices.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct VertexSubset {
    vertex_mask: BitVec,
}

impl From<&HashSet<VertexId>> for VertexSubset {
    fn from(vs: &HashSet<VertexId>) -> VertexSubset {
        let nbits = vs.len();
        let vertex_mask = BitVec::from_elem(nbits, false);
        return VertexSubset { vertex_mask };
    }
}

impl EnumSet<VertexId> for VertexSubset {
    fn add(&self, v: &VertexId) -> VertexSubset {
        let mut vertex_mask = BitVec::from((*self).vertex_mask.clone());
        vertex_mask.set(*v, true);
        VertexSubset { vertex_mask }
    }

    fn remove(&self, v: &VertexId) -> VertexSubset {
        let mut vertex_mask = BitVec::from((*self).vertex_mask.clone());
        vertex_mask.set(*v, false);
        VertexSubset { vertex_mask }
    }

    fn diff(&self, other: &VertexSubset) -> VertexSubset {
        let mut vertex_mask = BitVec::from((*self).vertex_mask.clone());
        vertex_mask.difference(&(*other).vertex_mask);
        VertexSubset { vertex_mask }
    }

    fn set_all(&self) -> VertexSubset {
        let mut vertex_mask = BitVec::from((*self).vertex_mask.clone());
        vertex_mask.set_all();
        VertexSubset { vertex_mask }
    }

    fn all(&self) -> bool {
        return self.vertex_mask.all();
    }

    fn contains(&self, v: &VertexId) -> Option<bool> {
        return self.vertex_mask.get(*v);
    }
}
