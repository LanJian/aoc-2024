use std::str::FromStr;

use anyhow::{anyhow, Ok};
use aoc_plumbing::Problem;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone)]
pub struct LinenLayout {
    designs: FxHashMap<String, usize>,
}

impl FromStr for LinenLayout {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (top, bottom) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("invalid input"))?;
        let patterns = top.split(", ").map(|x| x.to_owned()).collect();

        let lines = bottom.lines().collect::<Vec<_>>();
        let designs = lines
            .par_iter()
            .map(|&x| {
                (
                    x.to_owned(),
                    Self::helper(x, &patterns, &mut FxHashMap::default()),
                )
            })
            .collect();

        Ok(LinenLayout { designs })
    }
}

impl LinenLayout {
    fn part1(&self) -> usize {
        self.designs.values().filter(|&x| *x > 0).count()
    }

    fn part2(&self) -> usize {
        self.designs.values().sum()
    }

    fn helper(
        design: &str,
        patterns: &FxHashSet<String>,
        memo: &mut FxHashMap<String, usize>,
    ) -> usize {
        for i in 0..design.len() {
            if patterns.contains(&design[0..=i]) {
                memo.insert(design[0..=i].to_owned(), 1);
            }

            for j in 1..=i {
                let left = memo.get(&design[0..j]).copied().unwrap_or_default();
                let right = if patterns.contains(&design[j..=i]) {
                    1
                } else {
                    0
                };
                if left > 0 && right > 0 {
                    let count = left * right;
                    memo.entry(design[0..=i].to_owned())
                        .and_modify(|x| *x += count)
                        .or_insert(count);
                }
            }
        }

        memo.get(design).copied().unwrap_or_default()
    }
}

impl Problem for LinenLayout {
    const DAY: usize = 19;
    const TITLE: &'static str = "linen layout";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.part1())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.part2())
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
        let solution = LinenLayout::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(355, 732978410442050));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = LinenLayout::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(6, 16));
    }
}
