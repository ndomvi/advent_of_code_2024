use std::error::Error;

use aoc_runner_derive::{aoc, aoc_generator};
use smallvec::SmallVec;

type ParsedInput = SmallVec<[SmallVec<[char; 256]>; 256]>;

#[aoc_generator(day04)]
fn parse(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|l| l.chars().collect::<_>())
        .collect::<_>()
}

const CHARS: [char; 4] = ['X', 'M', 'A', 'S'];

fn explore(map: &ParsedInput, x: i32, y: i32, (dir_x, dir_y): (i32, i32), idx: usize) -> bool {
    // Out of bounds
    if x < 0 || x >= map[0].len() as i32 || y < 0 || y >= map.len() as i32 {
        return false;
    }

    // Current char is wrong
    if map[y as usize][x as usize] != CHARS[idx] {
        return false;
    } else if idx == CHARS.len() - 1 {
        // The char is correct and we are at the last char
        return true;
    }

    explore(map, x + dir_x, y + dir_y, (dir_x, dir_y), idx + 1)
}

const DIRS: [(i32, i32); 8] = [
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[aoc(day04, part1)]
fn part1(input: &ParsedInput) -> Result<i64, Box<dyn Error>> {
    let mut res = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            for dir in DIRS {
                if explore(input, x as i32, y as i32, dir, 0) {
                    res += 1;
                }
            }
        }
    }

    Ok(res)
}

#[aoc(day04, part2)]
fn part2(input: &ParsedInput) -> Result<i64, Box<dyn Error>> {
    let mut res = 0;
    for y in 1..input.len() - 1 {
        for x in 1..input[y].len() - 1 {
            // Quick check to see whether the current pos can be a center of a X-MAS
            if input[y][x] == 'A' {
                // Forgive me Father, for I have hardcoded the values
                match (input[y - 1][x - 1], input[y + 1][x + 1]) {
                    ('M', 'S') | ('S', 'M') => match (input[y - 1][x + 1], input[y + 1][x - 1]) {
                        ('M', 'S') | ('S', 'M') => res += 1,
                        _ => continue,
                    },
                    _ => continue,
                }
            }
        }
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)).unwrap(), 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE)).unwrap(), 9);
    }
}
