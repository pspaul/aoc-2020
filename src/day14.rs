use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;
use std::collections::HashMap;

enum Operation {
    UpdateMask(usize, usize),
    WriteValue(usize, usize),
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match &s[1..2] {
            "a" => Operation::UpdateMask(
                usize::from_str_radix(&s[7..].replace('X', "1"), 2).map_err(|_| ())?,
                usize::from_str_radix(&s[7..].replace('X', "0"), 2).map_err(|_| ())?,
            ),
            "e" => Operation::WriteValue(
                s[4..s.find(']').unwrap()].parse().map_err(|_| ())?,
                s[(s.find('=').unwrap() + 2)..].parse().map_err(|_| ())?,
            ),
            _ => return Err(()),
        })
    }
}

#[aoc_generator(day14)]
fn parse_input_day14(input: &str) -> Vec<Operation> {
    input.lines().filter_map(|line| Operation::from_str(line).ok()).collect()
}

#[aoc(day14, part1)]
fn part1(ops: &Vec<Operation>) -> usize {
    let mut mem: HashMap<usize, usize> = HashMap::new();

    let mut mask = (0, 0);
    for op in ops {
        match op {
            Operation::UpdateMask(one_mask, zero_mask) => {
                mask = (*one_mask, *zero_mask);
            },
            Operation::WriteValue(address, value) => {
                mem.insert(*address, *value & mask.0 | mask.1);
            }
        }
    }

    mem.values().sum()
}

fn mask_address(address: usize, x_mask: usize) -> Vec<usize> {
    let mut addresses = Vec::with_capacity(2usize.pow(x_mask.count_ones()));
    addresses.push(address);
    for pos in 0..36 {
        let pos_mask = 1 << pos;
        if (x_mask & pos_mask) > 0 {
            for i in 0..addresses.len() {
                addresses.push(addresses[i] ^ pos_mask);
            }
        }
    }
    addresses
}

#[aoc(day14, part2)]
fn part2(ops: &Vec<Operation>) -> usize {
    let mut mem: HashMap<usize, usize> = HashMap::new();

    let mut mask = (0, 0);
    for op in ops {
        match op {
            Operation::UpdateMask(one_mask, zero_mask) => {
                mask = (*one_mask, *zero_mask);
            },
            Operation::WriteValue(address, value) => {
                let x_mask = (mask.0 ^ mask.1) & ((1 << 36) - 1);
                let address_masked_ones = *address | mask.1;
                for possible_address in mask_address(address_masked_ones, x_mask) {
                    mem.insert(possible_address, *value);
                }
            }
        }
    }

    mem.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        assert_eq!(part1(&parse_input_day14(input)), 165);
    }

    #[test]
    fn part2_example() {
        let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        assert_eq!(part2(&parse_input_day14(input)), 208);
    }
}
