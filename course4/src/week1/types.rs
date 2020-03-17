use crate::common::types::{VertexId, Weight};
use std::collections::HashMap;

pub type ShortestPathsBF = HashMap<VertexId, Weight>;
pub type ShortestPathsFW = HashMap<(VertexId, VertexId), Weight>;
