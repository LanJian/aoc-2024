use std::str::FromStr;

use anyhow::anyhow;
use aoc_common::grid::{Coordinate, Grid};
use aoc_plumbing::Problem;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone)]
pub struct HoofIt {
    grid: Grid<u8>,
    trailheads: Vec<Coordinate>,
}

impl FromStr for HoofIt {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cells = Vec::default();
        let mut trailheads = Vec::default();

        for (i, line) in s.lines().enumerate() {
            let mut row = Vec::default();

            for (j, c) in line.chars().enumerate() {
                let d = c
                    .to_digit(10)
                    .map(|x| x as u8)
                    .ok_or_else(|| anyhow!("invalid cell"))?;

                if d == 0 {
                    trailheads.push((i, j).into());
                }

                row.push(d);
            }

            cells.push(row);
        }

        Ok(Self {
            grid: Grid::from(cells),
            trailheads,
        })
    }
}

impl HoofIt {
    fn total_trail_score(&self) -> usize {
        self.trailheads
            .par_iter()
            .map(|x| {
                self.total_trail_score_helper(x, &mut Grid::new(self.grid.n, self.grid.m, false))
            })
            .sum()
    }

    fn total_trail_score_helper(&self, head: &Coordinate, visited: &mut Grid<bool>) -> usize {
        if visited[*head] {
            return 0;
        }

        visited[*head] = true;

        let cell = self.grid[*head];
        if cell == 9 {
            return 1;
        }

        head.cardinal_neighbours()
            .iter()
            .filter(|&x| self.grid.is_in_bounds(*x) && self.grid[*x] == cell + 1)
            .map(|x| self.total_trail_score_helper(x, visited))
            .sum()
    }

    fn total_trail_rating(&self) -> usize {
        let mut memo = Grid::new(self.grid.n, self.grid.m, None);

        self.trailheads
            .iter()
            .map(|x| self.total_trail_rating_helper(x, &mut memo))
            .sum()
    }

    fn total_trail_rating_helper(
        &self,
        head: &Coordinate,
        memo: &mut Grid<Option<usize>>,
    ) -> usize {
        let cell = self.grid[*head];
        if cell == 9 {
            memo[*head] = Some(1)
        }

        if let Some(x) = memo[*head] {
            return x;
        }

        let result: usize = head
            .cardinal_neighbours()
            .iter()
            .filter(|&x| self.grid.is_in_bounds(*x) && self.grid[*x] == cell + 1)
            .map(|x| self.total_trail_rating_helper(x, memo))
            .sum();

        memo[*head] = Some(result);
        result
    }
}

impl Problem for HoofIt {
    const DAY: usize = 10;
    const TITLE: &'static str = "hoof it";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.total_trail_score())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.total_trail_rating())
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
        let solution = HoofIt::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(776, 1657));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = HoofIt::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(36, 81));
    }
}
