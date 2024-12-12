use core::panic;
use std::ops::BitAnd;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Ops {
    Add,
    Mul,
}

mod parser {
    use nom::bytes::complete::tag;
    use nom::character::complete::u64;
    use nom::multi::separated_list0;
    use nom::IResult;
    use nom::{character::complete::newline, sequence::separated_pair};

    use super::{Equation, Ops};

    fn numbers_list(input: &str) -> IResult<&str, Vec<u64>> {
        separated_list0(tag(" "), u64)(input)
    }

    fn equation(input: &str) -> IResult<&str, (u64, Vec<u64>)> {
        separated_pair(u64, tag(": "), numbers_list)(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Vec<Equation>> {
        let o = separated_list0(newline, equation)(input);
        match o {
            Ok(oo) => Ok((
                oo.0,
                oo.1.iter()
                    .map(|v| Equation {
                        goal: v.0,
                        comp: v.1.clone(),
                        ops: vec![Ops::Add; v.1.len() - 1],
                        ops_num: 0,
                    })
                    .collect(),
            )),
            Err(e) => Err(e),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Equation {
    goal: u64,
    comp: Vec<u64>,
    ops: Vec<Ops>,
    ops_num: u64,
}
#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<Equation> {
    match parser::parse(input) {
        Ok(v) => v.1,
        Err(e) => panic!("{}", e),
    }
}

#[aoc(day7, part1)]
fn part1(input: &Vec<Equation>) -> u64 {
    let mut input = input.clone();
    let mut out: u64 = 0;
    for equ in &mut input {
        // check if function is valid
        let mut leave = false;
        loop {
            if equ.check_valid() {
                out += equ.goal;
                break;
            }
            if leave {
                break;
            }

            leave = equ.update_ops();
        }
    }
    out
}

impl Equation {
    fn check_valid(&self) -> bool {
        let mut out = self.comp[0];
        for (i, v) in self.comp.iter().skip(1).enumerate() {
            if i >= self.ops.len() {
                break;
            }
            out = match self.ops.get(i) {
                Some(Ops::Add) => out + v,
                Some(Ops::Mul) => out * v,
                None => return out == self.goal,
            }
        }
        // println!(
        //     "Checking Valid | {0:?} against {1:?} for {2:?}, goal: {3:?}",
        //     self.comp, self.ops, out, self.goal
        // );
        out == self.goal
    }
    fn update_ops(&mut self) -> bool {
        // println!("Update ops on: {:?}", self.comp);
        let mut checker: u64 = 1;
        let mut counter = 0;
        self.ops_num += 1;
        let mut add_checker = 0;
        loop {
            self.ops[counter] = match checker.bitand(self.ops_num) {
                0 => {
                    add_checker = 1;
                    Ops::Add
                }
                _ => Ops::Mul,
            };
            checker <<= 1;
            counter += 1;
            if counter >= self.ops.len() {
                break;
            }
        }
        matches!(add_checker, 0)
    }
}

#[aoc(day7, part2)]
fn part2(input: &[Equation]) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
    #[test]
    fn test_update_ops() {
        let mut test_inp = Equation {
            goal: 0,
            comp: vec![0; 4],
            ops: vec![Ops::Add; 4],
            ops_num: 29,
        };
        test_inp.update_ops();
        test_inp.update_ops();
        assert_eq!(
            test_inp.ops,
            Vec::from([Ops::Mul, Ops::Mul, Ops::Mul, Ops::Mul])
        );
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TEST_INPUT)), 3749);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), 0);
    }
}
