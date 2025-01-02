use std::{collections::VecDeque, str::FromStr};

use anyhow::anyhow;
use aoc_common::grid::{Coordinate, Grid};
use aoc_plumbing::Problem;

#[derive(Debug, Clone)]
pub struct RamRun {
    coords: Vec<Coordinate>,
}

impl FromStr for RamRun {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = Vec::default();

        for l in s.lines() {
            if let Some((left, right)) = l.split_once(',') {
                coords.push(Coordinate::from((
                    right.parse::<isize>()?,
                    left.parse::<isize>()?,
                )));
            }
        }

        Ok(Self { coords })
    }
}

impl RamRun {
    fn part1(&self, size: usize, len: usize) -> Option<usize> {
        let mut grid = Grid::new(size, size, '.');
        self.coords.iter().take(len).for_each(|x| grid[*x] = '#');

        let mut q = VecDeque::default();
        let mut visited = Grid::new(size, size, false);
        let start = Coordinate::from((0_isize, 0_isize));
        let end = Coordinate::from((size - 1, size - 1));
        q.push_back((start, 0));

        while let Some((cur, dist)) = q.pop_front() {
            if visited[cur] {
                continue;
            }

            if cur == end {
                return Some(dist);
            }

            visited[cur] = true;

            q.extend(
                cur.cardinal_neighbours()
                    .into_iter()
                    .filter(|x| grid.get(*x).is_some_and(|y| y == '.'))
                    .map(|x| (x, dist + 1)),
            );
        }

        None
    }

    fn part2(&self, size: usize) -> Option<Coordinate> {
        let mut left = 0;
        let mut right = self.coords.len() - 1;

        while left <= right {
            let mid = (left + right) / 2;

            if self.part1(size, mid + 1).is_none() {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        Some(self.coords[left])
    }
}

impl Problem for RamRun {
    const DAY: usize = 18;
    const TITLE: &'static str = "ram run";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = String;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        self.part1(71, 1024)
            .ok_or_else(|| anyhow!("could not find path"))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        self.part2(71)
            .ok_or_else(|| anyhow!("could not find blocking byte"))
            .map(|x| format!("{},{}", x.col(), x.row()))
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
        let solution = RamRun::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(232, "44,64".to_owned()));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let problem = RamRun::from_str(&input).unwrap();
        assert_eq!(problem.part1(7, 12), Some(22));
        assert_eq!(problem.part2(7), Some((1_isize, 6_isize).into()));
    }
}
