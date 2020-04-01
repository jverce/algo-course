use crate::common::types::{PointVertex, Real, TspResult};
use crate::common::utils::{dist, read_lines, to_points};
use spade::rtree::RTree;
use typenum::U2;

pub fn solve_for_file(filename: &str) -> TspResult {
    let file_contents: Vec<Vec<Real>> = read_lines(filename);
    let points: Vec<PointVertex<Real, U2>> = to_points(file_contents);
    let home_vertex = points[0].clone(); // The first point should be the home vertex.
    let mut rtree = RTree::bulk_load(points);

    // Iterate over all the vertices in the graph, starting from the first vertex.
    let mut result = 0f64;
    let mut prev = home_vertex.clone();
    rtree.remove(&prev);

    // Continue computing the distance while there are still vertices to visit.
    while rtree.size() != 0 {
        // Get the nearest neighbors to the last vertex we visited.
        let mut nns = rtree.nearest_neighbors(&prev);
        // Sort by vertex ID and get the head of the sorted list.
        nns.sort();
        let nn = nns.iter().nth(0).unwrap();

        // Compute the Euclidean distance from the last vertex we visited to the next one.
        result += dist(prev.point, nn.point);

        // Mark the next vertex as the current/last one, and remove it from the set of
        // vertices to visit.
        prev = (*nn).clone();
        rtree.remove(&prev);
    }
    // Lastly, compute the distance from the last visited vertex back to the starting point.
    result = result + dist(prev.point, home_vertex.point);

    // Round down and cast the result to an integer.
    result.floor() as TspResult
}

pub fn solve() {
    let result = solve_for_file("resources/week3/tsp.txt");
    println!("{}", result);
}
