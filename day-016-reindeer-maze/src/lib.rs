use std::str::FromStr;

use anyhow::anyhow;
use aoc_common::{
    direction::Cardinal,
    grid::{Coordinate, Grid},
    pathfinding,
};
use aoc_plumbing::Problem;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone)]
pub struct ReindeerMaze {
    grid: Grid<char>,
    start: Coordinate,
    end: Coordinate,
    dijkstra_result: Option<(i64, Vec<Vec<Node>>)>,
}

impl FromStr for ReindeerMaze {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::from_str(s)?;
        let start = grid
            .find_coordinate(|&x| x == 'S')
            .ok_or_else(|| anyhow!("could not find start"))?;
        let end = grid
            .find_coordinate(|&x| x == 'E')
            .ok_or_else(|| anyhow!("could not find end"))?;

        Ok(Self {
            grid,
            start,
            end,
            dijkstra_result: None,
        })
    }
}

impl ReindeerMaze {
    fn part1(&mut self) -> Option<i64> {
        let successors = |x: &Node| {
            let candidates = [
                (
                    Node {
                        coord: x.coord.neighbour(&x.facing),
                        facing: x.facing,
                    },
                    1,
                ),
                (
                    Node {
                        coord: x.coord.neighbour(&x.facing.left()),
                        facing: x.facing.left(),
                    },
                    1001,
                ),
                (
                    Node {
                        coord: x.coord.neighbour(&x.facing.right()),
                        facing: x.facing.right(),
                    },
                    1001,
                ),
                (
                    Node {
                        coord: x.coord.neighbour(&x.facing.opposite()),
                        facing: x.facing.opposite(),
                    },
                    2001,
                ),
            ];

            candidates
                .into_iter()
                .filter(|(n, _)| self.grid.get(n.coord).is_some_and(|c| c == '.' || c == 'E'))
        };

        let success = |x: &Node| x.coord == self.end;

        let start = Node {
            coord: self.start,
            facing: Cardinal::East,
        };

        self.dijkstra_result = pathfinding::dijkstra_with_paths(&start, successors, success);
        self.dijkstra_result.as_ref().map(|(cost, _)| *cost)
    }

    fn part2(&self) -> Option<usize> {
        match &self.dijkstra_result {
            Some((_, paths)) => {
                let coords = paths
                    .iter()
                    .flatten()
                    .map(|x| x.coord)
                    .collect::<FxHashSet<_>>();

                Some(coords.len())
            }
            None => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Node {
    coord: Coordinate,
    facing: Cardinal,
}

impl Problem for ReindeerMaze {
    const DAY: usize = 16;
    const TITLE: &'static str = "reindeer maze";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i64;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        self.part1().ok_or_else(|| anyhow!("could not find path"))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        self.part2().ok_or_else(|| anyhow!("could not find path"))
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
        let solution = ReindeerMaze::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(108504, 538));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = ReindeerMaze::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(7036, 45));
    }
}
