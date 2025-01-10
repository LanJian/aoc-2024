use std::str::FromStr;

use anyhow::anyhow;
use aoc_common::{
    direction::Cardinal,
    grid::{Coordinate, Grid},
};
use aoc_plumbing::Problem;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

#[derive(Debug, Clone)]
pub struct RaceCondition {
    grid: Grid<char>,
    start: Coordinate,
    end: Coordinate,
    acc: Grid<usize>,
    path: Vec<Coordinate>,
}

impl FromStr for RaceCondition {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::from_str(s)?;
        let start = grid
            .find_coordinate(|x| *x == 'S')
            .ok_or_else(|| anyhow!("could not find start"))?;
        let end = grid
            .find_coordinate(|x| *x == 'E')
            .ok_or_else(|| anyhow!("could not find end"))?;
        let acc = Grid::new(grid.n, grid.m, usize::MAX);
        let path = Vec::default();

        Ok(Self {
            grid,
            start,
            end,
            acc,
            path,
        })
    }
}

impl RaceCondition {
    fn part1(&mut self, threshold: usize) -> Result<usize, anyhow::Error> {
        let mut ret = 0;

        let mut dist = 0;
        let mut cur = self.start;

        loop {
            self.acc[cur] = dist;
            self.path.push(cur);
            dist += 1;
            ret += self.short_cheats(&cur, threshold);

            if cur == self.end {
                break;
            }

            cur = *cur
                .cardinal_neighbours()
                .iter()
                .find(|&x| {
                    self.acc.get(*x).is_some_and(|n| n == usize::MAX)
                        && self.grid.get(*x).is_some_and(|c| c == '.' || c == 'E')
                })
                .ok_or_else(|| anyhow!("invalid path"))?;
        }

        Ok(ret)
    }

    fn long_cheats(&self, threshold: usize) -> usize {
        (1..self.path.len())
            .into_par_iter()
            .map(|i| {
                let b = self.path[i];
                self.path[0..i]
                    .iter()
                    .filter(|&a| {
                        let d = a.manhattan_distance(&b);
                        let saved = self.acc[*a].abs_diff(self.acc[b]);
                        d <= 20 && saved >= d && saved - d >= threshold
                    })
                    .count()
            })
            .sum()
    }

    fn short_cheats(&self, coord: &Coordinate, threshold: usize) -> usize {
        let dist = self.acc[*coord];

        let dirs = Cardinal::all();
        dirs.iter()
            .filter(|&dir| {
                self.grid
                    .get(coord.neighbour(dir))
                    .is_some_and(|c| c == '#')
                    && self
                        .grid
                        .get(coord.neighbour(dir).neighbour(dir))
                        .is_some_and(|c| c == '.' || c == 'E')
                    && self
                        .acc
                        .get(coord.neighbour(dir).neighbour(dir))
                        .is_some_and(|n| n < usize::MAX && dist - n - 2 >= threshold)
            })
            .count()
    }
}

impl Problem for RaceCondition {
    const DAY: usize = 20;
    const TITLE: &'static str = "race condition";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        self.part1(100)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.long_cheats(100))
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
        let solution = RaceCondition::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(1459, 1016066));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let mut problem = RaceCondition::from_str(&input).unwrap();
        assert_eq!(5, problem.part1(20).unwrap());
        assert_eq!(41, problem.long_cheats(70));
    }
}
