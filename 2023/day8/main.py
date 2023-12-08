from collections import defaultdict
import re
from collections import deque


ways = {
    0: "LEFT",
    1: "RIGHT"
}

def findZZZ(start, directions, graph):
    q = deque([start])
    steps = 0
    way_count = 0
    print("Number of directions: ", len(directions))
    
    while q:
        node = q.popleft()
        #print("node: ", node)

        if node == 'ZZZ':
            return steps
        
        neighbours = graph[node]

        # way = ways[directions[way_count%len(directions)]]
        # print("wayCount: ", way_count%len(directions))
        next_node = neighbours[directions[way_count%len(directions)]]
        if next_node == 'ZZZ':
            return steps
        # print(f"{node} to {next_node}")
        #print(f"next_node: {next_node} from {node} going {way}")
        q.append(next_node)
        steps += 1
        way_count += 1

        if steps % 100000 == 0:
            print(steps)
    
    return -1


if __name__ == '__main__':
    with open('2023/day8/input.txt', 'r') as file:
        lines = file.read().split('\n')

    directions = [1 if way == 'R' else 0 for way in lines[0]]
    graph = defaultdict(list)
    start_node = None
    key_nodes = []

    for i in range(2, len(lines)):
        nodes = re.findall(r'\b\w+\b', lines[i])
        key_nodes.append(nodes[0])
        graph[nodes[0]].extend(nodes[1:])
        if i == 2:
            start_node = nodes[0]

    print("Length of key nodes: ", len(key_nodes))
    print("Last key node: ", key_nodes[-1])
    print(findZZZ(start_node, directions, graph))