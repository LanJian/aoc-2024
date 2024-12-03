use std::str::FromStr;

use aoc_plumbing::Problem;
use regex::Regex;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
struct Instruction {
    a: i32,
    b: i32,
    enabled: bool,
}

#[derive(Debug, Clone)]
pub struct MullItOver {
    instructions: Vec<Instruction>,
}

impl FromStr for MullItOver {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))")?;
        let caps = re.captures_iter(s);

        let mut instructions = Vec::default();
        let mut enabled = true;

        for cap in caps {
            if &cap[0] == "do()" {
                enabled = true;
            } else if &cap[0] == "don't()" {
                enabled = false;
            } else {
                instructions.push(Instruction {
                    a: cap[2].parse::<i32>()?,
                    b: cap[3].parse::<i32>()?,
                    enabled,
                });
            }
        }

        Ok(Self { instructions })
    }
}

impl MullItOver {
    fn sum_without_conditionals(&self) -> i32 {
        self.instructions.iter().map(|x| x.a * x.b).sum()
    }

    fn sum_with_conditionals(&self) -> i32 {
        self.instructions
            .iter()
            .filter(|&x| x.enabled)
            .map(|x| x.a * x.b)
            .sum()
    }
}

impl Problem for MullItOver {
    const DAY: usize = 3;
    const TITLE: &'static str = "mull it over";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i32;
    type P2 = i32;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.sum_without_conditionals())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.sum_with_conditionals())
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
        let solution = MullItOver::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(182619815, 80747545));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = MullItOver::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(161, 48));
    }
}
