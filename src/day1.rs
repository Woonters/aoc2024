use std::{collections::HashMap, iter::zip, path::absolute};

pub struct Lists {
    left: Vec<i32>,
    right: Vec<i32>,
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    input.lines().for_each(|x| {
        let y: Vec<i32> = x.split("   ").map(|d| d.parse::<i32>().unwrap()).collect();
        left.push(y[0]);
        right.push(y[1]);
    });
    left.sort();
    right.sort();
    (left, right)
}

#[aoc(day1, part1)]
pub fn solver_p1(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    zip(input.0.iter(), input.1.iter())
        .map(|(x, y)| (x - y).abs())
        .sum()
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
    use super::*;

    static TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part_1() {
        assert_eq!(solver_p1(&input_generator(TEST_INPUT)), 11)
    }

    #[test]
    fn part_2() {
        assert_eq!(solver_p2(&input_generator(TEST_INPUT)), 31)
    }
}
