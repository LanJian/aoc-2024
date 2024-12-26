use std::str::FromStr;

use anyhow::{anyhow, bail};
use aoc_common::{
    direction::Cardinal,
    grid::{Coordinate, Grid},
};
use aoc_plumbing::Problem;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
pub struct WarehouseWoes {
    grid: Grid<char>,
    wide_grid: Grid<char>,
    robot: Coordinate,
    wide_robot: Coordinate,
    moves: Vec<Cardinal>,
}

impl FromStr for WarehouseWoes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (top, bottom) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("invalid input"))?;

        let grid = Grid::from_str(top)?;
        let robot = grid
            .find_coordinate(|&x| x == '@')
            .ok_or_else(|| anyhow!("could not find robot in grid"))?;
        let moves = bottom
            .chars()
            .filter(|x| *x != '\n')
            .map(|x| match x {
                '^' => Ok(Cardinal::North),
                '>' => Ok(Cardinal::East),
                '<' => Ok(Cardinal::West),
                'v' => Ok(Cardinal::South),
                _ => bail!("invalid moves"),
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut wide_grid = Grid::new(grid.n, grid.m * 2, '.');
        for i in 0..grid.n {
            for j in 0..grid.m {
                match grid[(i, j).into()] {
                    '#' => {
                        wide_grid[(i, j * 2).into()] = '#';
                        wide_grid[(i, j * 2 + 1).into()] = '#';
                    }
                    'O' => {
                        wide_grid[(i, j * 2).into()] = '[';
                        wide_grid[(i, j * 2 + 1).into()] = ']';
                    }
                    '@' => {
                        wide_grid[(i, j * 2).into()] = '@';
                    }
                    _ => (),
                }
            }
        }
        let wide_robot = wide_grid
            .find_coordinate(|&x| x == '@')
            .ok_or_else(|| anyhow!("could not find robot in wide grid"))?;

        Ok(Self {
            grid,
            moves,
            robot,
            wide_robot,
            wide_grid,
        })
    }
}

impl WarehouseWoes {
    fn simulate(&mut self) {
        for dir in &self.moves.clone() {
            self.simulate_one(dir);
        }
    }

    fn simulate_one(&mut self, dir: &Cardinal) {
        let mut coord = self.robot.neighbour(dir);
        let mut b = None;

        while let Some(x) = self.grid.get(coord) {
            match x {
                'O' if b.is_none() => b = Some(coord),
                '#' => break,
                '.' => {
                    if let Some(b_pos) = b {
                        self.grid[coord] = 'O';
                        self.grid[b_pos] = '@';
                        self.grid[self.robot] = '.';
                        self.robot = b_pos;
                    } else {
                        self.grid[coord] = '@';
                        self.grid[self.robot] = '.';
                        self.robot = coord;
                    }
                    break;
                }
                _ => (),
            }

            coord = coord.neighbour(dir);
        }
    }

    fn sum_gps(&self) -> usize {
        let mut ret = 0;

        for i in 0..self.grid.n {
            for j in 0..self.grid.m {
                if self.grid[(i, j).into()] == 'O' {
                    ret += i * 100 + j;
                }
            }
        }

        ret
    }

    fn wide_simulate(&mut self) {
        for dir in &self.moves.clone() {
            self.wide_simulate_one(dir);
        }
    }

    fn wide_simulate_one(&mut self, dir: &Cardinal) {
        let mut coord = self.wide_robot.neighbour(dir);

        if *dir == Cardinal::West || *dir == Cardinal::East {
            let mut boxes = Vec::default();

            while let Some(x) = self.wide_grid.get(coord) {
                match x {
                    '[' => boxes.push(coord),
                    '#' => break,
                    '.' => {
                        for c in &boxes {
                            self.wide_grid[*c] = '.';
                            self.wide_grid[c.east()] = '.';
                        }
                        for c in &boxes {
                            self.wide_grid[c.neighbour(dir)] = '[';
                            self.wide_grid[c.east().neighbour(dir)] = ']';
                        }
                        self.wide_grid[self.wide_robot.neighbour(dir)] = '@';
                        self.wide_grid[self.wide_robot] = '.';
                        self.wide_robot = self.wide_robot.neighbour(dir);
                        break;
                    }
                    _ => (),
                }

                coord = coord.neighbour(dir);
            }
        } else {
            let mut boxes = FxHashMap::default();
            let can_move = self.find_wide_boxes(&coord, dir, &mut boxes);
            if can_move {
                for c in boxes.keys() {
                    self.wide_grid[*c] = '.';
                    self.wide_grid[c.east()] = '.';
                }
                for c in boxes.keys() {
                    self.wide_grid[c.neighbour(dir)] = '[';
                    self.wide_grid[c.east().neighbour(dir)] = ']';
                }
                self.wide_grid[self.wide_robot.neighbour(dir)] = '@';
                self.wide_grid[self.wide_robot] = '.';
                self.wide_robot = self.wide_robot.neighbour(dir);
            }
        }
    }

    fn find_wide_boxes(
        &self,
        coord: &Coordinate,
        dir: &Cardinal,
        acc: &mut FxHashMap<Coordinate, bool>,
    ) -> bool {
        if let Some(x) = self.wide_grid.get(*coord) {
            match x {
                '#' => false,
                '.' => true,
                '[' => {
                    if let Some(b) = acc.get(coord) {
                        *b
                    } else {
                        let result = self.find_wide_boxes(&coord.neighbour(dir), dir, acc)
                            && self.find_wide_boxes(&coord.east().neighbour(dir), dir, acc);
                        acc.insert(*coord, result);
                        result
                    }
                }
                ']' => {
                    if let Some(b) = acc.get(&coord.west()) {
                        *b
                    } else {
                        let result = self.find_wide_boxes(&coord.west().neighbour(dir), dir, acc)
                            && self.find_wide_boxes(&coord.neighbour(dir), dir, acc);
                        acc.insert(coord.west(), result);
                        result
                    }
                }
                _ => unreachable!(),
            }
        } else {
            false
        }
    }

    fn wide_sum_gps(&self) -> usize {
        let mut ret = 0;

        for i in 0..self.wide_grid.n {
            for j in 0..self.wide_grid.m {
                if self.wide_grid[(i, j).into()] == '[' {
                    ret += i * 100 + j;
                }
            }
        }

        ret
    }
}

impl Problem for WarehouseWoes {
    const DAY: usize = 15;
    const TITLE: &'static str = "warehouse woes";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        self.simulate();
        Ok(self.sum_gps())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        self.wide_simulate();
        Ok(self.wide_sum_gps())
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
        let solution = WarehouseWoes::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(1490942, 1519202));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = WarehouseWoes::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(2028, 1751));
    }

    #[test]
    fn example2() {
        let input = std::fs::read_to_string("example2.txt").expect("Unable to load input");
        let solution = WarehouseWoes::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(10092, 9021));
    }

    #[test]
    fn example3() {
        let input = std::fs::read_to_string("example3.txt").expect("Unable to load input");
        let solution = WarehouseWoes::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(908, 618));
    }
}
