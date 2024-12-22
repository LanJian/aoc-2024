use std::str::FromStr;

use anyhow::{anyhow, bail, Ok};
use aoc_common::algebra::{Point3, Vector3};
use aoc_plumbing::Problem;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Robot {
    position: Point3<i64>,
    velocity: Vector3<i64>,
}

impl Robot {
    fn simulate(&self, width: i64, height: i64, seconds: i64) -> Point3<i64> {
        let mut ret = self.position + self.velocity * seconds;
        ret.x = ret.x.rem_euclid(width);
        ret.y = ret.y.rem_euclid(height);
        ret
    }
}

impl FromStr for Robot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .split_once(' ')
            .ok_or_else(|| anyhow!("invalid robot input"))?;

        let position = left[2..left.len()]
            .split_once(',')
            .and_then(|(l, r)| {
                l.parse::<i64>()
                    .and_then(|x| r.parse::<i64>().map(|y| Point3::new(x, y, 0)))
                    .ok()
            })
            .ok_or_else(|| anyhow!("invalid position input"))?;

        let velocity = right[2..right.len()]
            .split_once(',')
            .and_then(|(l, r)| {
                l.parse::<i64>()
                    .and_then(|x| r.parse::<i64>().map(|y| Vector3::new(x, y, 0)))
                    .ok()
            })
            .ok_or_else(|| anyhow!("invalid velocity input"))?;

        Ok(Self { position, velocity })
    }
}

#[derive(Debug, Clone)]
pub struct RestroomRedoubt {
    robots: Vec<Robot>,
}

impl RestroomRedoubt {
    fn safety_factor(&self, width: i64, height: i64) -> usize {
        let mut counts = [0_usize; 4];

        // this is fine because we are only given odd number size
        let mx = width / 2;
        let my = height / 2;

        for r in &self.robots {
            let p = r.simulate(width, height, 100);
            if (0..mx).contains(&p.x) && (0..my).contains(&p.y) {
                counts[0] += 1;
            } else if (mx + 1..width).contains(&p.x) && (0..my).contains(&p.y) {
                counts[1] += 1;
            } else if (0..mx).contains(&p.x) && (my + 1..height).contains(&p.y) {
                counts[2] += 1;
            } else if (mx + 1..width).contains(&p.x) && (my + 1..height).contains(&p.y) {
                counts[3] += 1;
            }
        }

        counts.iter().product()
    }

    fn find_easter_egg(&self, width: i64, height: i64) -> Result<i64, anyhow::Error> {
        let mut c = 0;
        let mut r = 0;

        for i in 0..=height {
            let mut columns = vec![0; width as usize];
            let mut rows = vec![0; height as usize];

            for robot in &self.robots {
                let p = robot.simulate(width, height, i);
                columns[p.x as usize] += 1;
                rows[p.y as usize] += 1;

                if columns[p.x as usize] >= 31 {
                    c = i;
                }

                if rows[p.y as usize] >= 31 {
                    r = i;
                }

                if c > 0 && r > 0 {
                    break;
                }
            }
        }

        for k in 1..500 {
            let num = k * width + c - r;
            if num % height == 0 {
                return Ok(k * width + c);
            }
        }

        bail!("could not find easter egg")
    }
}

impl FromStr for RestroomRedoubt {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            robots: s
                .lines()
                .map(Robot::from_str)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl Problem for RestroomRedoubt {
    const DAY: usize = 14;
    const TITLE: &'static str = "restroom redoubt";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.safety_factor(101, 103))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        self.find_easter_egg(101, 103)
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
        let solution = RestroomRedoubt::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(229839456, 7138));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let part1 = RestroomRedoubt::from_str(&input)
            .unwrap()
            .safety_factor(11, 7);
        assert_eq!(part1, 12);
    }
}
