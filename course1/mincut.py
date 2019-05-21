import csv
import math
import random

def gen_graph(filename):
    graph = {}
    with open(filename) as f:
        reader = csv.reader(f, delimiter='\t')
        for row in reader:
            items = [i for i in row if i != '']
            vertex = items[0]
            connections = items[1:]
            graph[vertex] = connections
    return graph

def random_contraction(graph):
    # choose random vertices
    vertex_a = random.choice(list(graph.keys()))
    vertex_b = random.choice(graph[vertex_a])
    new_vertex = ','.join([vertex_a, vertex_b])

    # remove connection between joined vertices
    edges_a = [e for e in graph[vertex_a] if e != vertex_b]
    edges_b = [e for e in graph[vertex_b] if e != vertex_a]
    new_edges = edges_a + edges_b

    # update graph with new vertex
    graph[new_vertex] = new_edges
    del graph[vertex_a]
    del graph[vertex_b]
    for other_vertex in new_edges:
        other_edges = [
            new_vertex if e in {vertex_a, vertex_b} else e
            for e in graph[other_vertex]
        ]
        graph[other_vertex] = other_edges

def karger(graph):
    while len(graph) > 2:
        random_contraction(graph)

def find_min_cut(graph):
    n = len(graph)
    n_iters = n# * n * int(math.log(n) / math.log(2))
    cuts_sizes = []
    for _ in range(n_iters):
        copy = dict(graph)
        karger(copy)
        cuts = list(copy.values())
        cuts_size = len(cuts[0])
        cuts_sizes.append(cuts_size)
    return min(cuts_sizes)
