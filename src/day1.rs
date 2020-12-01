use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

const SUM: usize = 2020;

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Result<Vec<usize>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day1, part1, ctf_style)]
fn part1_ctf_style(input: &[usize]) -> Option<usize> {
    for &a in input {
        for &b in input {
            if a + b  == SUM {
                return Some(a * b);
            }
        }
    }
    return None;
}

#[aoc(day1, part1)]
fn part1(input: &[usize]) -> Option<usize> {
    const BUCKETS_SIZE: usize = SUM + 1;
    let mut buckets: [bool; BUCKETS_SIZE] = [false; BUCKETS_SIZE];
    for &n in input {
        let match_index = SUM - n;
        if buckets[match_index] {
            return Some(match_index * n);
        } else {
            buckets[n] = true;
        }
    }
    return None;
}

#[aoc(day1, part2, ctf_style)]
fn part2_ctf_style(input: &[usize]) -> Option<usize> {
    for &a in input {
        for &b in input {
            for &c in input {
                if a + b + c == SUM {
                    return Some(a * b * c);
                }
            }
        }
    }
    return None;
}

#[aoc(day1, part2)]
fn part2(input: &[usize]) -> Option<usize> {
    const BUCKETS_SIZE: usize = (SUM + 1) * 2;
    let mut buckets: [Option<usize>; BUCKETS_SIZE] = [None; BUCKETS_SIZE];
    for (i, &a) in input.iter().enumerate() {
        for &b in input.iter().skip(i) {
            buckets[a + b] = Some(a * b);
        }
    }

    for &n in input {
        let match_index = SUM - n;
        if match_index > SUM {
            continue;
        }
        if let Some(intermediate_product) = buckets[match_index] {
            return Some(intermediate_product * n);
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = &[1721, 979, 366, 299, 675, 1456];
        let output = 514579;
        assert_eq!(part1_ctf_style(input).unwrap(), output);
        assert_eq!(part1(input).unwrap(), output);
    }
    #[test]
    fn part2_example() {
        let input = &[1721, 979, 366, 299, 675, 1456];
        let output = 241861950;
        assert_eq!(part2_ctf_style(input).unwrap(), output);
        assert_eq!(part2(input).unwrap(), output);
    }
}
