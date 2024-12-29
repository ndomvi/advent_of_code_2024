use aoc_runner_derive::{aoc, aoc_generator};
use cached::proc_macro::cached;

type ParsedInput = (Vec<String>, Vec<String>);

#[aoc_generator(day19)]
fn parse(input: &str) -> ParsedInput {
    let (pat, des) = input.split_once("\n\n").unwrap();
    let patterns = pat.split(", ").map(|s| s.to_owned()).collect();
    let designs = des.lines().map(|s| s.to_owned()).collect();
    (patterns, designs)
}

#[cached(key = "String", convert = r##"{ String::from(s) }"##)]
fn bite(s: &str, pat: &Vec<String>) -> bool {
    if s.is_empty() {
        return true;
    }

    for p in pat {
        if let Some(ns) = s.strip_prefix(p) {
            if bite(ns, pat) {
                return true;
            }
        }
    }

    false
}

#[aoc(day19, part1)]
fn part1((patterns, designs): &ParsedInput) -> i64 {
    let mut res = 0;
    for design in designs {
        if bite(design, patterns) {
            res += 1;
        }
    }

    res
}

#[cached(key = "String", convert = r##"{ String::from(s) }"##)]
fn bite_2(s: &str, pat: &Vec<String>) -> u64 {
    if s.is_empty() {
        return 1;
    }

    let mut res = 0;
    for p in pat {
        if let Some(ns) = s.strip_prefix(p) {
            res += bite_2(ns, pat);
        }
    }

    res
}

#[aoc(day19, part2)]
fn part2((patterns, designs): &ParsedInput) -> u64 {
    designs.iter().map(|design| bite_2(design, patterns)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)), 6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE)), 16);
    }
}
