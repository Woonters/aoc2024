use aoc_runner_derive::{aoc, aoc_generator};

pub struct Report {
    levels: Vec<u32>,
}

pub mod parser {
    use nom::{
        character::complete::{i32, multispace0, newline},
        multi::separated_list0,
        IResult,
    };

    use super::Report;

    pub fn parse_report(input: &str) -> IResult<&str, Report> {
        Report{separated_list0(multispace0, i32)}
    }

    pub fn parse(input: &str) -> IResult<&str, Vec<Report>> {
        separated_list0(newline, parse_report)(input)
    }
}
#[aoc_generator(day2)]
fn parse(input: &str) -> String {
    todo!()
}

#[aoc(day2, part1)]
fn part1(input: &str) -> String {
    todo!()
}

#[aoc(day2, part2)]
fn part2(input: &str) -> String {
    todo!()
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
        assert_eq!(part1(&parse("<EXAMPLE>")), "<RESULT>");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
