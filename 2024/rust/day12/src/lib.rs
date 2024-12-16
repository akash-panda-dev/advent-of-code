use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt,
};

use miette::Result;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Loc(i32, i32);

#[derive(Clone)]
struct Grid {
    gardens: Vec<Vec<char>>,
    bounds: (u32, u32),
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Grid {{")?;

        // Print bounds
        writeln!(f, "  bounds: ({}, {}),", self.bounds.0, self.bounds.1)?;

        // Print trails matrix
        writeln!(f, "  trails: [")?;
        for row in &self.gardens {
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
    const DIAGONALS: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, 1), (1, -1)];
    const VERTICAL: [(i32, i32); 2] = [(-1, 0), (1, 0)];
    const HORIZONTAL: [(i32, i32); 2] = [(0, -1), (0, 1)];

    fn is_outside(&self, loc: &Loc) -> bool {
        loc.0 < 0 || loc.1 < 0 || loc.0 > self.bounds.0 as i32 || loc.1 > self.bounds.1 as i32
    }

    fn parse(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let bounds = (lines.len() as u32 - 1, lines[0].len() as u32 - 1);
        let mut gardens: Vec<Vec<char>> = Vec::new();

        for (row, line) in lines.iter().enumerate() {
            let mut row_vec: Vec<char> = Vec::new();
            for (col, ch) in line.chars().enumerate() {
                row_vec.push(ch);
            }
            gardens.push(row_vec);
        }

        Grid { gardens, bounds }
    }

    fn find_cost_map(&self) -> HashMap<char, Vec<(u64, u64)>> {
        let mut visited: HashSet<Loc> = HashSet::new();
        let mut cost_map: HashMap<char, Vec<(u64, u64)>> = HashMap::new();

        for row in 0..=self.bounds.0 {
            for col in 0..=self.bounds.1 {
                let loc = Loc(row as i32, col as i32);
                // dbg!(loc);
                if !visited.contains(&loc) {
                    let curr_garden = self.gardens[loc.0 as usize][loc.1 as usize];
                    let area_and_perimeter = self.bfs(loc, &mut visited);
                    // dbg!(curr_garden);

                    cost_map
                        .entry(curr_garden)
                        .and_modify(|costs| costs.push(area_and_perimeter))
                        .or_insert(vec![area_and_perimeter]);
                }
            }
        }

        cost_map
    }

    fn is_loc_reachable(&self, new_loc: &Loc, curr_garden: char) -> bool {
        !self.is_outside(new_loc)
            && self.gardens[new_loc.0 as usize][new_loc.1 as usize] == curr_garden
    }

    fn bfs(&self, start_loc: Loc, visited: &mut HashSet<Loc>) -> (u64, u64) {
        let mut area: u64 = 0;
        let mut perimeter: u64 = 0;
        let mut queue = VecDeque::new();

        queue.push_back(start_loc);
        visited.insert(start_loc);

        while let Some(loc) = queue.pop_front() {
            area += 1;
            let curr_garden = self.gardens[loc.0 as usize][loc.1 as usize];

            for (dx, dy) in Self::DIRECTIONS.iter() {
                let new_loc = Loc(loc.0 + dx, loc.1 + dy);
                if self.is_loc_reachable(&new_loc, curr_garden) {
                    if !visited.contains(&new_loc) {
                        queue.push_back(new_loc);
                        visited.insert(new_loc);
                    }
                } else {
                    perimeter += 1;
                }
            }
        }

        (area, perimeter)
    }

    fn find_cost_map_discounted(&self) -> HashMap<char, Vec<(u64, u64)>> {
        let mut visited: HashSet<Loc> = HashSet::new();
        let mut cost_map: HashMap<char, Vec<(u64, u64)>> = HashMap::new();

        for row in 0..=self.bounds.0 {
            for col in 0..=self.bounds.1 {
                let loc = Loc(row as i32, col as i32);
                // dbg!(loc);
                if !visited.contains(&loc) {
                    let curr_garden = self.gardens[loc.0 as usize][loc.1 as usize];
                    let area_and_perimeter = self.bfs2(loc, &mut visited);
                    // dbg!(curr_garden);

                    cost_map
                        .entry(curr_garden)
                        .and_modify(|costs| costs.push(area_and_perimeter))
                        .or_insert(vec![area_and_perimeter]);
                }
            }
        }

        cost_map
    }

    fn get_corners(&self, loc: Loc, curr_garden: char) -> Option<Vec<Loc>> {
        const CORNER_TRIPLETS: [[(i32, i32); 3]; 4] = [
            [(-1, 0), (-1, -1), (0, -1)],
            [(1, 0), (1, -1), (0, -1)],
            [(-1, 0), (-1, 1), (0, 1)],
            [(1, 0), (1, 1), (0, 1)],
        ];

        let corners = CORNER_TRIPLETS
            .iter()
            .filter_map(|triplet| {
                let side1_loc = Loc(loc.0 + triplet[0].0, loc.1 + triplet[0].1);
                let diagonal_loc = Loc(loc.0 + triplet[1].0, loc.1 + triplet[1].1);
                let side2_loc = Loc(loc.0 + triplet[2].0, loc.1 + triplet[2].1);

                let side1_different = self.is_outside(&side1_loc)
                    || self.gardens[side1_loc.0 as usize][side1_loc.1 as usize] != curr_garden;
                let diagonal_different = self.is_outside(&diagonal_loc)
                    || self.gardens[diagonal_loc.0 as usize][diagonal_loc.1 as usize]
                        != curr_garden;
                let side2_different = self.is_outside(&side2_loc)
                    || self.gardens[side2_loc.0 as usize][side2_loc.1 as usize] != curr_garden;

                if (!side1_different && !side2_different && diagonal_different)
                    || (diagonal_different && side1_different && side2_different)
                    || (side1_different && side2_different)
                {
                    Some(oc)
                } else {
                    None
                }
            })
            .collect::<Vec<Loc>>();

        if corners.is_empty() {
            None
        } else {
            Some(corners)
        }
    }

    fn bfs2(&self, start_loc: Loc, visited: &mut HashSet<Loc>) -> (u64, u64) {
        let mut area: u64 = 0;
        let mut queue = VecDeque::new();
        let mut perimeter_gardens: Vec<Loc> = Vec::new();

        queue.push_back(start_loc);
        visited.insert(start_loc);

        while let Some(loc) = queue.pop_front() {
            area += 1;
            let curr_garden = self.gardens[loc.0 as usize][loc.1 as usize];
            if let Some(corners) = self.get_corners(loc, curr_garden) {
                perimeter_gardens.extend(corners);
            }

            for (dx, dy) in Self::DIRECTIONS.iter() {
                let new_loc = Loc(loc.0 + dx, loc.1 + dy);
                if self.is_loc_reachable(&new_loc, curr_garden) {
                    if !visited.contains(&new_loc) {
                        queue.push_back(new_loc);
                        visited.insert(new_loc);
                    }
                }
            }
        }

        let temp = self.gardens[start_loc.0 as usize][start_loc.1 as usize];
        println!("Garden: {temp}");
        println!("Perimeter garden: \n {:?}", perimeter_gardens);

        (area, perimeter_gardens.len() as u64)
    }

    fn count_sides(perimeter_gardens: Vec<Loc>) -> u64 {
        use std::collections::HashMap;

        // Early return for empty input
        if perimeter_gardens.is_empty() {
            return 0;
        }

        // Group by rows
        let mut rows: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut cols: HashMap<i32, Vec<i32>> = HashMap::new();

        // Fill both maps simultaneously
        for loc in perimeter_gardens {
            rows.entry(loc.0).or_insert_with(Vec::new).push(loc.1);
            cols.entry(loc.1).or_insert_with(Vec::new).push(loc.0);
        }

        // Sort all vectors
        for (_, v) in rows.iter_mut() {
            v.sort_unstable();
        }
        for (_, v) in cols.iter_mut() {
            v.sort_unstable();
        }

        // Count sides in both directions
        let horizontal_sides: u64 = rows
            .values()
            .map(|cols| Self::count_sides_in_direction(cols.clone()))
            .sum();

        let vertical_sides: u64 = cols
            .values()
            .map(|rows| Self::count_sides_in_direction(rows.clone()))
            .sum();

        horizontal_sides + vertical_sides
    }

    fn count_sides_in_direction(coords: Vec<i32>) -> u64 {
        if coords.is_empty() {
            return 0;
        }
        let mut sides = 1; // Start with 1 for the first segment
        let mut windows = coords.windows(2);
        while let Some([curr, next]) = windows.next() {
            // Only count a new side if there's a gap
            if next - curr > 1 {
                sides += 1;
            }
            // Adjacent cells (next - curr == 1) are part of the same side
        }
        sides
    }
}

pub mod part1 {
    use super::*;
    use miette::Result;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<u64> {
        let garden_grid = Grid::parse(&input);
        // dbg!(&garden_grid);

        let cost_map = garden_grid.find_cost_map();
        // dbg!(&cost_map);

        Ok(cost_map
            .values()
            .flatten()
            .map(|(area, perimeter)| area * perimeter)
            .sum())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_process() -> Result<()> {
            let input = "\
AAAA
BBCD
BBCC
EEEC";
            assert_eq!(140, process(input)?);
            Ok(())
        }

        #[test]
        fn test_process1() -> Result<()> {
            let input = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
            assert_eq!(772, process(input)?);
            Ok(())
        }
    }
}

pub mod part2 {
    use super::*;
    use miette::Result;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<u64> {
        let garden_grid = Grid::parse(&input);
        dbg!(&garden_grid);

        let cost_map = garden_grid.find_cost_map_discounted();
        dbg!(&cost_map);

        Ok(cost_map
            .values()
            .flatten()
            .map(|(area, perimeter)| area * perimeter)
            .sum())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_process() -> Result<()> {
            let input = "\
AAAA
BBCD
BBCC
EEEC";
            assert_eq!(80, process(input)?);
            Ok(())
        }

        #[test]
        fn test_process1() -> Result<()> {
            let input = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
            assert_eq!(436, process(input)?);
            Ok(())
        }

        #[test]
        fn test_process2() -> Result<()> {
            let input = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

            assert_eq!(236, process(input)?);
            Ok(())
        }

        #[test]
        fn test_process3() -> Result<()> {
            let input = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
            assert_eq!(368, process(input)?);
            Ok(())
        }
    }
}
