use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

mod parser {
    use std::char;

    use nom::{
        character::complete::{newline, none_of},
        multi::{many0, separated_list0},
        IResult,
    };
    fn parse_line(input: &str) -> IResult<&str, Vec<char>> {
        many0(none_of("\n"))(input)
    }

    // this is a slow parser that outputs a *bad* multidimentional Vec, there are better ways of doing this
    // and one of the first optimisations would be flattening and holding a second Vec which gives the indexes at which we split the first vec
    pub fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
        separated_list0(newline, parse_line)(input)
    }
}
#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Vec<char>> {
    parser::parse(input).unwrap().1
}

#[aoc(day4, part1)]
fn part1(input: &[Vec<char>]) -> usize {
    // find all X's
    // check each X's surrounding characters for the XMAS characters
    let masks: [(i32, i32); 8] = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    let mut character_masks: Vec<HashMap<char, (i32, i32)>> = Vec::new();
    for mask in masks {
        let mut new_map = HashMap::new();
        new_map.insert('M', mask);
        new_map.insert('A', (mask.0 * 2, mask.1 * 2));
        new_map.insert('S', (mask.0 * 3, mask.1 * 3));
        character_masks.push(new_map);
    }
    let mut counter = 0;
    let max_width = input[0].len() as i32 - 1;
    let max_height = input.len() as i32 - 1;
    for (column, l) in input.iter().enumerate() {
        for (row, c) in l.iter().enumerate() {
            if *c == 'X' {
                for direction in &character_masks {
                    let mut good = true;
                    for character in direction {
                        // check bounds of new value
                        let to_check_row = character.1 .1 + row as i32;
                        let to_check_column = character.1 .0 + column as i32;
                        if to_check_row < 0
                            || to_check_row > max_width
                            || to_check_column < 0
                            || to_check_column > max_height
                        {
                            good = false;
                            break;
                        }
                        if input[to_check_column as usize][to_check_row as usize] != *character.0 {
                            good = false;
                            break;
                        }
                    }
                    if good {
                        counter += 1;
                    }
                }
                // we have an X check the surrounding characters
            }
        }
    }
    counter
}

#[aoc(day4, part2)]
fn part2(_input: &[Vec<char>]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TEST_INPUT)), 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
