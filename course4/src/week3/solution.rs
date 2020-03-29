use crate::common::types::{PointVertex, Real};
use crate::common::utils::{dist, read_lines, to_points};
use crate::week2::types::TspResult;
use spade::rtree::RTree;
use typenum::U2;

pub fn solve_for_file(filename: &str) -> TspResult {
    let file_contents: Vec<Vec<Real>> = read_lines(filename);
    let points: Vec<PointVertex<Real, U2>> = to_points(file_contents);
    let home_vertex = points[0].clone();
    let mut rtree = RTree::bulk_load(points);

    let mut result = 0f64;
    let mut prev = home_vertex.clone();
    let mut nn;
    rtree.remove(&prev);
    while rtree.size() != 0 {
        nn = rtree.nearest_neighbor(&prev).unwrap();
        result = result + dist(prev.point, (*nn).point.clone());
        prev = (*nn).clone();
        rtree.remove(&prev);
    }
    result = result + dist(prev.point, home_vertex.point);

    result.floor() as TspResult
}

pub fn solve() {
    let result = solve_for_file("resources/week3/tsp.txt");
    println!("{}", result);
}
