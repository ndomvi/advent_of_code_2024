use aoc_runner_derive::{aoc, aoc_generator};

type ParsedInput = Vec<i32>;

#[aoc_generator(day09, part1)]
fn parse(input: &str) -> ParsedInput {
    let mut parsed = vec![];

    let mut file = true;
    let mut id = 0;
    for c in input.chars() {
        if file {
            parsed.extend([id].repeat(c.to_digit(10).unwrap() as usize));
            id += 1;
        } else {
            parsed.extend([-1].repeat(c.to_digit(10).unwrap() as usize));
        }

        file = !file;
    }

    parsed
}

#[aoc(day09, part1)]
fn part1(input: &ParsedInput) -> i64 {
    let mut fs = input.clone();
    let mut l = 0;
    let mut r = input.len() - 1;

    while l != r {
        if fs[l] != -1 {
            l += 1;
        } else if fs[r] == -1 {
            r -= 1;
        } else {
            fs.swap(l, r);
        }
    }

    fs.iter().enumerate().fold(0, |acc, (idx, val)| {
        if *val != -1 {
            acc + (idx as i64 * *val as i64)
        } else {
            acc
        }
    })
}

#[derive(Clone, Debug)]
enum Space {
    File { id: usize, len: usize },
    Empty { len: usize },
}

#[aoc_generator(day09, part2)]
fn parse_2(input: &str) -> Vec<Space> {
    let mut parsed = vec![];

    let mut file = true;
    let mut id = 0;
    for c in input.chars() {
        if file {
            parsed.push(Space::File {
                id,
                len: c.to_digit(10).unwrap() as usize,
            });
            id += 1;
        } else {
            parsed.push(Space::Empty {
                len: c.to_digit(10).unwrap() as usize,
            });
        }

        file = !file;
    }

    parsed
}

#[aoc(day09, part2)]
fn part2(input: &[Space]) -> i64 {
    let mut fs = input.to_vec();
    let mut idx = input.len() - 1;

    while idx > 0 {
        if let Space::File { id, len: len_file } = fs[idx] {
            let mut i = 0;
            while i < idx {
                if let Space::Empty { len: len_empty } = fs[i] {
                    if len_empty >= len_file {
                        fs[idx] = Space::Empty { len: len_file };
                        fs[i] = Space::File { id, len: len_file };
                        if len_empty > len_file {
                            fs.insert(
                                i + 1,
                                Space::Empty {
                                    len: len_empty - len_file,
                                },
                            );
                        }
                        break;
                    }
                }

                i += 1;
            }
        }
        idx -= 1;
    }

    let mut res = 0;
    let mut i = 0;
    for space in fs {
        match space {
            Space::File { id, len } => {
                res += id * len * (i + i + len - 1) / 2;
                i += len;
            }
            Space::Empty { len } => i += len,
        }
    }

    res as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#"2333133121414131402"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)), 1928);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_2(TESTCASE)), 2858);
    }
}
