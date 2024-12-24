use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::{HashMap, HashSet};
use smallvec::SmallVec;

type RulesType = HashMap<u32, HashSet<u32>>;
type ParsedInput = (RulesType, Vec<Vec<u32>>);

#[aoc_generator(day05)]
fn parse(input: &str) -> ParsedInput {
    let input = input.split_once("\n\n").unwrap();
    let rules_parsed = input
        .0
        .lines()
        .map(|l| l.split_once("|").unwrap())
        .map(|pair| {
            (
                pair.0.parse::<u32>().unwrap(),
                pair.1.parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut rules: RulesType = HashMap::new();
    for rule in rules_parsed {
        rules
            .entry(rule.1)
            .and_modify(|r| {
                r.insert(rule.0);
            })
            .or_insert_with(|| HashSet::from([rule.0]));
    }

    let updates = input
        .1
        .lines()
        .map(|l| {
            l.split(',')
                .map(|page| page.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    (rules, updates)
}

fn validate_update(update: &[u32], rules: &RulesType) -> bool {
    let mut banned: HashSet<u32> = HashSet::new();

    for page in update {
        if banned.contains(page) {
            return false;
        }
        if let Some(banned_new) = rules.get(page) {
            banned.extend(banned_new);
        }
    }

    true
}

#[aoc(day05, part1)]
fn part1((rules, updates): &ParsedInput) -> i64 {
    let mut res = 0;

    for update in updates {
        if validate_update(update, rules) {
            res += update[update.len() / 2];
        }
    }

    res as i64
}

fn fix_update(update: &[u32], rules: &RulesType) -> SmallVec<[u32; 32]> {
    let mut update = update.to_owned();
    let mut fixed = SmallVec::new();
    while !update.is_empty() {
        'candidate: for (idx, page) in update.iter().enumerate() {
            for (_, prev_page) in update.iter().enumerate().filter(|(i, _)| *i != idx) {
                if let Some(rule) = rules.get(prev_page) {
                    if rule.contains(page) {
                        continue 'candidate;
                    }
                }
            }

            fixed.push(*page);
            update.remove(idx);
            break;
        }
    }

    fixed.reverse();
    fixed
}

#[aoc(day05, part2)]
fn part2((rules, updates): &ParsedInput) -> i64 {
    let mut res = 0;

    for update in updates {
        if !validate_update(update, rules) {
            let fixed = fix_update(update, rules);
            res += fixed[fixed.len() / 2];
        }
    }

    res as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TESTCASE)), 143);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TESTCASE)), 123);
    }
}
