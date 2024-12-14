use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

type ParsedInput = (Point, HashMap<char, Vec<Point>>);

#[aoc_generator(day08)]
fn parse(input: &str) -> ParsedInput {
    let lines = input.lines().collect::<Vec<_>>();
    let dims = Point {
        x: lines.len() as i32,
        y: lines[0].len() as i32,
    };

    let mut antennas = HashMap::new();
    for (y, l) in lines.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '.' {
                continue;
            }

            let point = Point {
                x: x as i32,
                y: y as i32,
            };
            antennas
                .entry(c)
                .and_modify(|e: &mut Vec<Point>| {
                    e.push(point);
                })
                .or_insert_with(|| vec![point]);
        }
    }

    (dims, antennas)
}

#[aoc(day08, part1)]
fn part1((dims, antennas): &ParsedInput) -> i64 {
    let mut antinodes = HashSet::new();
    for antennas_freq in antennas.values() {
        for center in antennas_freq {
            for other in antennas_freq.iter().filter(|o| center != *o) {
                let new = *center + *center - *other;
                if new.x >= 0 && new.x < dims.x && new.y >= 0 && new.y < dims.y {
                    antinodes.insert(new);
                }
            }
        }
    }

    antinodes.len() as i64
}

#[aoc(day08, part2)]
fn part2((dims, antennas): &ParsedInput) -> i64 {
    let mut antinodes = HashSet::new();
    for antennas_freq in antennas.values() {
        for center in antennas_freq {
            for other in antennas_freq.iter().filter(|o| center != *o) {
                let mut new = *center;
                let diff = *center - *other;
                while new.x >= 0 && new.x < dims.x && new.y >= 0 && new.y < dims.y {
                    antinodes.insert(new);
                    new = new + diff;
                }
            }
        }
    }

    antinodes.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    const TESTCASE_2: &str = r#"..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
.........."#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)), 14);
    }

    #[test]
    fn part1_example_2() {
        assert_eq!(part1(&parse(TESTCASE_2)), 4);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE)), 34);
    }
}
