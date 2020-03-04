#[derive(Debug)]
pub struct Edge {
    pub tail: u64,
    pub head: u64,
    pub weight: i64,
}

pub type Graph = Vec<Edge>;
