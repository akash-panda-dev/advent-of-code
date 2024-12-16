use std::collections::HashSet;

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

pub mod part2 {
    use std::collections::HashMap;

    use super::*;
    use miette::Result;
    use pathfinding::prelude::*;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<usize> {
        let maze = Maze::parse(&input);
        let reindeer = Reindeer {
            loc: maze.start,
            dir: Direction::Right,
        };

        let mut predecessors: HashMap<Reindeer, Vec<Reindeer>> = HashMap::new();

        let result = dijkstra(
            &reindeer,
            |reindeer| {
                let successors = reindeer.successors(&maze);

                for (next, _) in &successors {
                    predecessors.entry(*next).or_default().push(*reindeer);
                }

                successors
            },
            |reindeer| reindeer.loc == maze.end,
        );

        dbg!(predecessors.len());

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
