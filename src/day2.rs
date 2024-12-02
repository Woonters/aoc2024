use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn check_report(&self) -> bool {
        //first check
        if self.levels[0] == self.levels[1] {
            return false
        }
        let inc = self.levels[0] - self.levels[1] < 0;

        for i in 2..self.levels.len() {
            match self.levels[i-1] - self.levels[i] {
                4.. => {return false}
                1..=3 => {
                    if inc {
                        return false;
                    }
                }
                0 => {return false;},
                -3..=-1 => {
                    if inc == false {
                        return false;
                    }
                }
                ..=-4 => {
                    return false;
                }
                }
            };
        return true
    }
}

pub mod parser {
    use nom::{
         bytes::complete::{tag, take_while}, character::complete::line_ending, combinator::map_res, multi::{separated_list0, separated_list1}, IResult
    };

    use super::Report;

    pub fn num(input: &str) -> IResult<&str, i32> {
        map_res(take_while(|c: char| c.is_digit(10)), |raw: &str| {
            raw.parse::<i32>()
        })(input)
    }


    pub fn parse_report(input: &str) -> IResult<&str, Report> {
        let out: (&str, Vec<i32>) = separated_list1(tag(" "), num)(input).unwrap();
        Ok((out.0, Report{levels:out.1}))
    }

    pub fn parse(input: &str) -> IResult<&str, Vec<Report>> {
        separated_list0(line_ending, parse_report)(input)
    }
}
#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Report> {
    let reports: Vec<Report> = parser::parse(input).unwrap().1;
    reports
}

#[aoc(day2, part1)]
fn part1(input: &Vec<Report>) -> usize {
    input.iter().filter(|x| x.check_report()).count()

    // loop over all the reports and check 
}



// #[aoc(day2, part2)]
// fn part2(input: &str) -> i32 {
//     todo!()
// }

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
        assert_eq!(part1(&parse(TEST_INPUT)), 2);
    }

//     #[test]
//     fn part2_example() {
//         assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
//     }
// 
}
