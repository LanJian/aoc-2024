use std::str::FromStr;

use aoc_common::grid::{Coordinate, Grid};
use aoc_plumbing::Problem;

#[derive(Debug, Clone)]
pub struct GardenGroups {
    grid: Grid<char>,
}

impl FromStr for GardenGroups {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            grid: Grid::from_str(s)?,
        })
    }
}

impl GardenGroups {
    fn total_price_regular(&self) -> usize {
        let mut visited = Grid::new(self.grid.n, self.grid.m, false);
        let mut ret = 0;

        for i in 0..self.grid.n {
            for j in 0..self.grid.m {
                let coord = (i, j).into();

                if visited[coord] {
                    continue;
                }

                let plant = self.grid[coord];
                let (area, perimeter) =
                    self.total_price_regular_helper(&coord, plant, &mut visited);
                ret += area * perimeter;
            }
        }

        ret
    }

    fn total_price_regular_helper(
        &self,
        coord: &Coordinate,
        plant: char,
        visited: &mut Grid<bool>,
    ) -> (usize, usize) {
        if visited[*coord] {
            return (0, 0);
        }

        visited[*coord] = true;

        let mut area = 1;
        let mut perimeter = 4;
        for n in coord.cardinal_neighbours() {
            if self.grid.is_in_bounds(n) && self.grid[n] == plant {
                let result = self.total_price_regular_helper(&n, plant, visited);
                area += result.0;
                perimeter = perimeter + result.1 - 1;
            }
        }

        (area, perimeter)
    }

    fn total_price_discount(&self) -> usize {
        let mut visited = Grid::new(self.grid.n, self.grid.m, false);
        let mut ret = 0;

        for i in 0..self.grid.n {
            for j in 0..self.grid.m {
                let coord = (i, j).into();

                if visited[coord] {
                    continue;
                }

                let plant = self.grid[coord];
                let (area, turns) = self.total_price_discount_helper(&coord, plant, &mut visited);
                ret += area * turns;
            }
        }

        ret
    }

    fn total_price_discount_helper(
        &self,
        coord: &Coordinate,
        plant: char,
        visited: &mut Grid<bool>,
    ) -> (usize, usize) {
        if visited[*coord] {
            return (0, 0);
        }

        visited[*coord] = true;

        let mut area = 1;
        let mut turns = 0;

        let (mut a, mut b) = (
            self.grid.get(coord.north()).unwrap_or('.'),
            self.grid.get(coord.east()).unwrap_or('.'),
        );
        if (a != plant && b != plant)
            || (a == plant && b == plant && self.grid[coord.northeast()] != plant)
        {
            turns += 1;
        }

        (a, b) = (
            self.grid.get(coord.east()).unwrap_or('.'),
            self.grid.get(coord.south()).unwrap_or('.'),
        );
        if (a != plant && b != plant)
            || (a == plant && b == plant && self.grid[coord.southeast()] != plant)
        {
            turns += 1;
        }

        (a, b) = (
            self.grid.get(coord.south()).unwrap_or('.'),
            self.grid.get(coord.west()).unwrap_or('.'),
        );
        if (a != plant && b != plant)
            || (a == plant && b == plant && self.grid[coord.southwest()] != plant)
        {
            turns += 1;
        }

        (a, b) = (
            self.grid.get(coord.west()).unwrap_or('.'),
            self.grid.get(coord.north()).unwrap_or('.'),
        );
        if (a != plant && b != plant)
            || (a == plant && b == plant && self.grid[coord.northwest()] != plant)
        {
            turns += 1;
        }

        for n in coord.cardinal_neighbours() {
            if self.grid.is_in_bounds(n) && self.grid[n] == plant {
                let result = self.total_price_discount_helper(&n, plant, visited);
                area += result.0;
                turns += result.1;
            }
        }

        (area, turns)
    }
}

impl Problem for GardenGroups {
    const DAY: usize = 12;
    const TITLE: &'static str = "garden groups";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.total_price_regular())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.total_price_discount())
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
        let solution = GardenGroups::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(1433460, 855082));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = GardenGroups::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(1930, 1206));
    }
}
