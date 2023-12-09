from collections import defaultdict
import math
import re
from collections import deque


ways = {
    0: "L",
    1: "R"
}

# start node is start list
<<<<<<< Updated upstream
def findZZZ(start, directions, graph):
    q = deque([start])
=======
def findZZZ(start_list, directions, graph):
    q = deque(start_list)
>>>>>>> Stashed changes
    steps = 0
    way_count = 0
    
    while q:
<<<<<<< Updated upstream
        if q[0][-1] == 'Z':
            return steps
        
        node = q.popleft()
=======

        reachedDestination = True
        for destination in q:
            if destination[-1] != 'Z':
                reachedDestination = False
        
        if reachedDestination:
            return steps
        

        for _ in range(len(q)):
            node = q.popleft()
>>>>>>> Stashed changes
            
        neighbours = graph[node]
        next_node = neighbours[directions[way_count%len(directions)]]
        q.append(next_node)
        
        steps += 1
        way_count += 1

<<<<<<< Updated upstream
=======
        if steps % 1000000 == 0:
            print("Steps: ", steps)

>>>>>>> Stashed changes
    return -1


if __name__ == '__main__':
    with open('2023/day8/input.txt', 'r') as file:
        lines = file.read().split('\n')

    directions = [1 if way == 'R' else 0 for way in lines[0]]
    graph = defaultdict(list)
    start_list = []
    key_nodes = []

    for i in range(2, len(lines)):
        nodes = re.findall(r'\b\w+\b', lines[i])
        key_nodes.append(nodes[0])
        graph[nodes[0]].extend(nodes[1:])
        if nodes[0][-1] == 'A':
            start_list.append(nodes[0])

    distances = [findZZZ(start, directions, graph) for start in start_list]
    lcm = math.lcm(*distances)
    print(lcm)