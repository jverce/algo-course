pub type VertexId = usize;
pub type Weight = f64;

#[derive(Debug)]
pub struct Edge {
    pub tail: VertexId,
    pub head: VertexId,
    pub weight: Weight,
}

pub type Graph = Vec<Edge>;
