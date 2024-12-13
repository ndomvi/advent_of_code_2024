use std::error::Error;

use aoc_runner_derive::{aoc, aoc_generator};
use smallvec::SmallVec;

type ParsedInput = Vec<SmallVec<[i32; 16]>>;

#[aoc_generator(day02)]
fn parse(input: &str) -> ParsedInput {
    input
        .lines()
        .map(String::from)
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<_>()
        })
        .collect::<_>()
}

fn check_report(report: &SmallVec<[i32; 16]>) -> bool {
    let growing = report[0] < report[1];

    for (cur, next) in report.iter().zip(report.iter().skip(1)) {
        if cur == next || ((cur < next) != growing) || (cur.abs_diff(*next) > 3) {
            return false;
        }
    }

    true
}

#[aoc(day02, part1)]
fn part1(input: &ParsedInput) -> Result<i64, Box<dyn Error>> {
    let mut res = 0;

    for i in input {
        if check_report(i) {
            res += 1;
        }
    }
    Ok(res)
}

fn check_report_2(report: &SmallVec<[i32; 16]>, removed: bool) -> bool {
    let growing = report[0] < report[1];

    for (idx, (cur, next)) in report.iter().zip(report.iter().skip(1)).enumerate() {
        if cur == next || ((cur < next) != growing) || (cur.abs_diff(*next) > 3) {
            if removed {
                return false;
            }

            let mut rep_l = report.clone();
            rep_l.remove(idx);
            let mut rep_r = report.clone();
            rep_r.remove(idx + 1);
            // HACK: suboptimal approach.The first element affects the growing, so it is never considered incorrect
            // This solves the issue by always trying to remove it (even if the problem is in much later elements).
            let mut rep_z = report.clone();
            rep_z.remove(0);
            return check_report_2(&rep_l, true)
                || check_report_2(&rep_r, true)
                || check_report_2(&rep_z, true);
        }
    }

    true
}

#[aoc(day02, part2)]
fn part2(input: &ParsedInput) -> Result<i64, Box<dyn Error>> {
    let mut res = 0;

    for i in input {
        if check_report_2(i, false) {
            res += 1;
        }
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)).unwrap(), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE)).unwrap(), 4);
    }

    #[test]
    fn part2_1() {
        assert_eq!(
            part2(&parse(
                r#"63 60 62 65 67 69
43 41 43 45 46 49
21 24 23 21 20 19 18
26 24 26 29 30 31 33 34"#
            ))
            .unwrap(),
            4
        );
    }
}
