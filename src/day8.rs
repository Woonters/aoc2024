use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use nom::Slice;

type InpMap = HashMap<char, Vec<(usize, usize)>>;

pub mod parser {
    use nom::{
        character::complete::{anychar, newline, none_of},
        multi::{many0, separated_list0},
        IResult,
    };

    use super::InpMap;

    fn line_parser(input: &str) -> IResult<&str, Vec<char>> {
        many0(none_of("\n"))(input)
    }

    /// Input parser.
    ///
    /// # Errors
    ///
    /// This function will return an error if the input doesn't match the problem spec
    pub fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
        separated_list0(newline, line_parser)(input)
    }
}

#[aoc_generator(day8)]
fn parse(input: &str) -> (InpMap, usize, usize) {
    let parsed_str = parser::parse(input).unwrap().1;
    let mut out: InpMap = InpMap::default();
    parsed_str.iter().enumerate().for_each(|(row, line)| {
        line.iter().enumerate().for_each(|(column, character)| {
            if *character != '.' {
                out.entry(*character)
                    .or_insert(Vec::from([]))
                    .push((row, column));
            }
        });
    });
    let m = input.lines().last().unwrap().len();
    (out, m, input.lines().count())
}

#[aoc(day8, part1)]
fn part1((input, max_columns, max_rows): &(InpMap, usize, usize)) -> u64 {
    let mut out: HashSet<(usize, usize)> = HashSet::new();
    for ch in input {
        check_characters_antinodes(ch.1, *max_columns, *max_rows, &mut out);
    }

    out.len() as u64
}

fn check_characters_antinodes(
    nodes: &[(usize, usize)],
    max_columns: usize,
    max_rows: usize,
    antinode_set: &mut HashSet<(usize, usize)>,
) {
    // for each node generate the valid antinodes
    let get_distance = |left: (usize, usize), right: (usize, usize)| {
        (
            2 * (left.0 as i64 - right.0 as i64),
            2 * (left.1 as i64 - right.1 as i64),
        )
    };
    let check_inbounds = |position: (i64, i64), max_columns: usize, max_rows: usize| {
        if position.0 >= (max_rows as i64)
            || position.0 < 0
            || position.1 >= (max_columns as i64)
            || position.1 < 0
        {
            return false;
        }
        true
    };
    for (index, left_node) in nodes.iter().enumerate() {
        // check this value against the tail of nodes from this
        for right_node in nodes.iter().skip(index + 1) {
            let distance = get_distance(*left_node, *right_node);
            // add distance
            let add: (i64, i64) = (
                (right_node.0 as i64) + distance.0,
                (right_node.1 as i64) + distance.1,
            );
            // minus distance
            let sub: (i64, i64) = (
                (left_node.0 as i64) - distance.0,
                (left_node.1 as i64) - distance.1,
            );
            if check_inbounds(add, max_columns, max_rows) {
                antinode_set.insert((add.0 as usize, add.1 as usize));
            }
            if check_inbounds(sub, max_columns, max_rows) {
                antinode_set.insert((sub.0 as usize, sub.1 as usize));
            }
        }
    }
}

#[aoc(day8, part2)]
fn part2((input, max_columns, max_rows): &(InpMap, usize, usize)) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TEST_INPUT)), 14);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
