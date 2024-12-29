use aoc_runner_derive::{aoc, aoc_generator};
use cached::proc_macro::cached;
use hashbrown::HashMap;
use rayon::prelude::*;

type ParsedInput = Vec<u64>;

#[aoc_generator(day22)]
fn parse(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>()
}

const fn next_number(mut num: u64) -> u64 {
    let m1 = num * 64;
    num ^= m1;
    num %= 16777216;

    let d = num / 32;
    num ^= d;
    num %= 16777216;

    let m2 = num * 2048;
    num ^= m2;
    num %= 16777216;

    num
}

#[aoc(day22, part1)]
fn part1(input: &ParsedInput) -> u64 {
    input
        .par_iter()
        .map(|n| {
            let mut num = *n;
            for _ in 1..=2000 {
                num = next_number(num);
            }

            num
        })
        .sum()
}

#[cached(key = "[i64; 4]", convert = r##"{ key }"##)]
fn calculate_value(key: [i64; 4], sellers: &[HashMap<[i64; 4], u64>]) -> u64 {
    sellers
        .iter()
        .map(|i| i.get(&key).copied().unwrap_or(0))
        .sum()
}

#[aoc(day22, part2)]
fn part2(input: &ParsedInput) -> u64 {
    let sellers = input
        .par_iter()
        .map(|n| {
            let mut num = *n;
            let prices = (1..=2001)
                .map(|_| {
                    let res = num % 10;
                    num = next_number(num);
                    res
                })
                .collect::<Vec<_>>();

            let deltas = prices
                .windows(2)
                .map(|x| x[1] as i64 - x[0] as i64)
                .collect::<Vec<_>>();
            let delta_price: HashMap<[i64; 4], u64> = deltas
                .windows(4)
                .enumerate()
                .map(|(i, v)| (v.try_into().unwrap(), prices[i + 4]))
                .fold(HashMap::with_capacity(prices.len()), |mut m, kv| {
                    m.entry(kv.0).or_insert(kv.1);
                    m
                });

            delta_price
        })
        .collect::<Vec<_>>();

    sellers
        .par_iter()
        .map(|s| {
            s.keys()
                .map(|k| calculate_value(*k, &sellers))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

#[aoc(day22, part2, fast)]
fn part2_fast(input: &ParsedInput) -> u64 {
    input
        .par_iter()
        // Prepare the sequence - delta maps
        .map(|n| {
            let mut num = *n;
            let prices = (0..=2000)
                .map(|_| {
                    let res = num % 10;
                    num = next_number(num);
                    res
                })
                .collect::<Vec<_>>();

            // Deltas are the changes in the price
            let deltas = prices
                .windows(2)
                .map(|x| x[1] as i64 - x[0] as i64)
                .collect::<Vec<_>>();

            // A map between sequence-price for current trader. Only the first price is considered
            let mut seq_price: HashMap<[i64; 4], u64> = HashMap::with_capacity(prices.len());
            for (i, seq) in deltas.windows(4).enumerate() {
                seq_price
                    .entry(seq.try_into().unwrap())
                    .or_insert(prices[i + 4]);
            }

            seq_price
        })
        // Reduce the maps to a single one by adding up total profit from each sequence
        .reduce(HashMap::new, |mut total_seq, cur_seq| {
            for (key, value) in cur_seq {
                total_seq
                    .entry(key)
                    .and_modify(|v| *v += value)
                    .or_insert(value);
            }

            total_seq
        })
        .iter()
        .max_by_key(|x| x.1)
        .unwrap()
        .1
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#"1
10
100
2024"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)), 37327623);
    }

    const TESTCASE_2: &str = r#"1
2
3
2024"#;
    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE_2)), 23);
    }
}
