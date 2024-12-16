import os
from time import sleep
import time
from typing import Dict, List, Tuple
from collections import deque


def get_neigbors(
    point: Tuple[int, int], r_len: int, c_len: int
) -> List[Tuple[int, int]]:
    r_delta = [0, 0, 1]
    c_delta = [-1, 1, 0]

    neighbors = []

    for i in range(3):
        r = point[0] + r_delta[i]
        c = point[1] + c_delta[i]

        if r >= 0 and r < len(space) and c >= 0 and c < len(space[r]):
            neighbors.append((r, c))

    return neighbors



def print_space(space):
    os.system('cls' if os.name == 'nt' else 'clear')  # 'nt' for windows, 'posix' for Unix-based systems
    for row in space:
        print("".join(row))
    time.sleep(0.5)  # delay for visibility, adjust as needed

def get_shortest_path(space: List[List[str]], start_point: Tuple[int, int], pair: Tuple[int, int], galaxy: Dict[Tuple[int, int], int], rowG, colG):
    visited = {start_point}
    space_points = deque([start_point])
    distance = 0
    start = pair[0]

    while space_points:
        for _ in range(len(space_points)):
            p = space_points.popleft()
            
            if space[p[0]][p[1]] != ".":
                d_delta = 0
                for rI, isGalaxyPresent in enumerate(rowG):
                    if not isGalaxyPresent and (start_point[0] < rI < p[0] or p[0] < rI < start_point[0]):
                        d_delta += 1000000 - 1
                
                for cI, isGalaxyPresent in enumerate(colG):
                    if not isGalaxyPresent and (start_point[1] < cI < p[1] or p[1] < cI < start_point[1]):
                        d_delta += 1000000 - 1
                t = int(space[p[0]][p[1]])
                if t > start:
                    galaxy[(start, t)] = distance + d_delta

            for neighbor in get_neigbors(p, len(space), len(space[0])):
                if neighbor not in visited:
                    space_points.append(neighbor)
                    visited.add(neighbor)
        
        distance += 1


    return -1

if __name__ == "__main__":
    part1Result = 0
    sky = None
    with open("2023/day11/input.txt") as f:
        spaceInput = [list(line.strip()) for line in f.readlines()]

    space = [[point for point in list(spaceInput[i])] for i in range(len(spaceInput))]

    rowG = [False for i in range(len(space))]
    colG = [False for i in range(len(space[0]))]

    for i in range(len(space)):
        for j in range(len(space[i])):
            if space[i][j] == "#":
                rowG[i] = True
                colG[j] = True
    
    galaxyNum = 0
    galaxyCoords = {}
    for i in range(len(space)):
        for j in range(len(space[i])):
            if space[i][j] == "#":
                galaxyNum += 1
                space[i][j] = str(galaxyNum)
                galaxyCoords[galaxyNum] = (i, j)

    galaxyPairs = {}

    for x in range(galaxyNum):
        for y in range(x + 1, galaxyNum):
            galaxyPairs[(x + 1, y + 1)] = 0

    # for pair in list(galaxyPairs.keys()):
    #     if galaxyPairs[pair] == 0:
    #         get_shortest_path(space, galaxyCoords[pair[0]], pair, galaxyPairs, rowG, colG)

    # for _, d in galaxyPairs.items():
    #     part1Result += d

    for pair in list(galaxyPairs.keys()):
        start = galaxyCoords[pair[0]]
        end = galaxyCoords[pair[1]]

        part1Result += abs(start[0] - end[0]) + abs(start[1] - end[1])

        for rI, isGalaxyPresent in enumerate(rowG):
            if not isGalaxyPresent and (start[0] < rI < end[0] or end[0] < rI < start[0]):
                part1Result += 1000000 - 1
                
        for cI, isGalaxyPresent in enumerate(colG):
            if not isGalaxyPresent and (start[1] < cI < end[1] or end[1] < cI < start[1]):
                part1Result += 1000000 - 1

    
    print(part1Result)