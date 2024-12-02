use std::str::FromStr;

use aoc_plumbing::Problem;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
struct Report {
    levels: Vec<i32>,
}

impl FromStr for Report {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels = s
            .split(' ')
            .map(|x| x.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { levels })
    }
}

impl Report {
    fn is_safe(&self) -> bool {
        for i in 2..self.levels.len() {
            let a = self.levels[i - 2];
            let b = self.levels[i - 1];
            let c = self.levels[i];

            let ab = b - a;
            let bc = c - b;

            match (ab, bc) {
                (-3..=-1, -3..=-1) => (),
                (1..=3, 1..=3) => (),
                _ => return false,
            }
        }

        true
    }

    fn is_gradually_monotonic(a: i32, b: i32, c: i32) -> bool {
        let ab = b - a;
        let bc = c - b;

        ((-3..=-1).contains(&ab) && (-3..=-1).contains(&bc))
            || ((1..=3).contains(&ab) && (1..=3).contains(&bc))
    }

    fn is_safe_single_tolerance(&self) -> bool {
        let mut tolerance: Option<usize> = None;

        for i in 3..self.levels.len() {
            let a = self.levels[i - 3];
            let b = self.levels[i - 2];
            let c = self.levels[i - 1];
            let d = self.levels[i];

            let ab = b - a;
            let bc = c - b;
            let cd = d - c;

            match (ab, bc, cd) {
                (-3..=-1, -3..=-1, -3..=-1) => (),
                (1..=3, 1..=3, 1..=3) => (),
                _ => {
                    if Self::is_gradually_monotonic(b, c, d)
                        && tolerance.map_or(true, |x| x == i - 3)
                    {
                        tolerance = Some(i - 3);
                    } else if Self::is_gradually_monotonic(a, c, d)
                        && tolerance.map_or(true, |x| x == i - 2)
                    {
                        tolerance = Some(i - 2);
                    } else if Self::is_gradually_monotonic(a, b, d)
                        && tolerance.map_or(true, |x| x == i - 1)
                    {
                        tolerance = Some(i - 1);
                    } else if Self::is_gradually_monotonic(a, b, c)
                        && tolerance.map_or(true, |x| x == i)
                    {
                        tolerance = Some(i);
                    } else {
                        return false;
                    }
                }
            }
        }

        true
    }
}

#[derive(Debug, Clone)]
pub struct RedNosedReports {
    reports: Vec<Report>,
}

impl FromStr for RedNosedReports {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reports = s
            .lines()
            .map(Report::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { reports })
    }
}

impl RedNosedReports {
    fn total_safe(&self) -> usize {
        self.reports.iter().filter(|&x| x.is_safe()).count()
    }

    fn total_safe_single_tolerance(&self) -> usize {
        self.reports
            .iter()
            .filter(|&x| x.is_safe_single_tolerance())
            .count()
    }
}

impl Problem for RedNosedReports {
    const DAY: usize = 2;
    const TITLE: &'static str = "red nosed reports";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.total_safe())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.total_safe_single_tolerance())
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
        let solution = RedNosedReports::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(564, 604));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = RedNosedReports::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(2, 4));
    }
}
