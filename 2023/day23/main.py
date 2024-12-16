from collections import namedtuple, deque
from heapq import heappop, heappush
from typing import List, NamedTuple, Self, Set, Tuple
import time

import sys

sys.setrecursionlimit(100000)


Direction = namedtuple('Direction', 'x y')

UP = Direction(-1, 0)
DOWN = Direction(1, 0)
LEFT = Direction(0, -1)
RIGHT = Direction(0, 1)
NO_DIRECTION = Direction(0, 0)

slope_to_direction = {
    '>': RIGHT,
    '<': LEFT,
    '^': UP,
    'v': DOWN
}

def get_neighbors(row, col, map: List[List[str]]) -> List[Tuple[int, int]]:
    nrows = len(map)
    ncols = len(map[0])
    nTiles = []

    # if map[row][col] in slope_to_direction.keys():
    #     direction = slope_to_direction[map[row][col]]
    #     nrow = row + direction.x
    #     ncol = col + direction.y
        
    #     if 0 <= nrow < nrows and 0 <= ncol < ncols:
    #         if map[nrow][ncol] != '#':
    #             nTiles.append((nrow, ncol))

    # else:
    for direction in [UP, DOWN, LEFT, RIGHT]:
        nrow = row + direction.x
        ncol = col + direction.y
        if 0 <= nrow < nrows and 0 <= ncol < ncols:
            ntile_val = map[nrow][ncol]
            if ntile_val == '#':
                # (ntile_val == '<' and direction == RIGHT) or \
                # (ntile_val == '>' and direction == LEFT) or \
                # (ntile_val == '^' and direction == DOWN) or \
                # (ntile_val == 'v' and direction == UP):
                continue
            
            nTiles.append((nrow, ncol))

    return nTiles

class Tile(NamedTuple):
    row: int
    col: int
    path: Set[Tuple[int, int]]

    def get_neighbors(self, map: List[List[str]]) -> List[Self]:
        nrows = len(map)
        ncols = len(map[0])
        nTiles = []

        # if map[self.row][self.col] in slope_to_direction.keys():
        #     direction = slope_to_direction[map[self.row][self.col]]
        #     nrow = self.row + direction.x
        #     ncol = self.col + direction.y
            
        #     if 0 <= nrow < nrows and 0 <= ncol < ncols and (nrow, ncol) not in npath:
        #         if map[nrow][ncol] != '#':
        #             nTiles.append(self.__class__(nrow, ncol, npath))

        # else:
        for direction in [UP, DOWN, LEFT, RIGHT]:
            nrow = self.row + direction.x
            ncol = self.col + direction.y
            npath = self.path.copy()
            npath.add((self.row, self.col))
            ntile_val = map[nrow][ncol]
            if 0 <= nrow < nrows and 0 <= ncol < ncols and (nrow, ncol) not in npath:
                if ntile_val == '#':
                    # (ntile_val == '<' and direction == RIGHT) or \
                    # (ntile_val == '>' and direction == LEFT) or \
                    # (ntile_val == '^' and direction == DOWN) or \
                    # (ntile_val == 'v' and direction == UP):
                    continue
                
                nTiles.append(self.__class__(nrow, ncol, npath))

        return nTiles
    
def heuristic(a: Tile, b: Tile):
   # Manhattan distance on a square grid
   return abs(a.row - b.row) + abs(a.col - b.col)
    
def get_longest_path(map: List[List[str]]):
    nrows = len(map)
    ncols = len(map[0])
    s = (0,1)
    e = (nrows-1, ncols-2)
    start = Tile(s[0], s[1], set())

    heap = [(0,start)]
    max_path = 0

    while heap:
        _, cur_tile = heappop(heap)

        if (cur_tile.row, cur_tile.col) == e:
            max_path = max(max_path, len(cur_tile.path))
            continue
        
        for neighbor in cur_tile.get_neighbors(map):
            priority = heuristic(neighbor, Tile(e[0], e[1], set()))
            heappush(heap, (-priority, neighbor))  # type: ignore

    return max_path

def get_longest_path_dfs(map: List[List[str]]):
    nrows = len(map)
    ncols = len(map[0])
    s = (0,1)
    e = (nrows-1, ncols-2)
    start = Tile(s[0], s[1], set())

    stack = [start]
    max_path = 0

    while stack:
        cur_tile = stack.pop()

        if (cur_tile.row, cur_tile.col) == e:
            max_path = max(max_path, len(cur_tile.path))
            continue
        
        for neighbor in cur_tile.get_neighbors(map):
            stack.append(neighbor)

    return max_path

def dfs_recursive(map: List[List[str]]):
    end = (len(map) - 1, len(map[0]) - 2)

    def dfs(start: Tuple[int, int], visited: Set[Tuple[int, int]], steps: int) -> List[int]:
        if start == end:
            return [steps]

        if start in visited:
            return []
        
        visited.add(start)
        results = []

        for neighbor in get_neighbors(start[0], start[1], map):
            if neighbor not in visited:
                results.extend(dfs(neighbor, visited, steps + 1))

        visited.remove(start)
        return results

    all_path_distances = dfs((0, 1), set(), 0)
    return max(all_path_distances)

if __name__ == '__main__':
    with open("2023/day23/input.txt", "r") as file:
        map = [list(line.strip()) for line in file.readlines()]

    start_time = time.time()
    longest_path = dfs_recursive(map)
    end_time = time.time()

    print(f"Longest path: {longest_path}")
    print(f"Time taken: {end_time - start_time} seconds")