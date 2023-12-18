from collections import defaultdict, namedtuple
from heapq import heappush, heappop, heapify
from typing import List, NamedTuple, Self
from enum import Enum

"""
Logic:

Start from top left corner and then use Djikstra to go through the map.
We can't go reverse so the get_neighbours function should know of the direction we are coming from.

We also can't go more than 3 steps in the same direction. So we need to keep count of the number of steps we have taken in the same direction.
"""

Direction = namedtuple('Direction', 'x y')

UP = Direction(-1, 0)
DOWN = Direction(1, 0)
LEFT = Direction(0, -1)
RIGHT = Direction(0, 1)

class Point(NamedTuple):
    row: int
    col: int
    run: int
    direction: Direction

    def get_neighbors(self, nrows: int, ncols: int) -> List[Self]:
        nPoints = []
        excluded_directions = [Direction(-self.direction.x, -self.direction.y)]

        if self.run == 3:
            excluded_directions.append(self.direction)
        
        for direction in [UP, DOWN, LEFT, RIGHT]:
            if direction not in excluded_directions:
                nrow = self.row + direction.x
                ncol = self.col + direction.y
                if 0 <= nrow < nrows and 0 <= ncol < ncols:
                    nPoints.append(self.__class__(nrow, ncol, self.run + 1 if direction == self.direction else 1, direction))

        return nPoints
    
    @property
    def min_run(self):
        return 0


class UltraPoint(Point):
    def get_neighbors(self, nrows: int, ncols: int) -> List[Self]:
        directions = []
        if 1 <= self.run < 4:
            directions = [self.direction]
        else:
            directions = [UP, DOWN, LEFT, RIGHT]

        nPoints = []
        excluded_directions = [Direction(-self.direction.x, -self.direction.y)]

        if self.run == 10:
            excluded_directions.append(self.direction)
        
        for direction in directions:
            if direction not in excluded_directions:
                nrow = self.row + direction.x
                ncol = self.col + direction.y
                if 0 <= nrow < nrows and 0 <= ncol < ncols:
                    nPoints.append(self.__class__(nrow, ncol, self.run + 1 if direction == self.direction else 1, direction))

        return nPoints

    @property
    def min_run(self):
        return 4


def find_shortest_path(map: List[List[str]], start: Point) -> int:
    def bfs(start: Point) -> int:
        nrows, ncols = len(map), len(map[0])
        target = nrows - 1, ncols - 1
        queue = [(0, start)]

        # This is map rather than a grid like normal djikstra
        # because points can be visited in different ways
        # If we follow the normal djikstra approach, it does not guarantee the path to the current point is the shortest
        # Becuase we have a condition that we can't go more than 3 steps in the same direction 
        # So we need to keep track of heat loss to a point from all possible states not just path. States can be number of steps, direction and ofcourse the point itself
        total_heat_loss_map = defaultdict(lambda: float('inf'))

        while queue:
            heat_loss, point= heappop(queue)
            

            if (point.row, point.col) == target and point.run >= point.min_run:
                return heat_loss
            
            for nPoint in point.get_neighbors(nrows, ncols):
                new_heat_loss = heat_loss + int(map[nPoint.row][nPoint.col])

                if new_heat_loss < total_heat_loss_map[nPoint]:
                    total_heat_loss_map[nPoint] = new_heat_loss
                    heappush(queue, (new_heat_loss, nPoint))
    
    result = bfs(start)
    return result

if __name__ == '__main__':
    with open("2023/day17/input.txt", "r") as file:
        map = [list(line.strip()) for line in file.readlines()]

    start = Point(0, 0, 0, RIGHT)

    heat_loss_p1= find_shortest_path(map, start)
    heat_loss_p2 = find_shortest_path(map, UltraPoint(0, 0, 0, RIGHT))
    print(heat_loss_p1, heat_loss_p2)
