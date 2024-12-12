use std::cmp::Reverse;
use std::{collections::BinaryHeap, str::FromStr};

use aoc_common::interval::Interval;
use aoc_plumbing::Problem;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
pub struct DiskFragmenter {
    file_blocks: Vec<Interval>,
    free_blocks: Vec<Interval>,
}

impl FromStr for DiskFragmenter {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cur = 0;
        let mut iter = s.chars();
        let mut file_blocks = Vec::default();
        let mut free_blocks = Vec::default();

        while let (Some(a), b) = (iter.next(), iter.next()) {
            match a.to_digit(10).map(|d| d as isize) {
                Some(x) if x.is_positive() => {
                    file_blocks.push(Interval::new(cur, cur + x));
                    cur += x;
                }
                _ => (),
            }

            match b.and_then(|d| d.to_digit(10)).map(|d| d as isize) {
                Some(x) if x.is_positive() => {
                    free_blocks.push(Interval::new(cur, cur + x));
                    cur += x;
                }
                _ => (),
            }
        }

        Ok(Self {
            file_blocks,
            free_blocks,
        })
    }
}

impl DiskFragmenter {
    fn part1(&self) -> i64 {
        let mut ret = 0;

        let mut i = 0;
        let mut j = 0;
        let mut k = self.file_blocks.len() - 1;
        let mut cur_file = self.file_blocks[i];
        let mut cur_free = self.free_blocks[j];
        let mut cur_candidate = self.file_blocks[k];

        while cur_free.less_than(cur_candidate.start) {
            if cur_file.less_than(cur_free.start) {
                ret += i as isize * cur_file.sum();
                i += 1;
                cur_file = self.file_blocks[i];
            } else if cur_candidate.len() == cur_free.len() {
                ret += k as isize * cur_free.sum();
                j += 1;
                k -= 1;
                cur_free = self.free_blocks[j];
                cur_candidate = self.file_blocks[k];
            } else if cur_candidate.len() < cur_free.len() {
                let (left, right) =
                    cur_free.split_unchecked(cur_free.start + cur_candidate.len() as isize);
                ret += k as isize * left.sum();
                k -= 1;
                cur_free = right;
                cur_candidate = self.file_blocks[k];
            } else {
                ret += k as isize * cur_free.sum();
                cur_candidate.end -= cur_free.len() as isize;
                j += 1;
                cur_free = self.free_blocks[j];
            }
        }

        if i == k {
            ret += k as isize * cur_candidate.sum();
        }

        ret as i64
    }

    fn part2(&self) -> Result<i64, anyhow::Error> {
        let mut spans: FxHashMap<usize, BinaryHeap<Reverse<(isize, isize)>>> = FxHashMap::default();

        for block in &self.free_blocks {
            spans
                .entry(block.len())
                .and_modify(|x| x.push(Reverse((block.start, block.end))))
                .or_insert(BinaryHeap::from([Reverse((block.start, block.end))]));
        }

        let mut ret = 0;

        for (k, cur_candidate) in self.file_blocks.iter().enumerate().rev() {
            if let Some((min_k, _)) = spans
                .iter()
                .filter(|&(k, v)| *k >= cur_candidate.len() && !v.is_empty())
                .min_by_key(|&(_, v)| v.peek().map(|x| x.0).unwrap_or_default())
            {
                // safe to unwrap because we guarantee that this entry exists above
                let block = Interval::from(spans[min_k].peek().unwrap().0);
                if cur_candidate.less_than(block.start) {
                    ret += k as isize * cur_candidate.sum();
                    continue;
                }

                let (left, right) =
                    block.split_unchecked(block.start + cur_candidate.len() as isize);

                ret += k as isize * left.sum();

                spans.entry(*min_k).and_modify(|x| {
                    x.pop();
                });

                if !right.is_empty() {
                    spans
                        .entry(right.len())
                        .and_modify(|x| x.push(Reverse((right.start, right.end))))
                        .or_insert(BinaryHeap::from([Reverse((right.start, right.end))]));
                }
            } else {
                ret += k as isize * cur_candidate.sum();
            }
        }

        Ok(ret as i64)
    }
}

impl Problem for DiskFragmenter {
    const DAY: usize = 9;
    const TITLE: &'static str = "disk fragmenter";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.part1())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        self.part2()
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
        let solution = DiskFragmenter::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(6401092019345, 6431472344710));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = DiskFragmenter::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(1928, 2858));
    }
}
