use std::{
    collections::HashSet,
    fmt::{write, Display},
};

// Memory will have the bounds and the corrupted cells
//

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Coord {
    row: isize,
    col: isize,
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

#[derive(Debug, Clone)]
struct Memory {
    corrupted_cells: HashSet<Coord>,
    row_max: isize,
    col_max: isize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct SearchParty(Coord);

impl SearchParty {
    const DIRECTIONS: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Right,
        Direction::Left,
    ];

    fn successors(&self, memory: &Memory) -> Vec<SearchParty> {
        Self::DIRECTIONS
            .iter()
            .filter_map(|&dir| {
                if memory.is_visitable(self.0.next_pos(dir)) {
                    Some(SearchParty(self.0.next_pos(dir)))
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut memory: Vec<Vec<char>> =
            vec![vec!['.'; self.col_max as usize]; self.row_max as usize];

        for cell in self.corrupted_cells.iter() {
            memory[cell.row as usize][cell.col as usize] = '#';
        }

        writeln!(f, "┌{}┐", "─".repeat(self.col_max as usize))?;
        for row in memory {
            write!(f, "|")?;
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "└{}┘", "─".repeat(self.col_max as usize))
    }
}

impl Memory {
    fn is_outside(&self, coord: Coord) -> bool {
        coord.row > self.row_max - 1
            || coord.row < 0
            || coord.col > self.col_max - 1
            || coord.col < 0
    }

    fn is_visitable(&self, coord: Coord) -> bool {
        !self.is_outside(coord) && !self.corrupted_cells.contains(&coord)
    }
}

fn parse(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|line| {
            let (col, row) = line.split_once(',').unwrap();

            Coord {
                row: row.parse().unwrap(),
                col: col.parse().unwrap(),
            }
        })
        .collect()
}

pub mod part1 {
    use super::*;
    use miette::Result;
    use pathfinding::prelude::bfs;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<u64> {
        let row_max = 71;
        let col_max = 71;
        let corrupted_cells_full = parse(&input).into_iter();
        let corrupted_cells_init: HashSet<Coord> = corrupted_cells_full.take(1024).collect();
        let goal = Coord {
            row: row_max - 1,
            col: col_max - 1,
        };
        let memory = Memory {
            row_max,
            col_max,
            corrupted_cells: corrupted_cells_init.clone(),
        };
        let search_party = SearchParty(Coord { row: 0, col: 0 });

        let shortest_path = bfs(&search_party, |s| s.successors(&memory), |p| p.0 == goal);

        Ok(shortest_path.unwrap().len() as u64 - 1)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        //         #[test]
        //         fn test_process() -> Result<()> {
        //             let input = "\
        // 5,4
        // 4,2
        // 4,5
        // 3,0
        // 2,1
        // 6,3
        // 2,4
        // 1,5
        // 0,6
        // 3,3
        // 2,6
        // 5,1
        // 1,2
        // 5,5
        // 2,5
        // 6,5
        // 1,4
        // 0,4
        // 6,4
        // 1,1
        // 6,1
        // 1,0
        // 0,5
        // 1,6
        // 2,0";
        //             assert_eq!(45, process(input)?);
        //             Ok(())
        //         }
    }
}

pub mod part2 {
    use super::*;
    use miette::Result;
    use pathfinding::prelude::bfs;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<String> {
        let row_max = 71;
        let col_max = 71;
        let corrupted_cells_full = parse(&input);
        let corrupted_cells_init: HashSet<Coord> = corrupted_cells_full
            .clone()
            .into_iter()
            .take(1024)
            .collect();
        let goal = Coord {
            row: row_max - 1,
            col: col_max - 1,
        };
        let mut memory = Memory {
            row_max,
            col_max,
            corrupted_cells: corrupted_cells_init.clone(),
        };
        let start = SearchParty(Coord { row: 0, col: 0 });
        let mut search_cells = corrupted_cells_full.into_iter().skip(12);

        let mut result_cell = None;

        while let Some(next_cell) = search_cells.next() {
            memory.corrupted_cells.insert(next_cell);
            if let None = bfs(&start, |s| s.successors(&memory), |p| p.0 == goal) {
                result_cell = Some(next_cell);
                break;
            }
        }
        let result = result_cell.unwrap();

        Ok(format!("{},{}", result.col, result.row))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        //         #[test]
        //         fn test_process() -> Result<()> {
        //             let input = "\
        // 5,4
        // 4,2
        // 4,5
        // 3,0
        // 2,1
        // 6,3
        // 2,4
        // 1,5
        // 0,6
        // 3,3
        // 2,6
        // 5,1
        // 1,2
        // 5,5
        // 2,5
        // 6,5
        // 1,4
        // 0,4
        // 6,4
        // 1,1
        // 6,1
        // 1,0
        // 0,5
        // 1,6
        // 2,0";
        //             assert_eq!("6,1", process(input)?);
        //             Ok(())
        //         }
    }
}
