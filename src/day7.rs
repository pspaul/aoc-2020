use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};
use lazy_static::lazy_static;
use regex::Regex;

struct BagRules {
    /// all the bags
    bags: HashMap<String, Bag>,

    /// which bag is inside of which bags?
    contained_by_lookup: HashMap<String, Vec<(usize, String)>>,
}

impl BagRules {
    fn new() -> Self {
        BagRules {
            bags: HashMap::new(),
            contained_by_lookup: HashMap::new(),
        }
    }
}

#[derive(Clone)]
struct Bag {
    color: String,
    inner_bags: Vec<(usize, String)>,
}

lazy_static! {
    static ref LINE_PATTERN: Regex = Regex::new(r"^(?P<color>\w+ \w+) bags contain (?P<first>[\w ]+ bags?)(?:, (?P<remaining>[,\w ]+))?\.$").unwrap();
    static ref LINE_REMAINING_PATTERN: Regex = Regex::new(r", (?P<remaining>[\w ]+ bags?)").unwrap();
    static ref QUANTITY_PATTERN: Regex = Regex::new(r"(?P<quantity>\d+) (?P<color>\w+ \w+) bags?").unwrap();
}

fn parse_bag_rule(line: &str) -> Option<Bag> {
    let cap = LINE_PATTERN.captures(line)?;

    let mut bag = Bag {
        color: cap.name("color")?.as_str().to_string(),
        inner_bags: vec!(),
    };

    let first_inner = cap.name("first")?.as_str();
    if first_inner != "no other bags" {
        let (quantity, color) = parse_bag_quantity(first_inner)?;
        bag.inner_bags.push((quantity, color.to_string()));

        if let Some(remaining) = cap.name("remaining") {
            for nth_inner in QUANTITY_PATTERN.captures_iter(remaining.as_str()) {
                let (quantity, color) = (nth_inner["quantity"].parse::<usize>().unwrap(), nth_inner["color"].to_string());
                bag.inner_bags.push((quantity, color));
            };
        }
    }

    Some(bag)
}

fn parse_bag_quantity(s: &str) -> Option<(usize, &str)> {
    let cap = QUANTITY_PATTERN.captures(s)?;
    let quantity = cap.name("quantity")?.as_str().parse::<usize>().map_or(None, Some)?;
    let color = cap.name("color")?.as_str();
    Some((quantity, color))
}

#[aoc_generator(day7)]
fn parse_input_day7(input: &str) -> BagRules {
    input
        .lines()
        .filter_map(|line| parse_bag_rule(line))
        .fold(BagRules::new(), |mut rules, bag| {
            rules.bags.insert(bag.color.clone(), bag.clone());
            for (quantity, color) in bag.inner_bags {
                if !rules.contained_by_lookup.contains_key(&color) {
                    rules.contained_by_lookup.insert(color.clone(), Vec::new());
                }
                let lookup = rules.contained_by_lookup.get_mut(&color).unwrap();
                lookup.push((quantity, bag.color.clone()));
            }
            rules
        })
}

#[aoc(day7, part1)]
fn part1(rules: &BagRules) -> usize {
    let target = "shiny gold";

    if !rules.bags.contains_key(target) {
        return 0;
    }

    let mut stack: Vec<&(usize, String)> = Vec::new();
    let mut visited: HashSet<&String> = HashSet::new();
    let mut counter = 0;

    // start at target and go up the tree
    rules.contained_by_lookup.get(target).unwrap().iter().for_each(|item| stack.push(item));

    while let Some((_cur_count, cur_bag)) = stack.pop() {
        if visited.contains(cur_bag) {
            continue;
        }
        visited.insert(cur_bag);
        counter += 1;

        if let Some(inner_bags) = rules.contained_by_lookup.get(cur_bag) {
            for inner_bag in inner_bags {
                stack.push(inner_bag);
            }
        }
    }

    counter
}

#[aoc(day7, part2)]
fn part2(rules: &BagRules) -> usize {
    let target = "shiny gold";

    if !rules.bags.contains_key(target) {
        return 0;
    }

    let mut stack: Vec<(usize, String)> = Vec::new();
    let mut counter = 0;

    // start at target and go up the tree
    stack.push((1 as usize, target.to_string()));

    while let Some((cur_count, cur_bag)) = stack.pop() {
        for inner_bag in &rules.bags.get(cur_bag.as_str()).unwrap().inner_bags {
            counter += cur_count * inner_bag.0;
            stack.push((cur_count * inner_bag.0, inner_bag.1.clone()));
        }
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test_1() {
        let rule = parse_bag_rule("muted gold bags contain 1 wavy red bag, 3 mirrored violet bags, 5 bright gold bags, 5 plaid white bags.");
        assert!(rule.is_some());
        let rule = rule.unwrap();
        assert_eq!(rule.inner_bags.len(), 4);
    }

    const INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input_day7(INPUT)), 4);
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(part2(&parse_input_day7(INPUT)), 32);
    }

    #[test]
    fn part2_example_2() {
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        assert_eq!(part2(&parse_input_day7(input)), 126);
    }
}
