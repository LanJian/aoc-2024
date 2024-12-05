use std::{cmp::Ordering, str::FromStr};

use anyhow::anyhow;
use aoc_plumbing::Problem;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone)]
pub struct PrintQueue {
    valid_sum: i32,
    invalid_sum: i32,
}

impl FromStr for PrintQueue {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (part1, part2) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("could not parse input"))?;

        let rules = part1
            .trim()
            .lines()
            .map(|l| {
                l.split_once('|')
                    .and_then(|(a, b)| match (a.parse::<i32>(), b.parse::<i32>()) {
                        (Ok(parsed_a), Ok(parsed_b)) => Some((parsed_a, parsed_b)),
                        _ => None,
                    })
                    .ok_or_else(|| anyhow!("could not parse rule"))
            })
            .collect::<Result<FxHashSet<_>, _>>()?;

        let mut updates = part2
            .trim()
            .lines()
            .map(|l| {
                l.split(',')
                    .map(|x| x.parse::<i32>())
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut valid_sum = 0;
        let mut invalid_sum = 0;

        for pages in updates.iter_mut() {
            if pages.windows(2).all(|x| rules.contains(&(x[0], x[1]))) {
                valid_sum += pages[pages.len() / 2];
            } else {
                let i = pages.len() / 2;
                let (_, mid, _) = pages.select_nth_unstable_by(i, |&a, &b| {
                    if rules.contains(&(a, b)) {
                        Ordering::Less
                    } else if rules.contains(&(b, a)) {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                });

                invalid_sum += *mid;
            }
        }

        Ok(Self {
            valid_sum,
            invalid_sum,
        })
    }
}

impl PrintQueue {
    fn valid_page_updates(&self) -> i32 {
        self.valid_sum
    }

    fn invalid_page_updates(&self) -> i32 {
        self.invalid_sum
    }
}

impl Problem for PrintQueue {
    const DAY: usize = 5;
    const TITLE: &'static str = "print queue";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i32;
    type P2 = i32;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.valid_page_updates())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.invalid_page_updates())
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
        let solution = PrintQueue::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(5509, 4407));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = PrintQueue::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(143, 123));
    }
}
