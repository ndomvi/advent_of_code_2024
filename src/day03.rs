use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

type ParsedInput = Vec<String>;

#[aoc_generator(day03)]
fn parse(input: &str) -> ParsedInput {
    input.lines().map(String::from).collect::<Vec<_>>()
}

#[aoc(day03, part1)]
fn part1(input: &ParsedInput) -> i64 {
    let mut res: i64 = 0;
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    for l in input {
        re.captures_iter(l).for_each(|m| {
            res += m.get(1).unwrap().as_str().parse::<i64>().unwrap()
                * m.get(2).unwrap().as_str().parse::<i64>().unwrap();
        });
    }

    res
}

#[aoc(day03, part2)]
fn part2(input: &ParsedInput) -> i64 {
    let mut res: i64 = 0;
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)|don\'t\(\)").unwrap();
    let mut enabled = true;
    for l in input {
        re.captures_iter(l)
            .for_each(|m| match m.get(0).unwrap().as_str() {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {
                    if enabled {
                        res += m.get(1).unwrap().as_str().parse::<i64>().unwrap()
                            * m.get(2).unwrap().as_str().parse::<i64>().unwrap();
                    }
                }
            });
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str =
        r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)), 161);
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#
            )),
            48
        );
    }
}
