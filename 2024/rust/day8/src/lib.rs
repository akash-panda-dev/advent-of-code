use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Loc(i64, i64);

#[derive(Clone, Debug)]
struct Grid {
    antennas: HashMap<char, Vec<Loc>>,
    bounds: (i64, i64),
}

impl Grid {
    fn is_outside(&self, loc: &Loc) -> bool {
        loc.0 < 0 || loc.1 < 0 || loc.0 > self.bounds.0 || loc.1 > self.bounds.1
    }

    fn parse(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let bounds = (lines.len() as i64 - 1, lines[0].len() as i64 - 1);
        let mut antennas = HashMap::new();

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let loc = Loc(row as i64, col as i64);
                if ch != '.' {
                    antennas.entry(ch).or_insert_with(Vec::new).push(loc);
                }
            }
        }

        Grid { antennas, bounds }
    }

    // For each loc combinations find antinodes and add them to antinodes
    fn find_antinodes(&self) -> HashSet<Loc> {
        let mut antinodes = HashSet::new();

        for locs in self.antennas.values().filter(|locs| locs.len() > 1) {
            for pair in locs.iter().combinations(2) {
                let (loc1, loc2) = (pair[0], pair[1]);
                let delta = (loc2.0 - loc1.0, loc2.1 - loc1.1);

                let before = Loc(loc1.0 - delta.0, loc1.1 - delta.1);
                let after = Loc(loc1.0 + 2 * delta.0, loc1.1 + 2 * delta.1);

                if !self.is_outside(&before) {
                    antinodes.insert(before);
                }
                if !self.is_outside(&after) {
                    antinodes.insert(after);
                }
            }
        }

        antinodes
    }

    fn find_antinodes_continuous(&self) -> HashSet<Loc> {
        self.antennas
            .values()
            .filter(|locs| locs.len() > 1)
            .flat_map(|locs| locs.iter().combinations(2))
            .flat_map(|pair| {
                let (loc1, loc2) = (pair[0], pair[1]);
                let delta = (loc2.0 - loc1.0, loc2.1 - loc1.1);
                let forward = std::iter::successors(Some(*loc1), move |&loc| {
                    let next = Loc(loc.0 + delta.0, loc.1 + delta.1);
                    (!self.is_outside(&next)).then_some(next)
                });

                let backward = std::iter::successors(Some(*loc1), move |&loc| {
                    let next = Loc(loc.0 - delta.0, loc.1 - delta.1);
                    (!self.is_outside(&next)).then_some(next)
                });

                forward.chain(backward)
            })
            .collect()
    }
}

pub mod part1 {
    use super::*;
    use miette::Result;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<u64> {
        let grid = Grid::parse(&input);
        let antinodes = grid.find_antinodes().iter().copied().collect::<Vec<Loc>>();

        Ok(antinodes.len() as u64)
    }

    #[test]
    fn test_process() -> Result<()> {
        let input = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(14, process(input)?);
        Ok(())
    }
}

pub mod part2 {
    use super::*;
    use miette::Result;

    #[tracing::instrument]
    pub fn process(input: &str) -> Result<u64> {
        let grid = Grid::parse(&input);
        let antinodes = grid
            .find_antinodes_continuous()
            .iter()
            .copied()
            .collect::<Vec<Loc>>();

        Ok(antinodes.len() as u64)
    }
}
