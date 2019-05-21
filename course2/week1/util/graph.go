package util

import (
	"fmt"
	"math"
)

type Edge struct {
	Dst    Vertex
	Weight float32

	// Meaning of this variable:
	// - 1: direction is u -> v
	// - -1: direction is u <- v
	// - 0: u <-> v (i.e. undirected)
	Dir int8
}

func (e *Edge) String() string {
	return fmt.Sprintf("--%f-->%s", e.Weight, e.Dst)
}

type Vertex struct {
	Data string
}

func (v Vertex) String() string {
	return fmt.Sprintf("((%s))", v.Data)
}

// Graph a set of vertices and edges
type Graph struct {
	Edges map[Vertex][]*Edge
}

func (g *Graph) GetVertices() []Vertex {
	vertices := map[Vertex]bool{}
	for v, edges := range g.Edges {
		vertices[v] = true
		for _, e := range edges {
			vertices[e.Dst] = true
		}
	}

	var result []Vertex
	for v := range vertices {
		result = append(result, v)
	}
	return result
}

func (g *Graph) Size() int {
	return len(g.Edges)
}

func (g *Graph) AddEdge(src string, dst string, w float32) {
	u := Vertex{
		Data: src,
	}
	v := Vertex{
		Data: dst,
	}
	e := Edge{
		Dst:    v,
		Weight: w,
		Dir:    1,
	}
	edges := g.Edges[u]
	if edges != nil {
		edges = append(edges, &e)
	} else {
		edges = []*Edge{&e}
	}
	g.Edges[u] = edges
}

func (g *Graph) Dfs() []Vertex {
	results := []Vertex{}
	visited := map[Vertex]bool{}

	for v := range g.Edges {
		if !visited[v] {
			visited[v] = true
			results = append(results, dfsRec(g, v, &visited)...)
		}
	}
	return results
}

func dfsRec(g *Graph, v Vertex, visited *map[Vertex]bool) []Vertex {
	var results []Vertex
	for _, nextHop := range g.Edges[v] {
		nextHopVertex := nextHop.Dst
		isVisited := (*visited)[nextHopVertex]
		if !isVisited {
			(*visited)[nextHopVertex] = true
			results = append(results, dfsRec(g, nextHopVertex, visited)...)
		}
	}
	results = append(results, v)
	return results
}

func (g *Graph) Scc(vertices []Vertex) [][]Vertex {
	sccs := [][]Vertex{}
	visited := map[Vertex]bool{}

	for _, v := range vertices {
		if !visited[v] {
			visited[v] = true
			vs := dfsRec(g, v, &visited)
			sccs = append(sccs, vs)
		}
	}
	return sccs
}

func (g *Graph) Dijkstra(src Vertex) map[Vertex]float32 {
	distances := map[Vertex]float32{}
	computed := map[Vertex]bool{}
	vs := g.GetVertices()
	for _, v := range vs {
		if v != src {
			distances[v] = float32(1000000)
		}
	}
	distances[src] = float32(0)
	computed[src] = true

	// Main loop: iterate until the list of computed distances
	// matches the size of the graph (i.e. all the vertices in the
	// graph are covered)
	for len(computed) < len(vs) {
		// Compute the list of edges to analyze for this iteration
		metrics := map[Vertex]float32{}
		for v := range computed {
			for _, edge := range g.Edges[v] {
				// Only consider edges which destination
				// lies outside of the computed ones.
				if !computed[edge.Dst] {
					// Compute Dijkstra's greedy distance
					newMetric := distances[v] + edge.Weight
					if metric, ok := metrics[edge.Dst]; ok {
						metrics[edge.Dst] = float32(math.Min(float64(metric), float64(newMetric)))
					} else {
						metrics[edge.Dst] = newMetric
					}
				}
			}
		}

		var minDistance float32 = 1000000
		var nextVertex Vertex
		for v, d := range metrics {
			if d < minDistance {
				minDistance = d
				nextVertex = v
			}
		}

		distances[nextVertex] = minDistance
		computed[nextVertex] = true
	}

	return distances
}
