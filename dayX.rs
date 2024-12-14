use aoc_runner_derive::{aoc, aoc_generator};

type ParsedInput = Vec<String>;

#[aoc_generator(dayX)]
fn parse(input: &str) -> ParsedInput {
    input.lines().map(String::from).collect::<Vec<_>>()
}

#[aoc(dayX, part1)]
fn part1(input: &ParsedInput) -> i64 {
    todo!()
}

#[aoc(dayX, part2)]
fn part2(input: &ParsedInput) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#""#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)), 0);
    }

    #[test]
    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE)), 0);
    }
}
