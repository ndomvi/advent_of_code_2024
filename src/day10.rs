use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

type ParsedInput = Vec<Vec<u32>>;

#[aoc_generator(day10)]
fn parse(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<_>>()
}

const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
fn calculate_score(x: i32, y: i32, map: &ParsedInput, visited: &mut HashSet<(i32, i32)>) -> i32 {
    if !visited.insert((x, y)) {
        return 0;
    }

    if map[y as usize][x as usize] == 9 {
        return 1;
    }

    let mut score = 0;
    for dir in DIRS {
        let x_n = x + dir.0;
        let y_n = y + dir.1;

        if (x_n >= 0 && x_n < map[0].len() as i32 && y_n >= 0 && y_n < map.len() as i32)
            && map[y as usize][x as usize] as i32 - map[y_n as usize][x_n as usize] as i32 == -1
        {
            score += calculate_score(x_n, y_n, map, visited);
        }
    }

    score
}

#[aoc(day10, part1)]
fn part1(input: &ParsedInput) -> i32 {
    let mut res = 0;

    for (y, l) in input.iter().enumerate() {
        for (x, height) in l.iter().enumerate() {
            if *height == 0 {
                res += calculate_score(x as i32, y as i32, input, &mut HashSet::new());
            }
        }
    }

    res
}

fn calculate_score_2(x: i32, y: i32, map: &ParsedInput) -> i32 {
    if map[y as usize][x as usize] == 9 {
        return 1;
    }

    let mut score = 0;
    for dir in DIRS {
        let x_n = x + dir.0;
        let y_n = y + dir.1;

        if (x_n >= 0 && x_n < map[0].len() as i32 && y_n >= 0 && y_n < map.len() as i32)
            && map[y as usize][x as usize] as i32 - map[y_n as usize][x_n as usize] as i32 == -1
        {
            score += calculate_score_2(x_n, y_n, map);
        }
    }

    score
}

#[aoc(day10, part2)]
fn part2(input: &ParsedInput) -> i32 {
    let mut res = 0;

    for (y, l) in input.iter().enumerate() {
        for (x, height) in l.iter().enumerate() {
            if *height == 0 {
                res += calculate_score_2(x as i32, y as i32, input);
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

    const TESTCASE_2: &str = r#"5550555
5551555
8772778
6543456
7226227
8222228
9222229"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)), 36);
    }

    #[test]
    fn part1_example_2() {
        assert_eq!(part1(&parse(TESTCASE_2)), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE)), 81);
    }
}
