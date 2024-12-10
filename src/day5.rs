use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
pub type Map = HashMap<u32, HashSet<u32>>;
pub struct NaiveData {
    checker_map: Map,
    print_runs: Vec<Vec<u32>>,
}

pub mod naive_parser {
    use std::collections::{HashMap, HashSet};

    use nom::{
        bytes::complete::tag,
        character::complete::{newline, u32},
        multi::separated_list0,
        sequence::separated_pair,
        IResult,
    };

    use super::{Map, NaiveData};

    fn checker_line(input: &str) -> IResult<&str, (u32, u32)> {
        separated_pair(u32, tag("|"), u32)(input)
    }

    fn runs_line(input: &str) -> IResult<&str, Vec<u32>> {
        separated_list0(tag(","), u32)(input)
    }

    fn runs(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
        separated_list0(newline, runs_line)(input)
    }

    fn checker(input: &str) -> IResult<&str, Map> {
        let mut out: Map = HashMap::new();
        let (ret_str, vec_to_unpack) = match separated_list0(newline, checker_line)(input) {
            Ok(o) => o,
            Err(e) => return Err(e),
        };
        for v in &vec_to_unpack {
            out.entry(v.0)
                .and_modify(|l| {
                    l.insert(v.1);
                })
                .or_insert(HashSet::from([v.1]));
        }
        Ok((ret_str, out))
    }

    /// simple input parser
    ///
    /// # Panics
    ///
    /// Panics if The input doesn't match the problem input pattern
    pub fn parse(input: &str) -> NaiveData {
        let (checker_map, print_runs) = separated_pair(checker, tag("\n\n"), runs)(input)
            .expect("Input didn't match the input pattern")
            .1;
        NaiveData {
            checker_map,
            print_runs,
        }
    }
}

#[aoc_generator(day5, naive)]
fn parse(input: &str) -> NaiveData {
    naive_parser::parse(input)
}

#[aoc(day5, part1, naive)]
fn part1(input: &NaiveData) -> u32 {
    let mut out = 0;
    input.print_runs.iter().for_each(|run| {
        let mut seen: HashSet<u32> = HashSet::new();
        let mut valid = true;
        for elem in run {
            if let Some(m) = input.checker_map.get(elem) {
                if m.intersection(&seen).count() != 0 {
                    valid = false;
                }
            }
            seen.insert(*elem);
        }
        if valid {
            out += run[run.len() / 2];
        }
    });
    out
}

#[aoc(day5, part2)]
fn part2(_input: &NaiveData) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TEST_INPUT)), 143);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
