pub type VertexId = usize;
pub type Weight = f64;

#[derive(Debug)]
pub struct Edge {
    pub tail: VertexId,
    pub head: VertexId,
    pub weight: Weight,
}

pub type Graph = Vec<Edge>;

/// Represents a point in the space `T^2`.
pub struct Point<T> {
    pub x: T,
    pub y: T,
}
