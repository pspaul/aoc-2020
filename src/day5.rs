use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::BTreeSet;
use std::str::FromStr;
use itertools::Itertools;

fn decode_binary_space_partitioning(s: &str, lower: char, higher: char) -> usize {
    let start = 0 as usize;
    let end = (2 as usize).pow(s.len() as u32) - 1 as usize;
    s.chars()
        .fold(start..end, |range, c| {
            if c == lower {
                range.start..((range.start + range.end) / 2)
            } else if c == higher {
                ((range.start + range.end) / 2 + 1)..range.end
            } else {
                range
            }
        })
        .start
}

struct Seat {
    row: usize,
    col: usize,
}

impl Seat {
    fn id(&self) -> usize {
        self.row * 8 + self.col
    }
}

impl FromStr for Seat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Seat {
            row: decode_binary_space_partitioning(&s[..7], 'F', 'B'),
            col: decode_binary_space_partitioning(&s[7..10], 'L', 'R'),
        })
    }
}

#[aoc_generator(day5)]
fn parse_input_day5(input: &str) -> BTreeSet<usize> {
    input
        .lines()
        .filter_map(|line| match Seat::from_str(line) {
            Err(_) => None,
            Ok(seat) => Some(seat.id()),
        })
        .collect()
}

#[aoc(day5, part1)]
fn part1(input: &BTreeSet<usize>) -> usize {
    *input.iter().max().expect("no seats provided")
}

#[aoc(day5, part2)]
fn part2(input: &BTreeSet<usize>) -> Option<usize> {
    input
        .iter()
        .tuple_windows::<(&usize, &usize)>()
        .find_map(|tuple| {
            let (current, next) = tuple;
            if *current + 2 == *next {
                Some(*current + 1)
            } else {
                None
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bsp_1() {
        assert_eq!(decode_binary_space_partitioning("FBFBBFF", 'F', 'B'), 44);
        assert_eq!(decode_binary_space_partitioning("RLR", 'L', 'R'), 5);
    }
    #[test]
    fn bsp_2() {
        assert_eq!(decode_binary_space_partitioning("BFFFBBF", 'F', 'B'), 70);
        assert_eq!(decode_binary_space_partitioning("RRR", 'L', 'R'), 7);
    }
    #[test]
    fn bsp_3() {
        assert_eq!(decode_binary_space_partitioning("FFFBBBF", 'F', 'B'), 14);
        assert_eq!(decode_binary_space_partitioning("RRR", 'L', 'R'), 7);
    }
    #[test]
    fn bsp_4() {
        assert_eq!(decode_binary_space_partitioning("BBFFBBF", 'F', 'B'), 102);
        assert_eq!(decode_binary_space_partitioning("RLL", 'L', 'R'), 4);
    }
}
