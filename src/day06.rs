use std::ops::Add;

use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashSet;
use rayon::prelude::*;

type ParsedInput = (Point, Point, HashSet<Point>);
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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

#[aoc_generator(day06)]
fn parse(input: &str) -> ParsedInput {
    let lines = input.lines().collect::<Vec<_>>();
    let dims = Point {
        x: lines.len() as i32,
        y: lines[0].len() as i32,
    };

    let mut obstacles = HashSet::new();
    let mut start = None;
    for (y, l) in lines.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                '.' => continue,
                '#' => {
                    obstacles.insert(Point {
                        x: x as i32,
                        y: y as i32,
                    });
                }
                '^' => {
                    start = Some(Point {
                        x: x as i32,
                        y: y as i32,
                    })
                }
                unknown => panic!("Unknown char {unknown} at x={x}, y={y}"),
            }
        }
    }

    (dims, start.unwrap(), obstacles)
}

fn find_visited((dims, start, obstacles): &ParsedInput) -> (HashSet<(Point, Point)>, bool) {
    let mut visited = HashSet::new();
    let mut cur = *start;

    let mut dir = Point { x: 0, y: -1 };

    while cur.x >= 0 && cur.x < dims.x && cur.y >= 0 && cur.y < dims.y {
        if !visited.insert((cur, dir)) {
            return (visited, true);
        }

        let next = cur + dir;

        // Hit an obstacle
        if obstacles.contains(&next) {
            dir = Point {
                x: -dir.y,
                y: dir.x,
            };
        } else {
            cur = next;
        }
    }

    (visited, false)
}

#[aoc(day06, part1)]
fn part1(input: &ParsedInput) -> i64 {
    find_visited(input)
        .0
        .iter()
        .map(|v| v.0)
        .collect::<HashSet<_>>()
        .len() as i64
}

#[aoc(day06, part2)]
fn part2(input: &ParsedInput) -> i64 {
    let visited = find_visited(input)
        .0
        .iter()
        .map(|v| v.0)
        .collect::<HashSet<_>>();
    let res = visited
        .par_iter()
        .filter(|candidate| {
            let mut obstacles_new = input.2.clone();
            obstacles_new.insert(**candidate);

            find_visited(&(input.0, input.1, obstacles_new)).1
        })
        .count();

    res as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)), 41);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE)), 6);
    }
}
