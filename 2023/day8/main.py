from collections import defaultdict
import re
from collections import deque


ways = {
    0: "L",
    1: "R"
}

# start node is start list
def findZZZ(start_list, directions, graph):
    q = deque(start_list)
    start_len = len(start_list)
    steps = 0
    way_count = 0
    print("Number of directions: ", len(directions))
    
    while q:

        reachedDestination = True
        foundSomeZs = False
        for destination in q:
            if destination[-1] != 'Z':
                reachedDestination = False
            elif destination[-1] == 'Z':
                foundSomeZs = True
        
        if reachedDestination:
            return steps
        
        # if foundSomeZs:
        #     print(q)

        for _ in range(len(q)):
            node = q.popleft()
            
            neighbours = graph[node]
            next_node = neighbours[directions[way_count%len(directions)]]
            q.append(next_node)
        
        steps += 1
        way_count += 1

        if steps % 1000000 == 0:
            print("Steps: ", steps)


    
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

    print("Length of key nodes: ", len(key_nodes))
    print("Last key node: ", key_nodes[-1])
    print("Start list: ", start_list)
    print(findZZZ(start_list, directions, graph))