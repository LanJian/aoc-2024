use std::str::FromStr;

use anyhow::anyhow;
use aoc_plumbing::Problem;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
struct Equation {
    value: u64,
    terms: Vec<u64>,
}

impl FromStr for Equation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (part1, part2) = s
            .split_once(": ")
            .ok_or_else(|| anyhow!("invalid equation"))?;

        let value = part1.parse()?;
        let terms = part2
            .split(' ')
            .map(|x| x.parse())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { value, terms })
    }
}

impl Equation {
    fn test(&self) -> bool {
        self.test_helper(1, self.terms[0])
    }

    fn test_helper(&self, i: usize, acc: u64) -> bool {
        if i == self.terms.len() {
            return acc == self.value;
        }

        if acc > self.value {
            return false;
        }

        self.test_helper(i + 1, acc * self.terms[i]) || self.test_helper(i + 1, acc + self.terms[i])
    }

    fn test_with_concat(&self) -> bool {
        self.test_with_concat_helper(1, self.terms[0])
    }

    fn test_with_concat_helper(&self, i: usize, acc: u64) -> bool {
        if i == self.terms.len() {
            return acc == self.value;
        }

        if acc > self.value {
            return false;
        }

        let term = self.terms[i];
        let exp = term.ilog10() + 1;

        self.test_with_concat_helper(i + 1, acc * term)
            || self.test_with_concat_helper(i + 1, acc * 10_u64.pow(exp) + term)
            || self.test_with_concat_helper(i + 1, acc + term)
    }
}

#[derive(Debug, Clone)]
pub struct BridgeRepair {
    equations: Vec<Equation>,
}

impl FromStr for BridgeRepair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let equations = s
            .lines()
            .map(Equation::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { equations })
    }
}

impl BridgeRepair {
    fn total_calibration_result(&self) -> u64 {
        self.equations
            .iter()
            .filter(|&x| x.test())
            .map(|x| x.value)
            .sum()
    }

    fn total_calibration_result_with_concat(&self) -> u64 {
        self.equations
            .iter()
            .filter(|&x| x.test_with_concat())
            .map(|x| x.value)
            .sum()
    }
}

impl Problem for BridgeRepair {
    const DAY: usize = 7;
    const TITLE: &'static str = "bridge repair";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = u64;
    type P2 = u64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.total_calibration_result())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.total_calibration_result_with_concat())
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
        let solution = BridgeRepair::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(4364915411363, 38322057216320));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = BridgeRepair::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(3749, 11387));
    }
}
