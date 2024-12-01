use std::{collections::HashMap, iter::zip};

use parser::parse;

pub mod parser {
    use nom::bytes::complete::tag;
    use nom::character::complete::i32;
    use nom::character::complete::multispace0;
    use nom::multi::separated_list0;
    use nom::sequence::separated_pair;
    use nom::IResult;

    pub fn parse_pair(input: &str) -> IResult<&str, (i32, i32)> {
        separated_pair(i32, multispace0, i32)(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
        separated_list0(tag("\n"), parse_pair)(input)
    }
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<i32>, Vec<i32>) {
    let p = parse(input).unwrap().1;
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = p.iter().cloned().unzip();
    left.sort_unstable();
    right.sort_unstable();
    (left, right)
}

#[aoc(day1, part1)]
pub fn solver_p1(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    zip(input.0.iter(), input.1.iter()).fold(0, |acc, (x, y)| acc + ((x - y).abs()))
}

#[aoc(day1, part2)]
pub fn solver_p2(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    let mut pointer_index = 0;
    let mut cache = HashMap::new();
    let mut total = 0;
    input.0.iter().for_each(|i| {
        if let Some(v) = cache.get(i) {
            total += v
        } else {
            let mut counter = 0;
            while input.1[pointer_index] < *i {
                pointer_index += 1;
            }
            while input.1[pointer_index] == *i {
                pointer_index += 1;
                counter += i;
            }
            cache.insert(i, counter);
            total += counter;
        }
    });
    total
}

#[cfg(test)]
mod tests {
    use parser::{parse, parse_pair};

    use super::*;

    static TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn total_nom_parse() {
        let (_, parsed) = parse(TEST_INPUT).unwrap();
        assert_eq!(
            parsed,
            Vec::from([(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)])
        )
    }
    #[test]
    fn part_1() {
        assert_eq!(solver_p1(&input_generator(TEST_INPUT)), 11)
    }

    #[test]
    fn test_parse_line() {
        let (_, output) = parse_pair("123   456").unwrap();

        assert_eq!(output, (123, 456))
    }

    #[test]
    fn part_2() {
        assert_eq!(solver_p2(&input_generator(TEST_INPUT)), 31)
    }
}
