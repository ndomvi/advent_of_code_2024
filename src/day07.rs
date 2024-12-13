use std::error::Error;

use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;

type ParsedInput = Vec<(u64, Vec<u64>)>;

#[aoc_generator(day07)]
fn parse(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|l| {
            let split = l.split_once(":").unwrap();
            (
                split.0.parse().unwrap(),
                split
                    .1
                    .split_ascii_whitespace()
                    .map(|i| i.parse().unwrap())
                    .collect(),
            )
        })
        .collect::<Vec<_>>()
}

fn calculate(goal: u64, cur: u64, idx: usize, values: &Vec<u64>) -> bool {
    if idx == values.len() {
        return cur == goal;
    }

    if calculate(goal, cur + values[idx], idx + 1, values) {
        true
    } else {
        calculate(goal, cur * values[idx], idx + 1, values)
    }
}

#[aoc(day07, part1)]
fn part1(input: &ParsedInput) -> Result<i64, Box<dyn Error>> {
    Ok(input
        .iter()
        .filter_map(|(goal, values)| {
            if calculate(*goal, values[0], 1, values) {
                Some(goal)
            } else {
                None
            }
        })
        .sum::<u64>() as i64)
}

#[aoc(day07, part1, rayon)]
fn part1_rayon(input: &ParsedInput) -> Result<i64, Box<dyn Error>> {
    Ok(input
        .par_iter()
        .filter_map(|(goal, values)| {
            if calculate(*goal, values[0], 1, values) {
                Some(goal)
            } else {
                None
            }
        })
        .sum::<u64>() as i64)
}

fn calculate_2(goal: u64, cur: u64, idx: usize, values: &Vec<u64>) -> bool {
    if idx == values.len() {
        return cur == goal;
    }

    if calculate_2(goal, cur + values[idx], idx + 1, values)
        || calculate_2(goal, cur * values[idx], idx + 1, values)
    {
        return true;
    }

    let mut cur = cur;
    let mut tmp = values[idx];
    while tmp > 0 {
        cur *= 10;
        tmp /= 10;
    }

    cur += values[idx];
    calculate_2(goal, cur, idx + 1, values)
}

#[aoc(day07, part2)]
fn part2(input: &ParsedInput) -> Result<i64, Box<dyn Error>> {
    Ok(input
        .par_iter()
        .filter_map(|(goal, values)| {
            if calculate_2(*goal, values[0], 1, values) {
                Some(goal)
            } else {
                None
            }
        })
        .sum::<u64>() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)).unwrap(), 3749);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE)).unwrap(), 11387);
    }
}
