use std::str::FromStr;

use aoc_common::grid::{Coordinate, Grid};
use aoc_plumbing::Problem;

#[derive(Debug, Clone)]
pub struct CeresSearch {
    grid: Grid<char>,
}

impl FromStr for CeresSearch {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            grid: Grid::from_str(s)?,
        })
    }
}

impl CeresSearch {
    fn total_xmas_occurences(&self) -> usize {
        let mut ret = 0;

        for i in 0..self.grid.n {
            for j in 0..self.grid.m {
                let coord = (i, j).into();
                if self.grid[coord] == 'X' {
                    ret += self.search_xmas_from(&coord);
                }
            }
        }

        ret
    }

    fn total_x_max_occurrences(&self) -> usize {
        let mut ret = 0;

        for i in 0..self.grid.n {
            for j in 0..self.grid.m {
                let coord = (i, j).into();
                if self.grid[coord] == 'A' {
                    ret += self.search_x_mas_from(&coord);
                }
            }
        }

        ret
    }

    fn search_xmas_from(&self, coord: &Coordinate) -> usize {
        let i = coord.row();
        let j = coord.col();

        let candidates = [
            [(i, j), (i - 1, j), (i - 2, j), (i - 3, j)],
            [(i, j), (i + 1, j), (i + 2, j), (i + 3, j)],
            [(i, j), (i, j - 1), (i, j - 2), (i, j - 3)],
            [(i, j), (i, j + 1), (i, j + 2), (i, j + 3)],
            [(i, j), (i - 1, j - 1), (i - 2, j - 2), (i - 3, j - 3)],
            [(i, j), (i - 1, j + 1), (i - 2, j + 2), (i - 3, j + 3)],
            [(i, j), (i + 1, j - 1), (i + 2, j - 2), (i + 3, j - 3)],
            [(i, j), (i + 1, j + 1), (i + 2, j + 2), (i + 3, j + 3)],
        ];

        candidates
            .iter()
            .filter(|&x| {
                x.map(|y| self.grid.get(y.into())) == [Some('X'), Some('M'), Some('A'), Some('S')]
            })
            .count()
    }

    fn search_x_mas_from(&self, coord: &Coordinate) -> usize {
        let i = coord.row();
        let j = coord.col();

        let candidates = [
            [(i - 1, j - 1), (i, j), (i + 1, j + 1)],
            [(i - 1, j + 1), (i, j), (i + 1, j - 1)],
        ];

        if candidates.iter().all(|x| {
            let sequence = x.map(|y| self.grid.get(y.into()));
            sequence == [Some('M'), Some('A'), Some('S')]
                || sequence == [Some('S'), Some('A'), Some('M')]
        }) {
            return 1;
        }

        0
    }
}

impl Problem for CeresSearch {
    const DAY: usize = 4;
    const TITLE: &'static str = "ceres search";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.total_xmas_occurences())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.total_x_max_occurrences())
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
        let solution = CeresSearch::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(2646, 2000));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = CeresSearch::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(18, 9));
    }
}
