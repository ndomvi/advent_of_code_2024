use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Clone, Copy, Debug)]
struct Robot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

type ParsedInput = Vec<Robot>;
#[aoc_generator(day14)]
fn parse(input: &str) -> ParsedInput {
    let re = Regex::new(r"[\-\d]+").unwrap();
    input
        .lines()
        .map(|l| {
            let mut c = re.find_iter(l);
            Robot {
                x: c.next().unwrap().as_str().parse().unwrap(),
                y: c.next().unwrap().as_str().parse().unwrap(),
                dx: c.next().unwrap().as_str().parse().unwrap(),
                dy: c.next().unwrap().as_str().parse().unwrap(),
            }
        })
        .collect::<Vec<_>>()
}

fn calculate_score(robots: &ParsedInput, seconds: i32, dim_x: i32, dim_y: i32) -> i32 {
    let mut robots = robots.to_owned();
    for robot in &mut robots {
        robot.x = (robot.x + robot.dx * seconds).rem_euclid(dim_x);
        robot.y = (robot.y + robot.dy * seconds).rem_euclid(dim_y);
    }

    let mut scores = [0; 4];
    let mid_x = dim_x / 2;
    let mid_y = dim_y / 2;
    for robot in robots {
        if robot.x < mid_x && robot.y < mid_y {
            scores[1] += 1;
        } else if robot.x > mid_x && robot.y < mid_y {
            scores[0] += 1;
        } else if robot.x < mid_x && robot.y > mid_y {
            scores[2] += 1;
        } else if robot.x > mid_x && robot.y > mid_y {
            scores[3] += 1;
        }
    }

    scores.iter().product()
}

#[aoc(day14, part1)]
fn part1(input: &ParsedInput) -> i64 {
    calculate_score(input, 100, 101, 103) as i64
}

#[aoc(day14, part2)]
fn part2(input: &ParsedInput) -> i64 {
    let mut robots = input.to_owned();
    let mut m = i32::MAX;
    // let mut m_r = vec![];
    let mut sec = 0;
    for second in 1..100000 {
        for robot in &mut robots {
            robot.x = (robot.x + robot.dx).rem_euclid(101);
            robot.y = (robot.y + robot.dy).rem_euclid(103);
        }

        let mut scores = [0; 4];
        let mid_x = 101 / 2;
        let mid_y = 103 / 2;
        for robot in &robots {
            if robot.x < mid_x && robot.y < mid_y {
                scores[1] += 1;
            } else if robot.x > mid_x && robot.y < mid_y {
                scores[0] += 1;
            } else if robot.x < mid_x && robot.y > mid_y {
                scores[2] += 1;
            } else if robot.x > mid_x && robot.y > mid_y {
                scores[3] += 1;
            }
        }

        let s = scores.iter().product();
        if s < m {
            m = s;
            sec = second;
            // m_r = robots.clone();
        }
    }

    // let mut grid = vec![vec!['.'; 101]; 103];
    // for robot in m_r {
    //     grid[robot.y as usize][robot.x as usize] = '#';
    // }

    // for l in grid {
    //     let mut s = "".to_owned();
    //     l.iter().for_each(|c| s.push(*c));
    //     println!("{s}");
    // }

    sec as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;
    #[test]
    fn part1_example() {
        assert_eq!(calculate_score(&parse(TESTCASE), 100, 11, 7), 12);
    }

    #[test]
    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE)), 0);
    }
}
