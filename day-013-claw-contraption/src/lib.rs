use std::str::FromStr;

use anyhow::anyhow;
use aoc_plumbing::Problem;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
struct Machine {
    x: i64,
    x1: i64,
    x2: i64,
    y: i64,
    y1: i64,
    y2: i64,
}

impl Machine {
    fn tokens_to_win(&self, offset: i64) -> i64 {
        let x = self.x + offset;
        let y = self.y + offset;

        let (num, denom) = (
            self.x2 * y - x * self.y2,
            self.x2 * self.y1 - self.x1 * self.y2,
        );
        let (a, rem) = (num / denom, num % denom);
        if rem != 0 {
            return 0;
        }

        let (num, denom) = (x - self.x1 * a, self.x2);
        let (b, rem) = (num / denom, num % denom);
        if rem != 0 {
            return 0;
        }

        a * 3 + b
    }
}

impl FromStr for Machine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let (x1, y1) = lines
            .next()
            .and_then(|l| {
                l[12..14]
                    .parse::<i64>()
                    .and_then(|x| l[18..20].parse::<i64>().map(|y| (x, y)))
                    .ok()
            })
            .ok_or_else(|| anyhow!("unexpected button A input"))?;

        let (x2, y2) = lines
            .next()
            .and_then(|l| {
                l[12..14]
                    .parse::<i64>()
                    .and_then(|x| l[18..20].parse::<i64>().map(|y| (x, y)))
                    .ok()
            })
            .ok_or_else(|| anyhow!("unexpected button B input"))?;

        let (x, y) = lines
            .next()
            .map(|l| l.split_whitespace().collect::<Vec<_>>())
            .and_then(|v| {
                v[1][2..v[1].len() - 1]
                    .parse::<i64>()
                    .and_then(|x| v[2][2..v[2].len()].parse::<i64>().map(|y| (x, y)))
                    .ok()
            })
            .ok_or_else(|| anyhow!("unexpected prize input"))?;

        Ok(Self {
            x,
            x1,
            x2,
            y,
            y1,
            y2,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ClawContraption {
    machines: Vec<Machine>,
}

impl FromStr for ClawContraption {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let machines = s
            .split("\n\n")
            .map(Machine::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { machines })
    }
}

impl Problem for ClawContraption {
    const DAY: usize = 13;
    const TITLE: &'static str = "claw contraption";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.machines.iter().map(|x| x.tokens_to_win(0)).sum())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self
            .machines
            .iter()
            .map(|x| x.tokens_to_win(10000000000000))
            .sum())
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
        let solution = ClawContraption::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(30413, 92827349540204));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = ClawContraption::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(480, 875318608908));
    }
}
