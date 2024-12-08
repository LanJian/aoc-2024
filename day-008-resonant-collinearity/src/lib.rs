use std::str::FromStr;

use anyhow::anyhow;
use aoc_common::grid::{Coordinate, Grid};
use aoc_plumbing::Problem;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone)]
pub struct ResonantCollinearity {
    num_antinodes_without_harmonics: usize,
    num_antinodes_with_harmonics: usize,
}

impl FromStr for ResonantCollinearity {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut antennas: FxHashMap<char, FxHashSet<Coordinate>> = FxHashMap::default();
        let n = s.lines().count();
        let m = s
            .lines()
            .next()
            .ok_or_else(|| anyhow!("could not parse empty input"))?
            .len();

        for (i, line) in s.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                let coord = (i, j).into();
                if c != '.' {
                    antennas
                        .entry(c)
                        .and_modify(|x| {
                            x.insert(coord);
                        })
                        .or_insert([coord].into_iter().collect());
                }
            }
        }

        let mut antinodes_without_harmonics = Grid::new(n, m, false);
        let mut antinodes_with_harmonics = Grid::new(n, m, false);
        let mut num_antinodes_without_harmonics = 0;
        let mut num_antinodes_with_harmonics = 0;

        for v in antennas.values() {
            for pair in v.iter().combinations(2) {
                let (r1, c1) = (pair[0].row(), pair[0].col());
                let (r2, c2) = (pair[1].row(), pair[1].col());
                let dr = r2 - r1;
                let dc = c2 - c1;

                // process candidates without harmonics (part 1)
                for candidate in [(r1 - dr, c1 - dc).into(), (r2 + dr, c2 + dc).into()] {
                    if antinodes_without_harmonics.get(candidate) == Some(false) {
                        antinodes_without_harmonics.set(candidate, true);
                        num_antinodes_without_harmonics += 1;
                    }
                }

                // process candidates with harmonics (part 2)
                let mut candidate = *pair[0];
                while antinodes_with_harmonics.is_in_bounds(candidate) {
                    if antinodes_with_harmonics.get(candidate) == Some(false) {
                        antinodes_with_harmonics.set(candidate, true);
                        num_antinodes_with_harmonics += 1;
                    }

                    candidate = (candidate.row() - dr, candidate.col() - dc).into();
                }

                let mut candidate = *pair[1];
                while antinodes_with_harmonics.is_in_bounds(candidate) {
                    if antinodes_with_harmonics.get(candidate) == Some(false) {
                        antinodes_with_harmonics.set(candidate, true);
                        num_antinodes_with_harmonics += 1;
                    }

                    candidate = (candidate.row() + dr, candidate.col() + dc).into();
                }
            }
        }

        Ok(Self {
            num_antinodes_without_harmonics,
            num_antinodes_with_harmonics,
        })
    }
}

impl Problem for ResonantCollinearity {
    const DAY: usize = 8;
    const TITLE: &'static str = "resonant collinearity";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.num_antinodes_without_harmonics)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.num_antinodes_with_harmonics)
    }
}

#[cfg(test)]
mod tests {
    use aoc_plumbing::Solution;

    use super::*;

    #[test]
    #[ignore]
    fn full_dataset() {
        let input = std::fs::read_to_string("input.txt").expect("Unable to load input");
        let solution = ResonantCollinearity::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(357, 1266));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = ResonantCollinearity::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(14, 34));
    }
}
