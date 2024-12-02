use std::{collections::BinaryHeap, str::FromStr};

use anyhow::bail;
use aoc_plumbing::Problem;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
pub struct HistorianHysteria {
    total_distance: u32,
    total_similarity: usize,
}

impl FromStr for HistorianHysteria {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut left = BinaryHeap::default();
        let mut right = BinaryHeap::default();
        let mut freq = FxHashMap::default();

        for line in s.lines() {
            if let Some((a, b)) = line.split_once("   ") {
                left.push(a.parse::<u32>()?);
                let parsed_b = b.parse::<u32>()?;
                right.push(parsed_b);
                freq.entry(parsed_b).and_modify(|x| *x += 1).or_insert(1);
            } else {
                bail!("could not parse line")
            }
        }

        let mut total_distance = 0;
        let mut total_similarity = 0;
        while let (Some(a), Some(b)) = (left.pop(), right.pop()) {
            total_distance += a.abs_diff(b);
            total_similarity += a as usize * freq.get(&a).unwrap_or(&0);
        }

        Ok(Self {
            total_distance,
            total_similarity,
        })
    }
}

impl HistorianHysteria {
    fn total_distance(&self) -> u32 {
        self.total_distance
    }

    fn total_similarity(&self) -> usize {
        self.total_similarity
    }
}

impl Problem for HistorianHysteria {
    const DAY: usize = 1;
    const TITLE: &'static str = "historian hysteria";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = u32;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.total_distance())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.total_similarity())
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
        let solution = HistorianHysteria::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(2378066, 18934359));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = HistorianHysteria::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(11, 31));
    }
}
