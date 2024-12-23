use std::collections::{HashMap, HashSet, VecDeque};

// Target for robot 1: 029A
// Graph for robot 1 (Numeric keypad)
//
// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A | -> Start
//     +---+---+
//
// Movements for robot 1: (up, down, left, right, A)
//
// Possible shotest movements: <A^A>^^AvvvA, <A^A^>^AvvvA, and <A^A^^>AvvvA
//
// ---------------------
// Target for robot 2: <A^A>^^AvvvA
// Graph for robot 2 (Directional keypad)
//
//     +---+---+
//     | ^ | A | -> Start
// +---+---+---+
// | < | v | > |
// +---+---+---+
//
// Movements for robot 2: (up, down, left, right, A)
// Possible shortest movement: v<<A>>^A<A>AvA<^AA>A<vAAA>^A
//
// ---------------------
// Target for human: v<<A>>^A<A>AvA<^AA>A<vAAA>^A
// Graph for human (Directional keypad)
//
//     +---+---+
//     | ^ | A | -> Start
// +---+---+---+
// | < | v | > |
// +---+---+---+
//
// Movements for human: (up, down, left, right, A)
// Possible shortest movement: <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
//
// Goal is to find the human's possible movement.

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn next_pos(&self, dir: Direction) -> Option<Coord> {
        match dir {
            Direction::Up => self
                .row
                .checked_sub(1)
                .map(|row| Coord { row, col: self.col }),
            Direction::Right => Some(Coord {
                row: self.row,
                col: self.col + 1,
            }),
            Direction::Down => Some(Coord {
                row: self.row + 1,
                col: self.col,
            }),
            Direction::Left => self
                .col
                .checked_sub(1)
                .map(|col| Coord { row: self.row, col }),
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
    fn to_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

trait Pad {
    fn is_visitable(&self, coord: Coord) -> bool;
    fn get_val(&self, coord: Coord) -> char;
}

#[derive(Debug, Clone)]
struct NumKeyPad {
    nums_pad: HashMap<Coord, char>,
}

impl NumKeyPad {
    fn new() -> Self {
        let mut nums_pad = HashMap::new();

        // Add numbers 7-9 (top row)
        nums_pad.insert(Coord { row: 0, col: 0 }, '7');
        nums_pad.insert(Coord { row: 0, col: 1 }, '8');
        nums_pad.insert(Coord { row: 0, col: 2 }, '9');

        // Add numbers 4-6 (middle row)
        nums_pad.insert(Coord { row: 1, col: 0 }, '4');
        nums_pad.insert(Coord { row: 1, col: 1 }, '5');
        nums_pad.insert(Coord { row: 1, col: 2 }, '6');

        // Add numbers 1-3 (bottom row)
        nums_pad.insert(Coord { row: 2, col: 0 }, '1');
        nums_pad.insert(Coord { row: 2, col: 1 }, '2');
        nums_pad.insert(Coord { row: 2, col: 2 }, '3');

        // Add 0 and A (lowest row)
        nums_pad.insert(Coord { row: 3, col: 1 }, '0');
        nums_pad.insert(Coord { row: 3, col: 2 }, 'A');

        Self { nums_pad }
    }
}

impl Pad for NumKeyPad {
    fn is_visitable(&self, coord: Coord) -> bool {
        self.nums_pad.contains_key(&coord)
    }

    fn get_val(&self, coord: Coord) -> char {
        *self.nums_pad.get(&coord).unwrap()
    }
}

#[derive(Debug, Clone)]
struct DirKeyPad {
    dirs_pad: HashMap<Coord, char>, // renamed from nums_pad to dirs_pad for clarity
}

impl DirKeyPad {
    fn new() -> Self {
        let mut dirs_pad = HashMap::new();
        // Add top row (^ and A)
        dirs_pad.insert(Coord { row: 0, col: 1 }, '^');
        dirs_pad.insert(Coord { row: 0, col: 2 }, 'A');

        // Add bottom row (<, v, >)
        dirs_pad.insert(Coord { row: 1, col: 0 }, '<');
        dirs_pad.insert(Coord { row: 1, col: 1 }, 'v');
        dirs_pad.insert(Coord { row: 1, col: 2 }, '>');

        Self { dirs_pad }
    }
}

impl Pad for DirKeyPad {
    fn is_visitable(&self, coord: Coord) -> bool {
        self.dirs_pad.contains_key(&coord)
    }

    fn get_val(&self, coord: Coord) -> char {
        *self.dirs_pad.get(&coord).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Robot(Coord);

impl Robot {
    const DIRECTIONS: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Right,
        Direction::Left,
    ];

    fn successors(&self, pad: &impl Pad) -> Vec<(Robot, Direction)> {
        Self::DIRECTIONS
            .iter()
            .filter_map(|&dir| {
                if let Some(coord) = self.0.next_pos(dir) {
                    if pad.is_visitable(coord) {
                        Some((Robot(coord), dir))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Path {
    robot: Robot,
    moves: String,
}

// TODO: should I return all possible paths
fn moves_to_target(start: Robot, target: char, pad: &impl Pad) -> Vec<Path> {
    let mut queue = VecDeque::new();
    let mut visited_at_length = HashMap::new();
    let mut min_path_len = usize::MAX;
    let mut paths = Vec::new();

    queue.push_back(Path {
        robot: start,
        moves: String::new(),
    });
    visited_at_length.insert(start, 0);

    while let Some(Path { robot, mut moves }) = queue.pop_front() {
        if !paths.is_empty() && moves.len() > min_path_len {
            continue;
        }

        if pad.get_val(robot.0) == target {
            let mut final_path = Path {
                robot,
                moves: moves.clone(),
            };
            final_path.moves.push('A');

            if paths.is_empty() || final_path.moves.len() == min_path_len {
                min_path_len = final_path.moves.len();
                paths.push(final_path)
            }
            continue;
        }

        for (next_robot, dir) in robot.successors(pad) {
            let next_moves = moves.clone() + &dir.to_char().to_string();
            let curr_length = next_moves.len();

            if let Some(&prev_length) = visited_at_length.get(&next_robot) {
                if curr_length > prev_length {
                    continue;
                }
            }

            queue.push_back(Path {
                robot: next_robot,
                moves: next_moves,
            });
            visited_at_length.insert(next_robot, curr_length);
        }
    }

    paths
}

fn get_possible_moves(code: String, start: Robot, pad: &impl Pad) -> Vec<String> {
    let mut possible_total_moves = vec![String::new()];
    let mut start = start;
    for target in code.chars() {
        let possible_paths = moves_to_target(start, target, pad);

        let mut new_total_moves = Vec::new();
        for prev_move in possible_total_moves {
            for path in &possible_paths {
                let combined = prev_move.clone() + &path.moves;
                new_total_moves.push(combined);
            }
        }

        possible_total_moves = new_total_moves;

        if let Some(path) = possible_paths.first() {
            start = path.robot;
        } else {
            panic!("No path found");
        }
    }

    possible_total_moves
}

pub mod part1 {
    use super::*;
    use miette::Result;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<String> {
        let mut code = input.to_owned();
        let num_pad = NumKeyPad::new();
        let dir_pad = DirKeyPad::new();
        let mut start = Robot(Coord { row: 3, col: 2 });

        let possible_moves = get_possible_moves(code, start, &num_pad);
        dbg!(&possible_moves);

        for p in possible_moves {
            code = p.clone();
            let possible_moves_dir = get_possible_moves(code, start, &dir_pad);
            dbg!(&possible_moves_dir);
        }

        Ok("abc".to_string())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_process() -> Result<()> {
            let input = "\
029A";

            assert_eq!(
                "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A",
                process(input)?
            );
            Ok(())
        }
    }
}

pub mod part2 {
    use super::*;
    use miette::Result;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<u64> {
        todo!("part 2");
    }
}
