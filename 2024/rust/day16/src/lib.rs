use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    usize::MAX,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn next_pos(&self, dir: Direction) -> Coord {
        match dir {
            Direction::Up => Coord {
                row: self.row - 1,
                col: self.col,
            },
            Direction::Right => Coord {
                row: self.row,
                col: self.col + 1,
            },
            Direction::Down => Coord {
                row: self.row + 1,
                col: self.col,
            },
            Direction::Left => Coord {
                row: self.row,
                col: self.col - 1,
            },
        }
    }
}

#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn turn_clockwise(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
        }
    }

    fn turn_counter_clockwise(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Left => Direction::Down,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Reindeer {
    loc: Coord,
    dir: Direction,
}

impl Reindeer {
    fn successors(&self, maze: &Maze) -> Vec<(Self, usize)> {
        let mut next_states: Vec<(Self, usize)> = vec![];
        let dir = self.dir;

        if !maze.walls.contains(&self.loc.next_pos(dir)) {
            next_states.push((
                Reindeer {
                    loc: self.loc.next_pos(dir),
                    dir,
                },
                1,
            ));
        }

        next_states.push((
            Reindeer {
                loc: self.loc,
                dir: dir.turn_clockwise(),
            },
            1000,
        ));

        next_states.push((
            Reindeer {
                loc: self.loc,
                dir: dir.turn_counter_clockwise(),
            },
            1000,
        ));

        next_states
    }
}

#[derive(Debug, Clone)]
struct Maze {
    walls: HashSet<Coord>,
    start: Coord,
    end: Coord,
}

impl Maze {
    fn parse(input: &str) -> Self {
        let mut walls = HashSet::new();
        let mut start: Option<Coord> = None;
        let mut end: Option<Coord> = None;

        for (row, line) in input.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let curr_coord = Coord { row, col };
                match ch {
                    '#' => {
                        walls.insert(curr_coord);
                    }
                    'S' => start = Some(curr_coord),
                    'E' => end = Some(curr_coord),
                    _ => {}
                }
            }
        }

        Maze {
            walls,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }
}

pub mod part1 {
    use super::*;
    use miette::Result;
    use pathfinding::{directed::dijkstra, prelude::dijkstra};

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<usize> {
        // It's a straighforward Djisktra algo I think
        // The nodes the variations of cell's location and it's direction.
        // The cost of moving to another cell is 1 and turning is 1000
        //
        // let min_cost = dijkstra()
        //
        let maze = Maze::parse(&input);
        let reindeer = Reindeer {
            loc: maze.start,
            dir: Direction::Right,
        };

        let result = dijkstra(
            &reindeer,
            |reindeer| reindeer.successors(&maze),
            |reindeer| reindeer.loc == maze.end,
        );

        Ok(result.unwrap().1)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_process() -> Result<()> {
            let input = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
            assert_eq!(7036, process(input)?);
            Ok(())
        }

        #[test]
        fn test_process2() -> Result<()> {
            let input = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

            assert_eq!(11048, process(input)?);
            Ok(())
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    reindeer: Reindeer,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn djikstra(
    start: Reindeer,
    end: Coord,
    maze: &Maze,
) -> Option<(HashMap<Coord, HashSet<Coord>>, usize)> {
    let mut predecessors: HashMap<Coord, HashSet<Coord>> = HashMap::new();
    let mut cost_so_far: HashMap<Reindeer, usize> = HashMap::new();
    let mut pq = BinaryHeap::new();
    let start_state = State {
        reindeer: start,
        cost: 0,
    };
    pq.push(start_state);
    cost_so_far.insert(start, 0);

    while let Some(State { cost, reindeer }) = pq.pop() {
        if reindeer.loc == end {
            return Some((predecessors, cost));
        }

        if cost > *cost_so_far.get(&reindeer).unwrap() {
            continue;
        }

        for (next_reindeer, move_cost) in reindeer.successors(maze) {
            let new_cost = cost + move_cost;

            match cost_so_far.get(&next_reindeer) {
                Some(&current_cost) => {
                    if new_cost < current_cost {
                        cost_so_far.insert(next_reindeer, new_cost);
                        if reindeer.loc != next_reindeer.loc {
                            predecessors
                                .entry(next_reindeer.loc)
                                .or_insert_with(HashSet::new)
                                .clear();
                            predecessors
                                .entry(next_reindeer.loc)
                                .or_insert_with(HashSet::new)
                                .insert(reindeer.loc);
                        }
                        pq.push(State {
                            reindeer: next_reindeer,
                            cost: new_cost,
                        });
                    } else if new_cost == current_cost {
                        if reindeer.loc != next_reindeer.loc {
                            predecessors
                                .entry(next_reindeer.loc)
                                .or_insert_with(HashSet::new)
                                .insert(reindeer.loc);
                        }
                    }
                }
                None => {
                    cost_so_far.insert(next_reindeer, new_cost);
                    if reindeer.loc != next_reindeer.loc {
                        predecessors
                            .entry(next_reindeer.loc)
                            .or_insert_with(HashSet::new)
                            .insert(reindeer.loc);
                    }
                    pq.push(State {
                        reindeer: next_reindeer,
                        cost: new_cost,
                    });
                }
            }
        }
    }

    None
}

fn bfs_track(predecessors: &HashMap<Coord, HashSet<Coord>>, start: Coord, end: Coord) -> usize {
    let mut queue = VecDeque::new();
    let mut cells = HashSet::new();

    queue.push_back(end);
    cells.insert(end);

    while let Some(curr) = queue.pop_front() {
        if let Some(preds) = predecessors.get(&curr) {
            for &pred in preds {
                if cells.insert(pred) {
                    queue.push_back(pred);
                }
            }
        }
    }

    cells.len()
}

pub mod part2 {
    use std::collections::HashMap;

    use super::*;
    use itertools::Itertools;
    use miette::Result;
    use pathfinding::prelude::*;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<usize> {
        let maze = Maze::parse(&input);
        let reindeer = Reindeer {
            loc: maze.start,
            dir: Direction::Right,
        };

        let maze = Maze::parse(&input);
        let reindeer = Reindeer {
            loc: maze.start,
            dir: Direction::Right,
        };

        // let result = dijkstra(
        //     &reindeer,
        //     |reindeer| reindeer.successors(&maze),
        //     |reindeer| reindeer.loc == maze.end,
        // );
        let predecessors = djikstra(reindeer, maze.end, &maze);
        dbg!(&predecessors);

        // let path = predecessors
        //     .unwrap()
        //     .0
        //     .values()
        //     .flat_map(|c| c.iter().copied().collect::<Vec<Coord>>())
        //     .dedup()
        //     .count();

        let path_cells = bfs_track(&predecessors.unwrap().0, maze.start, maze.end);

        Ok(path_cells)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_process() -> Result<()> {
            let input = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
            assert_eq!(45, process(input)?);
            Ok(())
        }

        #[test]
        fn test_process2() -> Result<()> {
            let input = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

            assert_eq!(64, process(input)?);
            Ok(())
        }
    }
}
