import sys
from typing import List, Tuple

sys.setrecursionlimit(100000)


def findStartingPoint(tiles: List[List[str]]) -> Tuple[int, int]:
    for i in range(len(tiles)):
        for j in range(len(tiles[i])):
            if tiles[i][j] == "S":
                return (i, j)

    return (-1, -1)


EAST = (0, 1)
WEST = (0, -1)
NORTH = (-1, 0)
SOUTH = (1, 0)

pipeToTile = {
    "|": [NORTH, SOUTH],
    "-": [EAST, WEST],
    "L": [NORTH, EAST],
    "J": [NORTH, WEST],
    "7": [SOUTH, WEST],
    "F": [SOUTH, EAST],
    ".": [],
}

def get_neigbors(tile, tiles, parent):
    neighbors = []

    deltas = pipeToTile[tiles[tile[0]][tile[1]]]

    for delta in deltas:
        r = tile[0] + delta[0]
        c = tile[1] + delta[1]
        if (
            r >= 0
            and r < len(tiles)
            and c >= 0
            and c < len(tiles[r])
            and (r, c) != parent
        ):
            neighbors.append((r, c))

    return neighbors


def find_cycle(matrix, start, parent):
    def dfs(tile, path, parent=None):
        if tile == start and path:
            return len(path)  # Return the length of the cycle if we are back at start

        path.append(tile)

        for n in get_neigbors(tile, matrix, parent):
            cycle_length = dfs(n, path, tile)
            if cycle_length:
                return cycle_length

        return 0  # No cycle found

    return dfs(start, [], parent)

def find_cycle_iter(matrix, start, parent):
    stack = [start]
    path = []

    while stack:
        tile = stack.pop()
        if tile == start and path:
            return len(path), path  # Return the length of the cycle if we are back at start
        
        path.append(tile)

        for n in get_neigbors(tile, matrix, parent):
            stack.append(n)
            parent = tile

    return 0, [] # No cycle found


if __name__ == "__main__":
    part1Result = 0
    tiles = None

    with open("2023/day10/input.txt", "r") as file:
        lines = file.read().split("\n")

    tiles = [[tile for tile in list(lines[i])] for i in range(len(lines))]

    r, c = findStartingPoint(tiles)
    tiles[r][c] = "|"

    length, path = find_cycle_iter(tiles, (r, c), (64, 62))

    print("Part 1: ", length // 2)
    

