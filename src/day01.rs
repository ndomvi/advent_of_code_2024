use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

type ParsedInput = (Vec<i32>, Vec<i32>);

#[aoc_generator(day01)]
fn parse(input: &str) -> ParsedInput {
    input
        .lines()
        .fold((vec![], vec![]), |(mut vec1, mut vec2), line| {
            let mut s = line.split_ascii_whitespace();

            let l = s.next().unwrap();
            let r = s.next().unwrap();

            vec1.push(l.parse().unwrap());
            vec2.push(r.parse().unwrap());

            (vec1, vec2)
        })
}

#[aoc(day01, part1)]
fn part1(input: &ParsedInput) -> i64 {
    let mut vec1 = input.0.clone();
    let mut vec2 = input.1.clone();

    vec1.sort_unstable();
    vec2.sort_unstable();

    vec1.iter()
        .zip(vec2.iter())
        .fold(0, |res, (l, r)| res + l.abs_diff(*r))
        .into()
}

#[aoc(day01, part2)]
fn part2(input: &ParsedInput) -> i64 {
    let vec1 = input.0.clone();
    let vec2 = input.1.clone();

    let mut occurences: HashMap<i32, i32> = HashMap::new();
    for x in vec2 {
        occurences.entry(x).and_modify(|i| *i += 1).or_insert(1);
    }

    vec1.iter()
        .fold(0, |acc, x| acc + (x * occurences.get(x).unwrap_or(&0)))
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE)), 31);
    }
}
