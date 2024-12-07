use std::fmt;
use std::str::FromStr;

use aoc_common::{
    direction::Cardinal,
    grid::{Coordinate, Grid},
};
use aoc_plumbing::Problem;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tile {
    Empty,
    Obstacle,
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Obstacle),
            _ => Ok(Self::Empty),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Obstacle => write!(f, "#"),
            Self::Empty => write!(f, "."),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GuardGallivant {
    grid: Grid<Tile>,
    start_pos: Coordinate,
    start_dir: Cardinal,
}

impl FromStr for GuardGallivant {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::default();
        let mut start_pos = Coordinate::default();
        let mut start_dir = Cardinal::North;

        for (i, line) in s.lines().enumerate() {
            let mut row = Vec::default();

            for (j, c) in line.chars().enumerate() {
                match c {
                    '<' => {
                        start_pos = (i, j).into();
                        start_dir = Cardinal::West;
                    }
                    '>' => {
                        start_pos = (i, j).into();
                        start_dir = Cardinal::East;
                    }
                    '^' => {
                        start_pos = (i, j).into();
                        start_dir = Cardinal::North;
                    }
                    'v' => {
                        start_pos = (i, j).into();
                        start_dir = Cardinal::South;
                    }
                    _ => (),
                };

                row.push(Tile::try_from(c)?);
            }

            grid.push(row);
        }
        Ok(Self {
            grid: grid.into(),
            start_pos,
            start_dir,
        })
    }
}

impl GuardGallivant {
    fn visited_positions(&self) -> usize {
        let mut cur_pos = self.start_pos;
        let mut cur_dir = self.start_dir;

        let mut visited = Grid::new(self.grid.n, self.grid.m, false);
        let mut ret = 0;

        while self.grid.is_in_bounds(cur_pos) {
            if !visited[cur_pos] {
                ret += 1;
                visited[cur_pos] = true;
            }

            while let Some(Tile::Obstacle) = self.grid.get(cur_pos.neighbour(&cur_dir)) {
                cur_dir = cur_dir.right();
            }

            cur_pos = cur_pos.neighbour(&cur_dir);
        }

        ret
    }

    fn loop_positions(&self) -> usize {
        let mut cur_pos = self.start_pos;
        let mut cur_dir = self.start_dir;

        let mut ret = 0;
        let mut visited = Grid::new(self.grid.n, self.grid.m, false);
        visited[self.start_pos] = true;

        while self.grid.is_in_bounds(cur_pos) {
            while let Some(Tile::Obstacle) = self.grid.get(cur_pos.neighbour(&cur_dir)) {
                cur_dir = cur_dir.right();
            }

            let obstacle = cur_pos.neighbour(&cur_dir);
            if visited.is_in_bounds(obstacle) && !visited[obstacle] {
                if self.is_loop_with_obstacle(&cur_pos, &cur_dir) {
                    ret += 1;
                }

                visited[obstacle] = true;
            }

            cur_pos = cur_pos.neighbour(&cur_dir);
        }

        ret
    }

    /// Given a position and direction, simulate an obstacle directly in front and check if this
    /// leads to a loop
    fn is_loop_with_obstacle(&self, pos: &Coordinate, dir: &Cardinal) -> bool {
        let obstacle_pos = pos.neighbour(dir);

        let mut cur_pos = *pos;
        let mut cur_dir = *dir;
        let mut visited = Grid::new(self.grid.n, self.grid.m, None);

        while self.grid.is_in_bounds(cur_pos) {
            if visited[cur_pos] == Some(cur_dir) {
                return true;
            }
            visited[cur_pos] = Some(cur_dir);

            while cur_pos.neighbour(&cur_dir) == obstacle_pos
                || self.grid.get(cur_pos.neighbour(&cur_dir)) == Some(Tile::Obstacle)
            {
                cur_dir = cur_dir.right();
            }

            cur_pos = cur_pos.neighbour(&cur_dir);
        }

        false
    }
}

impl Problem for GuardGallivant {
    const DAY: usize = 6;
    const TITLE: &'static str = "guard gallivant";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.visited_positions())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.loop_positions())
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
        let solution = GuardGallivant::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(4752, 1719));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = GuardGallivant::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(41, 6));
    }
}
