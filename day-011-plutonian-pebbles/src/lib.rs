use std::str::FromStr;

use aoc_plumbing::Problem;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
pub struct PlutonianPebbles {
    pebbles: Vec<u64>,
}

impl FromStr for PlutonianPebbles {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            pebbles: s
                .split_whitespace()
                .map(|x| x.parse())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl PlutonianPebbles {
    fn part1(&self) -> usize {
        let mut memo = FxHashMap::default();
        self.pebbles
            .iter()
            .map(|x| Self::blink(*x, 25, &mut memo))
            .sum()
    }

    fn part2(&self) -> usize {
        let mut memo = FxHashMap::default();
        self.pebbles
            .iter()
            .map(|x| Self::blink(*x, 75, &mut memo))
            .sum()
    }

    fn blink(pebble: u64, iterations: usize, memo: &mut FxHashMap<(u64, usize), usize>) -> usize {
        if iterations == 0 {
            return 1;
        }

        if let Some(x) = memo.get(&(pebble, iterations)) {
            return *x;
        }

        let ret = if pebble == 0 {
            Self::blink(1, iterations - 1, memo)
        } else {
            let digits = pebble.ilog10() + 1;

            if digits % 2 == 0 {
                let divisor = 10_u64.pow(digits / 2);
                Self::blink(pebble / divisor, iterations - 1, memo)
                    + Self::blink(pebble % divisor, iterations - 1, memo)
            } else {
                Self::blink(
                    pebble.checked_mul(2024).expect("overflow"),
                    iterations - 1,
                    memo,
                )
            }
        };

        memo.insert((pebble, iterations), ret);
        ret
    }
}

impl Problem for PlutonianPebbles {
    const DAY: usize = 11;
    const TITLE: &'static str = "plutonian pebbles";
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
        let solution = PlutonianPebbles::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(218956, 259593838049805));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = PlutonianPebbles::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(55312, 65601038650482));
    }
}
