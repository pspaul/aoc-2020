use aoc_runner_derive::{aoc, aoc_generator};
use std::ops::RangeInclusive;
use std::str::FromStr;
use itertools::Itertools;
use std::collections::HashMap;

struct Rule {
    name: String,
    range1: RangeInclusive<usize>,
    range2: RangeInclusive<usize>,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, ranges) = s.split(": ").collect_tuple::<(&str, &str)>().unwrap();
        let (range1, range2) = ranges.split(" or ").collect_tuple::<(&str, &str)>().unwrap();
        let (range1min, range1max) = range1.split('-').collect_tuple::<(&str, &str)>().unwrap();
        let (range2min, range2max) = range2.split('-').collect_tuple::<(&str, &str)>().unwrap();
        Ok(Rule {
            name: name.to_string(),
            range1: (range1min.parse().unwrap())..=(range1max.parse().unwrap()),
            range2: (range2min.parse().unwrap())..=(range2max.parse().unwrap()),
        })
    }
}

struct Input {
    rules: Vec<Rule>,
    my_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

#[aoc_generator(day16)]
fn parse_input_day16(input: &str) -> Input {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let rules = parts[0]
        .lines()
        .map(|line| Rule::from_str(line).unwrap())
        .collect();

    let my_ticket = parts[1]
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>();

    let nearby_tickets = parts[2]
        .lines()
        .skip(1)
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect::<Vec<Vec<usize>>>();

    Input { rules, my_ticket, nearby_tickets }
}

#[aoc(day16, part1)]
fn part1(input: &Input) -> usize {
    let mut sum = 0;
    for ticket in &input.nearby_tickets {
        for value in ticket {
            let mut valid = false;
            for rule in &input.rules {
                if rule.range1.contains(value) || rule.range2.contains(value) {
                    valid = true;
                    break;
                }
            }
            if !valid {
                sum += value;
            }
        }
    }
    sum
}

#[aoc(day16, part2)]
fn part2(input: &Input) -> usize {
    input.nearby_tickets
        .iter()
        // filter out invalid tickets
        .filter(|ticket| {
            for value in *ticket {
                let mut valid = false;
                for rule in &input.rules {
                    if rule.range1.contains(value) || rule.range2.contains(value) {
                        valid = true;
                        break;
                    }
                }
                if !valid {
                    return false;
                }
            }
            true
        })
        // collect values per field (column) instead of ticket (row)
        .fold(Vec::new(), |mut acc: Vec<Vec<usize>>, valid_ticket| {
            for (i, value) in valid_ticket.iter().enumerate() {
                if let Some(column_values) = acc.get_mut(i) {
                    column_values.push(*value);
                } else {
                    acc.push(vec!(*value));
                }
            }
            acc
        })
        .iter()
        .enumerate()
        // find possible candidate rules for each column
        .fold(HashMap::new(), |mut map: HashMap<String, Vec<usize>>, (i, values)| {
            for rule in &input.rules {
                let mut all_in_range = true;
                for value in values {
                    if !rule.range1.contains(value) && !rule.range2.contains(value) {
                        all_in_range = false;
                        break;
                    }
                }
                if all_in_range {
                    if let Some(rule_candidates) = map.get_mut(&rule.name) {
                        rule_candidates.push(i);
                    } else {
                        map.insert(rule.name.clone(), vec!(i));
                    }
                }
            }
            map
        })
        .iter()
        // sort by number of candidates
        .sorted_by_key(|(_name, candidates)| candidates.len())
        // assign columns to rules based on their candidates
        .fold(HashMap::new(), |mut acc, (name, candidates)| {
            for candidate in candidates {
                if acc.contains_key(candidate) {
                    continue;
                }
                acc.insert(candidate, (name, input.my_ticket[*candidate]));
            }
            acc
        })
        .values()
        // we only want departure fields
        .filter_map(|(name, value)| {
            match name.starts_with("departure") {
                true => Some(value),
                false => None,
            }
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        assert_eq!(part1(&parse_input_day16(input)), 71);
    }

    #[test]
    fn part2_example_1() {
        let input = "departure class: 0-1 or 4-19
row: 0-5 or 8-19
departure seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
        assert_eq!(part2(&parse_input_day16(input)), 12 * 13);
    }
}
