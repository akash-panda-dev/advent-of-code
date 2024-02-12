from typing import List, Set, Tuple
import networkx as nx
from itertools import combinations
import sys

sys.setrecursionlimit(100000)

def get_nodes(start, graph: nx.Graph) -> Set[str]:
    def dfs(node, visited):
        visited.add(node)

        for neighbor in graph.neighbors(node):
            if neighbor not in visited:
                dfs(neighbor, visited)
    
    visited = set()
    dfs(start, visited)
    return visited

def find_edges_to_disconnect(graph: nx.Graph) -> Tuple[str, str, str]:
    start = list(graph.nodes)[0]
    for edges in combinations(graph.edges, 3):
        G_temp = graph.copy()
        G_temp.remove_edges_from(edges)
        if not len(get_nodes(start, G_temp)) == len(graph.nodes):
            return edges
        graph.add_edges_from(edges)

    return ("", "", "")

if __name__=="__main__":
    graph_spec = {}

    with open("2023/day25/input.txt") as f:
        graph_spec = {k.strip(): v.split() for k, v in (line.strip().split(":") for line in f)}

    graph = nx.Graph()

    for node, edges in graph_spec.items():
        for edge in edges:
            graph.add_edge(node, edge)

    # edges_to_remove = find_edges_to_disconnect_optimized(graph)
    # # graph.remove_edges_from(edges_to_remove)
    # disjoint_graph1 = len(get_nodes(edges_to_remove[0][0], graph))
    # disjoint_graph2 = len(get_nodes(edges_to_remove[0][1], graph))
    # print(f"Disjoint graphs: {disjoint_graph1}, {disjoint_graph2}")

    # Applying Stoer-Wagner algorithm to find minimum cut
    cut_value, partition = nx.stoer_wagner(graph)
    partition_1, partition_2 = partition

    print(f"Cut value: {cut_value}")
    # Displaying the result
    print(len(partition_1) * len(partition_2))

    

    