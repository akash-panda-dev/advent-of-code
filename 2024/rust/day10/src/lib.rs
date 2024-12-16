use std::{collections::HashSet, fmt};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Loc(i32, i32);

#[derive(Clone)]
struct Grid {
    trails: Vec<Vec<u32>>,
    trail_heads: Vec<Loc>,
    bounds: (i32, i32),
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Grid {{")?;

        // Print bounds
        writeln!(f, "  bounds: ({}, {}),", self.bounds.0, self.bounds.1)?;

        // Print trail heads
        writeln!(f, "  trail_heads: [")?;
        for head in &self.trail_heads {
            writeln!(f, "    {:?},", head)?;
        }
        writeln!(f, "  ],")?;

        // Print trails matrix
        writeln!(f, "  trails: [")?;
        for row in &self.trails {
            write!(f, "    [")?;
            for (i, &cell) in row.iter().enumerate() {
                if i < row.len() - 1 {
                    write!(f, "{}, ", cell)?;
                } else {
                    write!(f, "{}", cell)?;
                }
            }
            writeln!(f, "],")?;
        }
        writeln!(f, "  ]")?;

        write!(f, "}}")?;
        Ok(())
    }
}

impl Grid {
    const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    fn is_outside(&self, loc: &Loc) -> bool {
        loc.0 < 0 || loc.1 < 0 || loc.0 > self.bounds.0 || loc.1 > self.bounds.1
    }

    fn parse(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let bounds = (lines.len() as i32 - 1, lines[0].len() as i32 - 1);
        // let mut trails: HashMap<Loc, u32> = HashMap::new();
        let mut trails: Vec<Vec<u32>> = Vec::new();
        let mut trail_heads: Vec<Loc> = Vec::new();

        for (row, line) in lines.iter().enumerate() {
            let mut row_vec: Vec<u32> = Vec::new();
            for (col, ch) in line.chars().enumerate() {
                let loc = Loc(row as i32, col as i32);
                let trail = ch
                    .to_digit(10)
                    .unwrap_or_else(|| panic!("Invalid digit character: '{}'", ch));

                if trail == 0 {
                    trail_heads.push(loc);
                }

                row_vec.push(trail);
            }
            trails.push(row_vec);
        }

        Grid {
            trails,
            trail_heads,
            bounds,
        }
    }

    fn get_trail(&self, loc: Loc) -> Option<u32> {
        if self.is_outside(&loc) {
            None
        } else {
            Some(self.trails[loc.0 as usize][loc.1 as usize])
        }
    }

    // fn get_trail_score(
    //     &self,
    //     mut score: u32,
    //     trail: u32,
    //     loc: Loc,
    //     visited_nines: &mut HashSet<Loc>,
    // ) -> u32 {
    //     match trail {
    //         9 => {
    //             if !visited_nines.insert(loc) {
    //                 score
    //             } else {
    //                 score + 1
    //             }
    //         }
    //         trail_num => {
    //             for (dx, dy) in Self::DIRECTIONS {
    //                 let new_loc = Loc(loc.0 + dx, loc.1 + dy);
    //                 let new_trail = self.get_trail(new_loc);
    //
    //                 if let Some(nt) = new_trail {
    //                     if nt == trail_num + 1 {
    //                         score = self.get_trail_score(score, nt, new_loc, visited_nines);
    //                     }
    //                 }
    //             }
    //             score
    //         }
    //     }
    // }

    fn get_trail_score_func(&self, trail: u32, loc: Loc, visited_nines: &mut HashSet<Loc>) -> u32 {
        match trail {
            9 => {
                if !visited_nines.insert(loc) {
                    0
                } else {
                    1
                }
            }
            trail_num => Self::DIRECTIONS
                .iter()
                .filter_map(|(dx, dy)| {
                    let new_loc = Loc(loc.0 + dx, loc.1 + dy);
                    let new_trail = self.get_trail(new_loc)?;

                    if new_trail == trail_num + 1 {
                        Some(self.get_trail_score_func(new_trail, new_loc, visited_nines))
                    } else {
                        None
                    }
                })
                .sum(),
        }
    }

    fn get_trail_rating(&self, trail: u32, loc: Loc) -> u32 {
        match trail {
            9 => 1,
            trail_num => Self::DIRECTIONS
                .iter()
                .filter_map(|(dx, dy)| {
                    let new_loc = Loc(loc.0 + dx, loc.1 + dy);
                    let new_trail = self.get_trail(new_loc)?;

                    if new_trail == trail_num + 1 {
                        Some(self.get_trail_rating(new_trail, new_loc))
                    } else {
                        None
                    }
                })
                .sum(),
        }
    }
}

pub mod part1 {
    use super::*;
    use miette::Result;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<u32> {
        let grid = Grid::parse(&input);

        let score_sum: u32 = grid
            .trail_heads
            .iter()
            .map(|th| grid.get_trail_score_func(0, *th, &mut HashSet::new()))
            .sum::<u32>();

        // let score_sum: u32 = grid
        //     .trail_heads
        //     .iter()
        //     .map(|th| grid.get_trail_score(0, 0, *th, &mut HashSet::new()))
        //     .sum::<u32>();
        Ok(score_sum)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_process() -> Result<()> {
            let input = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
            assert_eq!(36, process(input)?);
            Ok(())
        }
    }
}

pub mod part2 {
    use super::*;
    use miette::Result;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<u32> {
        let grid = Grid::parse(&input);

        let score_sum: u32 = grid
            .trail_heads
            .iter()
            .map(|th| grid.get_trail_rating(0, *th))
            .sum::<u32>();

        Ok(score_sum)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_process() -> Result<()> {
            let input = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
            assert_eq!(81, process(input)?);
            Ok(())
        }
    }
}
