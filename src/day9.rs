use aoc_runner_derive::{aoc, aoc_generator};

pub mod parser_naive {
    use std::error::Error;

    use nom::{character::complete::anychar, multi::many0, IResult};

    fn digit(input: &str) -> IResult<&str, u8> {
        let o = anychar(input);
        match o {
            Ok(x) => Ok((
                x.0,
                if let Some(x) = x.1.to_digit(10) {
                    x.try_into().unwrap()
                } else {
                    return Err(nom::Err::Error(nom::error::Error {
                        input: x.0,
                        code: nom::error::ErrorKind::IsNot,
                    }));
                },
            )),
            Err(e) => Err(e),
        }
    }

    pub fn parse(input: &str) -> IResult<&str, Vec<u8>> {
        many0(digit)(input)
    }
}
#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<u8> {
    parser_naive::parse(input).unwrap().1
}

#[aoc(day9, part1)]
fn part1(input: &[u8]) -> usize {
    let mut new_out: Vec<usize> = Vec::with_capacity(input.len());
    let mut front_pointer = 0;
    let mut back_pointer = input.len() - 1;
    let mut counter = input[back_pointer];
    loop {
        if front_pointer == back_pointer {
            for _ in 0..counter {
                new_out.push(front_pointer / 2);
            }
        } else if front_pointer % 2 == 0 {
            // we are on a value entry
            for _ in 0..input[front_pointer] {
                new_out.push(front_pointer / 2);
            }
        } else {
            // get the back number and start filling the space
            for _ in 0..input[front_pointer] {
                if counter == 0 {
                    back_pointer -= 2;
                    counter = input[back_pointer];
                }
                new_out.push(back_pointer / 2);
                counter -= 1;
            }
        }
        if front_pointer >= back_pointer {
            break;
        }
        front_pointer += 1;
    }
    println!("{new_out:?}");
    new_out
        .iter()
        .enumerate()
        .fold(0, |acc, (i, v)| acc + (i * { *v }))
}

#[aoc(day9, part2)]
fn part2(input: &[u8]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TEST_INPUT)), 1928);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
