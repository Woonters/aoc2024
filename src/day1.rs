use std::{
    collections::{BTreeMap, HashMap},
    iter::zip,
};

use parser::parse;
use rayon::prelude::*;
use wide::i32x8;

type i32s = i32x8;

pub mod parser {
    // thoughts on nom:
    // I like the idea of splitting the parser into it's owm mod, it's a part of rust I should get better at and do more since it increases the
    // quality of code. Getting your head around the parser takes a little time but there are some niceties to it and it is certianly faster than my old
    // naieve approach. in theory if I were writing a larger library there would be some good benifits, the design is meant to encourage easy readability
    // for a large portion of the parsing (especially for big parsing tasks)
    use nom::bytes::complete::take_while;
    use nom::character::complete::multispace0;
    use nom::character::complete::{i32, newline};
    use nom::combinator::map_res;
    use nom::multi::separated_list0;
    use nom::sequence::separated_pair;
    use nom::IResult;

    // Instead of using the i32 nom parser you can use this custom one, I found it had a slight degredation on speed but was slightly more consistent
    // not really worth it
    pub fn num(input: &str) -> IResult<&str, i32> {
        map_res(take_while(|c: char| c.is_digit(10)), |raw| {
            i32::from_str_radix(raw, 10)
        })(input)
    }

    pub fn parse_pair(input: &str) -> IResult<&str, (i32, i32)> {
        separated_pair(i32, multispace0, i32)(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
        separated_list0(newline, parse_pair)(input)
    }
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<i32>, Vec<i32>) {
    let p = parse(input).unwrap().1;
    let mut left = Vec::with_capacity(p.len());
    let mut right = Vec::with_capacity(p.len());
    p.par_iter().cloned().unzip_into_vecs(&mut left, &mut right);
    left.sort_unstable();
    right.sort_unstable();
    (left, right)
}

#[aoc(day1, part1)]
pub fn solver_p1(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    zip(input.0.iter(), input.1.iter()).fold(0, |acc, (x, y)| acc + ((x - y).abs()))
}

#[aoc(day1, part1, simd)]
pub fn solver_simd_p1(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    let lefts: Vec<i32s> = input
        .0
        .chunks_exact(8)
        .map(|c| i32s::new(c.try_into().unwrap()))
        .collect();
    let rights: Vec<i32s> = input
        .1
        .chunks_exact(8)
        .map(|c| i32s::new(c.try_into().unwrap()))
        .collect();
    zip(lefts.iter(), rights.iter())
        .fold(i32s::splat(0), |acc, (&x, &y)| (x - y).abs() + acc)
        .reduce_add()
}

#[aoc(day1, part1, rayon_and_simd)]
pub fn solver_rayon_and_simd_p1(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    let lefts: Vec<i32s> = input
        .0
        .chunks_exact(8)
        .map(|c| i32s::new(c.try_into().unwrap()))
        .collect();
    let rights: Vec<i32s> = input
        .1
        .chunks_exact(8)
        .map(|c| i32s::new(c.try_into().unwrap()))
        .collect();
    lefts
        .par_iter()
        .zip(rights.par_iter())
        .fold(|| i32s::splat(0), |acc, (&x, &y)| acc + (x - y).abs())
        .sum::<i32s>()
        .reduce_add()
}

#[aoc(day1, part1, rayon)]
pub fn solver_rayon_p1(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    input
        .0
        .par_iter()
        .zip(input.1.par_iter())
        .fold(|| 0_i32, |acc, (x, y)| acc + ((x - y).abs()))
        .sum::<i32>()
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

// this is my re-implementation of Laura's solution whilst we don't have the server running, I mainly wanted to see how it compared to mine / understand her thought process
// technically this solution would be better with hers for a number of reasons (the main one being she wouldn't have the sort operation in her generator for this function)
#[aoc(day01, part2, laura)]
fn solve_part2(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    let mut left = BTreeMap::new();
    let mut right = BTreeMap::new();
    for i in 0..input.0.len() {
        left.entry(input.0.get(i))
            .and_modify(|count| *count += 1)
            .or_insert(1);
        right
            .entry(input.1.get(i))
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    left.into_iter()
        .filter_map(|(num, lc)| right.get(&num).map(|rc| (lc * rc) * num.unwrap_or(&0)))
        .sum::<i32>()
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
