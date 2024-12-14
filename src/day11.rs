use cached::proc_macro::cached;

use aoc_runner_derive::{aoc, aoc_generator};

type ParsedInput = Vec<u64>;

#[aoc_generator(day11)]
fn parse(input: &str) -> ParsedInput {
    input
        .split_ascii_whitespace()
        .map(|i| i.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
}

#[cached]
fn blink(stone: u64, num_blinks: u32) -> u64 {
    if num_blinks == 0 {
        return 1;
    }

    if stone == 0 {
        return blink(1, num_blinks - 1);
    }

    let num_digits = stone.ilog10() + 1;
    if num_digits % 2 == 0 {
        return blink(stone % 10u64.pow(num_digits / 2), num_blinks - 1)
            + blink(stone / 10u64.pow(num_digits / 2), num_blinks - 1);
    }

    return blink(stone * 2024, num_blinks - 1);
}

#[aoc(day11, part1)]
fn part1(input: &ParsedInput) -> u64 {
    input.iter().map(|stone| blink(*stone, 25)).sum()
}

#[aoc(day11, part2)]
fn part2(input: &ParsedInput) -> u64 {
    input.iter().map(|stone| blink(*stone, 75)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#"125 17"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)), 55312);
    }

    #[test]
    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE)), 0);
    }
}
