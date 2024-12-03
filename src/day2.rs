use std::{cmp::Ordering, ops::RangeInclusive};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Report {
    levels: Vec<i32>,
}

const DESIRED_DIFF: RangeInclusive<i32> = 1..=3;

impl Report {
    fn check_report(&self) -> bool {
        //first check
        let ordering = self.levels[0].cmp(&self.levels[1]);
        if ordering == Ordering::Equal {
            return false;
        }

        let mut left = self.levels[0];
        for (_ri, right) in self.levels.iter().enumerate().skip(1) {
            if left.cmp(right) != ordering
                || !DESIRED_DIFF.contains(&(left.abs_diff(*right) as i32))
            {
                return false;
            }
            left = *right;
        }
        true
    }

    fn check_report_dampner(&self) -> bool {
        //first check
        let ordering = self.levels[0].cmp(&self.levels[1]);
        if ordering == Ordering::Equal {
            // try without the first entry
            let mut tmp = self.levels.clone();
            let mut tmp2 = self.levels.clone();
            tmp2.remove(1);
            tmp.remove(0);

            return (Report { levels: tmp }).check_report()
                || (Report { levels: tmp2 }).check_report();
            // we need to now update the ordering to be new one

            // try without the second entry
        }

        let mut left = self.levels[0];
        for (_ri, right) in self.levels.iter().enumerate().skip(1) {
            if left.cmp(right) != ordering
                || !DESIRED_DIFF.contains(&(left.abs_diff(*right) as i32))
            {
                let mut tmp = self.levels.clone();
                let mut tmp2 = self.levels.clone();
                tmp.remove(_ri - 1);
                tmp2.remove(_ri);
                return (Report { levels: tmp }).check_report()
                    || (Report { levels: tmp2 }).check_report();
            }
            left = *right;
        }
        true
    }
}

pub mod parser {
    use nom::{
        bytes::complete::{tag, take_while},
        character::complete::line_ending,
        combinator::map_res,
        multi::separated_list0,
        IResult,
    };

    use super::Report;

    pub fn num(input: &str) -> IResult<&str, i32> {
        map_res(take_while(|c: char| c.is_ascii_digit()), |raw: &str| {
            raw.parse::<i32>()
        })(input)
    }

    pub fn parse_report(input: &str) -> IResult<&str, Report> {
        let out: (&str, Vec<i32>) = separated_list0(tag(" "), num)(input).unwrap();
        Ok((out.0, Report { levels: out.1 }))
    }

    pub fn parse(input: &str) -> IResult<&str, Vec<Report>> {
        separated_list0(line_ending, parse_report)(input)
    }

    pub fn parse2(input: &str) -> Vec<Report> {
        input
            .lines()
            .map(|l| Report {
                levels: l.split(' ').map(|num| num.parse().unwrap()).collect(),
            })
            .collect()
    }
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Report> {
    parser::parse(input).unwrap().1
}

#[aoc(day2, part1)]
fn part1(input: &[Report]) -> usize {
    input.iter().filter(|x| x.check_report()).count()

    // loop over all the reports and check
}

#[aoc(day2, part2)]
fn part2(input: &[Report]) -> usize {
    input.iter().filter(|x| x.check_report_dampner()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TEST_INPUT)), 2);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TEST_INPUT)), 4);
    }

    //     #[test]
    //     fn part2_example() {
    //         assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    //     }
    //
}
