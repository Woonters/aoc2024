use aoc_runner_derive::{aoc, aoc_generator};

pub struct NaiveInput {
    map: Vec<Vec<u32>>,
}
pub mod naive_parser {
    use nom::{
        branch::alt,
        character::complete::{char, line_ending},
        multi::{many0, separated_list0},
        IResult,
    };

    use super::NaiveInput;
    fn character(input: &str) -> IResult<&str, u32> {
        let o = alt((char('.'), char('#'), char('^')))(input);
        let (s, c) = match o {
            Ok(oo) => oo,
            Err(e) => return Err(e),
        };
        let ret = match c {
            '.' => 0,
            '#' => 3,
            '^' => 2,
            _ => u32::MAX,
        };
        Ok((s, ret))
    }

    fn map_line(input: &str) -> IResult<&str, Vec<u32>> {
        many0(character)(input)
    }

    pub fn parse(input: &str) -> IResult<&str, NaiveInput> {
        let out = separated_list0(line_ending, map_line)(input).unwrap();
        Ok((out.0, NaiveInput { map: out.1 }))
    }
}
#[aoc_generator(day6)]
fn parse(input: &str) -> NaiveInput {
    naive_parser::parse(input).unwrap().1
}

enum Direction {
    North,
    East,
    South,
    West,
}

#[aoc(day6, part1)]
fn part1(input: &NaiveInput) -> u32 {
    let mut local_map: Vec<Vec<u32>> = vec![vec![0; input.map[0].len()]; input.map.len()];
    let mut row = input.map.iter().position(|l| l.contains(&2)).unwrap();
    let mut column = input.map[row].iter().position(|l| l == &2).unwrap();
    let mut direction = Direction::North;
    let max_row = input.map.len();
    let max_column = input.map[0].len();
    loop {
        local_map[row][column] = 1;
        if let Direction::North = direction {
            let Some(new_row) = row.checked_sub(1) else {
                break;
            };
            row = new_row;
        } else if let Direction::East = direction {
            let new_column = column + 1;
            column = new_column;
        } else if let Direction::South = direction {
            let new_row = row + 1;
            row = new_row;
        } else if let Direction::West = direction {
            let Some(new_column) = column.checked_sub(1) else {
                break;
            };
            column = new_column;
        }
        if row >= max_row || column >= max_column {
            break;
        }
        if input.map[row][column] == 3 {
            direction = match direction {
                Direction::North => {
                    row += 1;
                    Direction::East
                }
                Direction::East => {
                    column -= 1;
                    Direction::South
                }
                Direction::South => {
                    row -= 1;
                    Direction::West
                }
                Direction::West => {
                    column += 1;
                    Direction::North
                }
            };
        }
    }

    local_map.iter().map(|line| line.iter().sum::<u32>()).sum()
}

#[aoc(day6, part2)]
fn part2(_input: &NaiveInput) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TEST_INPUT)), 41);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
