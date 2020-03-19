use crate::common::types::VertexId;
use bit_vec::BitVec;
use std::collections::HashSet;

/// Data type to represent the expected output of this week's assignment.
pub type TspResult = i64;

/// Wrapper that represents operations on a set of enumerated items.
pub trait EnumSet<T> {
    fn add(&mut self, x: &T);
    fn remove(&mut self, x: &T);
    fn all(&self) -> bool;
    fn contains(&self, x: &T) -> Option<bool>;
}

/// Data type to use to refer to a subset of vertices.
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
    fn add(&mut self, v: &VertexId) {
        self.vertex_mask.set(*v, true);
    }

    fn remove(&mut self, v: &VertexId) {
        self.vertex_mask.set(*v, false);
    }

    fn all(&self) -> bool {
        return self.vertex_mask.all();
    }

    fn contains(&self, v: &VertexId) -> Option<bool> {
        return self.vertex_mask.get(*v);
    }
}
