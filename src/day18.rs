use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashSet; // 25% faster
use pathfinding::prelude::bfs;
use smallvec::{smallvec, SmallVec}; // 25% faster

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Pos(i32, i32);

const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
impl Pos {
    fn successors(self, corrupted: &HashSet<Self>) -> SmallVec<[Self; 4]> {
        let Self(x, y) = self;
        let mut res = smallvec![];
        for (dx, dy) in DIRS {
            let nx = x + dx;
            let ny = y + dy;
            if (0..=70).contains(&nx)
                && (0..=70).contains(&ny)
                && !corrupted.contains(&Self(nx, ny))
            {
                res.push(Self(nx, ny));
            }
        }

        res
    }
}

type ParsedInput = Vec<Pos>;

#[aoc_generator(day18)]
fn parse(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            Pos(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect::<_>()
}

static GOAL: Pos = Pos(70, 70);
#[aoc(day18, part1)]
fn part1(input: &ParsedInput) -> i64 {
    let corrupted = input[..1024].iter().copied().collect();
    let result = bfs(&Pos(0, 0), |p| p.successors(&corrupted), |p| *p == GOAL);

    result.unwrap().len() as i64 - 1
}

#[aoc(day18, part2)]
fn part2(input: &ParsedInput) -> String {
    let mut corrupted = input[..1024].iter().copied().collect();
    let mut prev_result = bfs(&Pos(0, 0), |p| p.successors(&corrupted), |p| *p == GOAL).unwrap();
    for c in input[1024..].iter().copied() {
        corrupted.insert(c);
        // Skip processing bytes that don't block the current shortest path
        if !prev_result.contains(&c) {
            continue;
        }

        let result = bfs(&Pos(0, 0), |p| p.successors(&corrupted), |p| *p == GOAL);
        match result {
            Some(r) => {
                prev_result = r;
            }
            None => return format!("{},{}", c.0, c.1),
        }
    }

    panic!("Blocking byte not found!")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#""#;
    #[test]
    #[ignore]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)), 0);
    }

    #[test]
    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE)), "0,0");
    }
}
