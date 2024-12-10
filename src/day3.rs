use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]

pub enum INST {
    Mul(u32, u32),
    Do,
    Dont,
}

pub mod parser {
    use super::INST;
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while},
        character::complete::anychar,
        combinator::map_res,
        multi::{many0, many_till},
        sequence::{delimited, preceded, separated_pair},
        IResult,
    };

    pub fn test_many(input: &str) -> IResult<&str, Vec<&str>> {
        many0(tag("a"))(input)
    }
    pub fn num(input: &str) -> IResult<&str, u32> {
        map_res(take_while(|c: char| c.is_ascii_digit()), |raw: &str| {
            raw.parse::<u32>()
        })(input)
    }

    fn numbers(input: &str) -> IResult<&str, (u32, u32)> {
        separated_pair(num, tag(","), num)(input)
    }
    fn bracket(input: &str) -> IResult<&str, (u32, u32)> {
        delimited(tag("("), numbers, tag(")"))(input)
    }

    fn mult(input: &str) -> IResult<&str, INST> {
        match preceded(tag("mul"), bracket)(input) {
            Ok((a, (b, c))) => Ok((a, INST::Mul(b, c))),
            Err(e) => Err(e),
        }
    }

    fn do_inst(input: &str) -> IResult<&str, INST> {
        match tag("do()")(input) {
            Ok((a, _)) => Ok((a, INST::Do)),
            Err(e) => Err(e),
        }
    }

    fn dont(input: &str) -> IResult<&str, INST> {
        match tag("don't()")(input) {
            Ok((a, _)) => Ok((a, INST::Dont)),
            Err(e) => Err(e),
        }
    }

    fn inst(input: &str) -> IResult<&str, INST> {
        alt((mult, do_inst, dont))(input)
    }
    fn rubbish_till(input: &str) -> IResult<&str, INST> {
        match many_till(anychar, inst)(input) {
            Ok(m) => Ok((m.0, m.1 .1)),
            Err(e) => Err(e),
        }
    }

    pub fn parse(input: &str) -> IResult<&str, Vec<INST>> {
        many0(rubbish_till)(input)
    }
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<INST> {
    parser::parse(input).unwrap().1
}

#[aoc(day3, part1)]
fn part1(input: &[INST]) -> u32 {
    input
        .iter()
        .filter_map(|x| match x {
            INST::Do => None,
            INST::Dont => None,
            INST::Mul(l, r) => Some(l * r),
        })
        .fold(0, |acc, x| x + acc)
}

#[aoc(day3, part2)]
fn part2(input: &[INST]) -> u32 {
    let mut flag = true;
    let mut counter = 0;
    for inst in input {
        match inst {
            INST::Mul(l, r) => {
                if flag {
                    counter += l * r
                }
            }
            INST::Do => flag = true,
            INST::Dont => flag = false,
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    static TEST_INPUT_2: &str =
        "mul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TEST_INPUT)), 161);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TEST_INPUT_2)), 48);
    }
}
