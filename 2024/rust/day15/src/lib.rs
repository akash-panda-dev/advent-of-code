use std::{
    collections::{HashSet, VecDeque},
    fmt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Coord {
    row: u32,
    col: u32,
}

impl Coord {
    fn next_pos(&self, dir: char) -> Coord {
        match dir {
            '^' => Coord {
                row: self.row - 1,
                col: self.col,
            },
            '>' => Coord {
                row: self.row,
                col: self.col + 1,
            },
            'v' => Coord {
                row: self.row + 1,
                col: self.col,
            },
            '<' => Coord {
                row: self.row,
                col: self.col - 1,
            },
            _ => panic!("Invalid direction"),
        }
    }

    fn prev_pos(&self, dir: char) -> Coord {
        match dir {
            '^' => Coord {
                row: self.row + 1,
                col: self.col,
            },
            '>' => Coord {
                row: self.row,
                col: self.col - 1,
            },
            'v' => Coord {
                row: self.row - 1,
                col: self.col,
            },
            '<' => Coord {
                row: self.row,
                col: self.col + 1,
            },
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Clone)]
struct Robot {
    loc: Coord,
    dirs: Vec<char>,
}

#[derive(Clone)]
struct Warehouse {
    items: Vec<Vec<char>>,
    robot: Robot,
}

impl Warehouse {
    fn get_item(&self, loc: Coord) -> char {
        self.items[loc.row as usize][loc.col as usize]
    }

    fn put_item(&mut self, loc: Coord, item: char) {
        let c = self.items[loc.row as usize][loc.col as usize];
        if c == '#' {
            panic!("Cannot put item in a wall");
        }
        self.items[loc.row as usize][loc.col as usize] = item;
    }

    fn remove_item(&mut self, loc: Coord) -> char {
        let c = self.items[loc.row as usize][loc.col as usize];
        if c == '#' {
            panic!("Cannot remove item from a wall");
        }
        self.items[loc.row as usize][loc.col as usize] = '.';
        c
    }

    fn get_neighbours(&self, current: Coord, dir: char) -> Vec<Coord> {
        let mut neighbors = Vec::new();

        match dir {
            '^' | 'v' => {
                if self.get_item(current) == '[' {
                    neighbors.push(current.next_pos('>'));
                } else if self.get_item(current) == ']' {
                    neighbors.push(current.next_pos('<'));
                }
                let next = current.next_pos(dir);
                match self.get_item(next) {
                    '[' | ']' => neighbors.push(next),
                    _ => {}
                }
            }
            '>' | '<' => {
                let next = current.next_pos(dir);
                match self.get_item(next) {
                    '[' | ']' => neighbors.push(next),
                    _ => {}
                }
            }
            _ => panic!("Invalid direction"),
        }

        neighbors
    }

    fn can_move_boxes(&self, box_coords: &[Coord], dir: char) -> bool {
        let current_pos: HashSet<Coord> = box_coords.iter().cloned().collect();
        let next_pos: HashSet<Coord> = current_pos.iter().map(|c| c.next_pos(dir)).collect();

        next_pos.iter().all(|c| match self.get_item(*c) {
            '.' | '[' | ']' => true,
            _ => false,
        })
    }

    fn find_boxes(&self, start: Coord, dir: char) -> Vec<Coord> {
        let mut visited: HashSet<Coord> = HashSet::new();
        let mut queue = VecDeque::from([start]);
        let mut box_coords = Vec::new();

        while let Some(current) = queue.pop_front() {
            if !visited.insert(current) {
                continue;
            }
            box_coords.push(current);
            for n in self.get_neighbours(current, dir) {
                queue.push_back(n);
            }
        }

        box_coords
    }

    fn push_box(&mut self, dir: char) -> bool {
        let box_loc: Coord = self.robot.loc.next_pos(dir);

        let box_coords = self.find_boxes(box_loc, dir);

        if !self.can_move_boxes(&box_coords, dir) {
            eprintln!("Cannot move box");
            return false;
        }

        for coord in box_coords.iter().rev() {
            let item = self.remove_item(*coord);
            let next_pos = coord.next_pos(dir);
            self.put_item(next_pos, item);
        }

        true
    }

    fn move_robot(&mut self, dir: char) {
        let current_pos = self.robot.loc;
        let next_pos: Coord = self.robot.loc.next_pos(dir);

        match self.get_item(next_pos) {
            '.' => {
                self.remove_item(current_pos);
                self.put_item(next_pos, '@');
                self.robot.loc = next_pos;
            }
            'O' => {
                self.push_box(dir);
            }
            '[' | ']' => {
                if self.push_box(dir) {
                    self.remove_item(current_pos);
                    self.put_item(next_pos, '@');
                    self.robot.loc = next_pos;
                    // println!("hurray")
                }
            }
            '#' => {}
            c => panic!("Invalid item: {c}"),
        }
    }
}
impl fmt::Debug for Warehouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Warehouse {{")?;

        writeln!(f, "  items: ")?;
        for row in &self.items {
            write!(f, "    ")?;
            for (_, &cell) in row.iter().enumerate() {
                write!(f, "{}", cell)?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "")?;

        write!(f, "}}")?;
        Ok(())
    }
}

impl Warehouse {
    fn parse(input: &str) -> Self {
        let mut parts = input.split("\n\n");
        let items_str = parts.next().unwrap();
        let dirs_str = parts.next().unwrap();

        let mut items: Vec<Vec<char>> = Vec::new();
        let mut robot_loc = None;

        for (row, item_line) in items_str.lines().enumerate() {
            let mut row_vec: Vec<char> = Vec::new();
            for (col, ch) in item_line.chars().enumerate() {
                row_vec.push(ch);
                if ch == '@' {
                    robot_loc = Some(Coord {
                        row: row as u32,
                        col: col as u32,
                    });
                }
            }
            items.push(row_vec);
        }

        let robot_directions: Vec<char> = dirs_str
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .flatten()
            .collect();

        Warehouse {
            items,
            robot: Robot {
                loc: robot_loc.unwrap(),
                dirs: robot_directions,
            },
        }
    }

    fn parse2(input: &str) -> Self {
        let mut parts = input.split("\n\n");
        let items_str = parts.next().unwrap();
        let dirs_str = parts.next().unwrap();

        let mut items: Vec<Vec<char>> = Vec::new();
        let mut robot_loc = None;

        for (row, item_line) in items_str.lines().enumerate() {
            let mut row_vec: Vec<char> = Vec::new();
            for (col, ch) in item_line.chars().enumerate() {
                if ch == '@' {
                    robot_loc = Some(Coord {
                        row: row as u32,
                        col: (col * 2) as u32,
                    });
                    row_vec.push(ch);
                    row_vec.push('.');
                } else if ch == 'O' {
                    row_vec.push('[');
                    row_vec.push(']');
                } else {
                    row_vec.push(ch);
                    row_vec.push(ch);
                }
            }
            items.push(row_vec);
        }

        let robot_directions: Vec<char> = dirs_str
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .flatten()
            .collect();

        Warehouse {
            items,
            robot: Robot {
                loc: robot_loc.unwrap(),
                dirs: robot_directions,
            },
        }
    }
}

pub mod part1 {
    use super::*;
    use miette::Result;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<u64> {
        let mut wh = Warehouse::parse(&input);
        let dirs = wh.robot.dirs.clone();

        for d in dirs {
            wh.move_robot(d);
        }

        let gps: usize = wh
            .items
            .into_iter()
            .enumerate()
            .flat_map(|(row_i, row)| {
                row.into_iter().enumerate().filter_map(move |(col_i, col)| {
                    if col == 'O' {
                        Some(100 * row_i + col_i)
                    } else {
                        None
                    }
                })
            })
            .sum();

        Ok(gps as u64)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        //         #[test]
        //         fn test_process() -> Result<()> {
        //             let input = "\
        // ##########
        // #..O..O.O#
        // #......O.#
        // #.OO..O.O#
        // #..O@..O.#
        // #O#..O...#
        // #O..O..O.#
        // #.OO.O.OO#
        // #....O...#
        // ##########
        //
        // <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        // vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        // ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        // <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        // ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        // ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        // >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        // <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        // ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        // v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        //
        //             assert_eq!(10092, process(input)?);
        //             Ok(())
        //         }

        #[test]
        fn test_process1() -> Result<()> {
            let input = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

            assert_eq!(12, process(input)?);
            Ok(())
        }
    }
}

pub mod part2 {
    use super::*;
    use miette::Result;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<u64> {
        let mut wh = Warehouse::parse2(&input);
        let dirs = wh.robot.dirs.clone();
        // dbg!(&wh);
        // Find all the box edges to move
        // check if those boxes can be moved. All of their next position can be either a
        // box or empty space. If there is a wall then cannot move
        // In reverse, move the boxes.
        for dir in dirs {
            wh.move_robot(dir);
            // println!("Moved in dir: {}", dir);
            // dbg!(&wh);
        }

        let gps: usize = wh
            .items
            .into_iter()
            .enumerate()
            .flat_map(|(row_i, row)| {
                row.into_iter().enumerate().filter_map(move |(col_i, col)| {
                    if col == '[' {
                        Some(100 * row_i + col_i)
                    } else {
                        None
                    }
                })
            })
            .sum();

        Ok(gps as u64)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_process() -> Result<()> {
            let input = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

            assert_eq!(9021, process(input)?);
            Ok(())
        }

        #[test]
        fn test_process1() -> Result<()> {
            let input = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

            assert_eq!(1751, process(input)?);
            Ok(())
        }
    }
}
