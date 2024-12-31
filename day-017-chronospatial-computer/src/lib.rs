use std::str::FromStr;

use anyhow::{anyhow, bail};
use aoc_plumbing::Problem;
use itertools::join;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Op {
    Adv(Operand),
    Bxl(Operand),
    Bst(Operand),
    Jnz(Operand),
    Bxc(Operand),
    Out(Operand),
    Bdv(Operand),
    Cdv(Operand),
}

impl TryFrom<(u8, u8)> for Op {
    type Error = anyhow::Error;

    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error> {
        let ret = match value.0 {
            0 => Self::Adv(value.1.try_into()?),
            1 => Self::Bxl(Operand::Literal(value.1)),
            2 => Self::Bst(value.1.try_into()?),
            3 => Self::Jnz(Operand::Literal(value.1)),
            4 => Self::Bxc(Operand::Literal(value.1)),
            5 => Self::Out(value.1.try_into()?),
            6 => Self::Bdv(value.1.try_into()?),
            7 => Self::Cdv(value.1.try_into()?),
            _ => bail!("invalid op"),
        };

        Ok(ret)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Operand {
    Literal(u8),
    RegisterA,
    RegisterB,
    RegisterC,
}

impl TryFrom<u8> for Operand {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let ret = match value {
            0..=3 => Self::Literal(value),
            4 => Self::RegisterA,
            5 => Self::RegisterB,
            6 => Self::RegisterC,
            _ => bail!("invalid operand"),
        };

        Ok(ret)
    }
}

#[derive(Debug, Clone)]
pub struct ChronospatialComputer {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    ops: Vec<Op>,
    raw: Vec<u8>,
}

impl FromStr for ChronospatialComputer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let register_a = lines
            .next()
            .and_then(|l| l[12..l.len()].parse().ok())
            .ok_or_else(|| anyhow!("could not parse register a"))?;
        let register_b = lines
            .next()
            .and_then(|l| l[12..l.len()].parse().ok())
            .ok_or_else(|| anyhow!("could not parse register b"))?;
        let register_c = lines
            .next()
            .and_then(|l| l[12..l.len()].parse().ok())
            .ok_or_else(|| anyhow!("could not parse register c"))?;

        lines.next();

        let raw = lines
            .next()
            .and_then(|l| {
                l[9..l.len()]
                    .split(',')
                    .map(|x| x.parse::<u8>())
                    .collect::<Result<Vec<_>, _>>()
                    .ok()
            })
            .ok_or_else(|| anyhow!("could not parse codes"))?;

        let ops = raw
            .chunks(2)
            .map(|x| Op::try_from((x[0], x[1])))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            register_a,
            register_b,
            register_c,
            ops,
            raw,
        })
    }
}

impl ChronospatialComputer {
    fn simulate(&mut self) -> String {
        let mut head = 0;
        let mut ret = Vec::default();

        while head < self.ops.len() {
            let op = &self.ops[head];
            match op {
                Op::Adv(x) => self.register_a /= 2_u64.pow(self.value(x) as u32),
                Op::Bxl(x) => self.register_b ^= self.value(x),
                Op::Bst(x) => self.register_b = self.value(x) % 8,
                Op::Jnz(x) => match self.register_a {
                    0 => (),
                    _ => {
                        head = self.value(x) as usize / 2;
                        continue;
                    }
                },
                Op::Bxc(_) => self.register_b ^= self.register_c,
                Op::Out(x) => ret.push(self.value(x) % 8),
                Op::Bdv(x) => self.register_b = self.register_a / 2_u64.pow(self.value(x) as u32),
                Op::Cdv(x) => self.register_c = self.register_a / 2_u64.pow(self.value(x) as u32),
            }

            head += 1;
        }

        join(ret, ",")
    }

    fn simulate_one_cycle(&mut self) -> Option<u8> {
        let mut head = 0;
        let mut ret = None;

        while head < self.ops.len() {
            let op = &self.ops[head];
            match op {
                Op::Adv(x) => self.register_a /= 2_u64.pow(self.value(x) as u32),
                Op::Bxl(x) => self.register_b ^= self.value(x),
                Op::Bst(x) => self.register_b = self.value(x) % 8,
                Op::Jnz(_) => match self.register_a {
                    0 => (),
                    _ => break,
                },
                Op::Bxc(_) => self.register_b ^= self.register_c,
                Op::Out(x) => ret = Some((self.value(x) % 8) as u8),
                Op::Bdv(x) => self.register_b = self.register_a / 2_u64.pow(self.value(x) as u32),
                Op::Cdv(x) => self.register_c = self.register_a / 2_u64.pow(self.value(x) as u32),
            }

            head += 1;
        }

        ret
    }

    fn find_register(&mut self) -> Option<u64> {
        self.find_register_helper(0, &self.raw.clone())
    }

    fn find_register_helper(&mut self, acc: u64, desired: &[u8]) -> Option<u64> {
        if desired.is_empty() {
            return Some(acc);
        }

        let code = desired[desired.len() - 1];

        let range = if acc == 0 {
            0o1..0o10
        } else {
            (acc * 0o10)..(acc * 0o10 + 0o10)
        };

        for i in range {
            self.register_a = i;
            self.register_b = 0;
            self.register_c = 0;

            if self.simulate_one_cycle().is_some_and(|x| x == code) {
                let result = self.find_register_helper(i, &desired[0..desired.len() - 1]);
                if result.is_some() {
                    return result;
                }
            }
        }

        None
    }

    fn value(&self, operand: &Operand) -> u64 {
        match operand {
            Operand::Literal(x) => *x as u64,
            Operand::RegisterA => self.register_a,
            Operand::RegisterB => self.register_b,
            Operand::RegisterC => self.register_c,
        }
    }
}

impl Problem for ChronospatialComputer {
    const DAY: usize = 17;
    const TITLE: &'static str = "chronospatial computer";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = String;
    type P2 = u64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.simulate())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        self.find_register()
            .ok_or_else(|| anyhow!("could not find value"))
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
        let solution = ChronospatialComputer::solve(&input).unwrap();
        assert_eq!(
            solution,
            Solution::new("1,5,3,0,2,5,2,5,3".to_owned(), 108107566389757)
        );
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let mut problem = ChronospatialComputer::from_str(&input).expect("could not parse input");
        assert_eq!(
            problem.part_one().unwrap(),
            "4,6,3,5,6,3,5,2,1,0".to_owned()
        );
    }

    #[test]
    fn example2() {
        let input = std::fs::read_to_string("example2.txt").expect("Unable to load input");
        let solution = ChronospatialComputer::solve(&input).unwrap();
        assert_eq!(solution, Solution::new("5,7,3,0".to_owned(), 117440));
    }
}
