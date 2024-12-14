use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

type ParsedInput = Vec<Vec<f64>>;

#[aoc_generator(day13)]
fn parse(input: &str) -> ParsedInput {
    let re = Regex::new(r"[\d]+").unwrap();
    input
        .split("\n\n")
        .map(|machine| {
            re.find_iter(machine)
                .map(|m| m.as_str().parse().unwrap())
                .collect::<_>()
        })
        .collect::<Vec<_>>()
}

#[aoc(day13, part1)]
fn part1(input: &ParsedInput) -> i64 {
    let mut res = 0;
    for machine in input {
        let [ax, ay, bx, by, x, y] = machine[..] else {
            panic!("Invalid machine! {machine:?}");
        };

        // Button A: X+94, Y+34
        // Button B: X+22, Y+67
        // Prize: X=8400, Y=5400
        // 94*a + 22*b = x
        // 34*a + 67*b = y
        // b = (x-94*a)/22
        // 34*a + 67*(x-94*a)/22 = y
        // 34*a + 67*x/22 -67*94*a/22 = y
        // a*(34 - 67*94/22) = y -67*x/22
        // a = (y-67*x/22)/(34 - 67*94/22)
        // a = (y-by*x/bx)/(ay-by*ax/bx)

        let a = (y - by * x / bx) / (ay - by * ax / bx);
        let b = (x - ax * a) / bx;

        // Bruh. I don't know how well it works, but it does.
        if (a - a.round()).abs() < f64::EPSILON * 10000000000000.0
            && (b - b.round()).abs() < f64::EPSILON * 10000000000000.0
        {
            res += (3.0 * a + b).round() as i64;
        }
    }

    res
}

#[aoc(day13, part2)]
fn part2(input: &ParsedInput) -> i64 {
    part1(
        &input
            .iter()
            .map(|m| {
                let mut m = m.to_owned();
                m[4] += 10000000000000.0;
                m[5] += 10000000000000.0;
                m
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)), 480);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE)), 875318608908);
    }
}
