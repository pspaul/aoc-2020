use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashSet;
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
            if a + b == SUM {
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

#[aoc(day1, part1, generic)]
fn part1_generic(input: &[usize]) -> Option<usize> {
    return part_generic(input, 2);
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

#[aoc(day1, part2, generic)]
fn part2_generic(input: &[usize]) -> Option<usize> {
    return part_generic(input, 3);
}

fn part_generic(input: &[usize], k: usize) -> Option<usize> {
    let set = input.iter().map(|&n| n).collect::<HashSet<usize>>();

    input
        .iter()
        .permutations(k - 1)
        .find(|n_tuple| {
            let mut sum = 0;
            for &&n in n_tuple {
                sum += n;
            }
            return sum <= SUM && set.contains(&(SUM - sum));
        })
        .map_or(None, |n_tuple| {
            let sum = n_tuple.iter().map(|&&n| n).sum::<usize>();
            let product = n_tuple.iter().map(|&&n| n).product::<usize>();
            Some(product * (SUM - sum))
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::ops::Range;

    #[test]
    fn part1_example() {
        let input = &[1721, 979, 366, 299, 675, 1456];
        let output = 514579;
        assert_eq!(part1_ctf_style(input).unwrap(), output);
        assert_eq!(part1(input).unwrap(), output);
        assert_eq!(part1_generic(input).unwrap(), output);
    }
    #[test]
    fn part2_example() {
        let input = &[1721, 979, 366, 299, 675, 1456];
        let output = 241861950;
        assert_eq!(part2_ctf_style(input).unwrap(), output);
        assert_eq!(part2(input).unwrap(), output);
        assert_eq!(part2_generic(input).unwrap(), output);
    }

    #[test]
    fn test_generic() {
        let mut input = vec![];
        let mut rng = rand::thread_rng();
        let r: Range<usize> = 0..256;
        for _ in r {
            let num = rng.gen_range(200, 2019) + 1 as usize;
            input.push(num);
        }
        input.append(vec![69, 187, 420, 1337, 7, 4, 3].as_mut());
        if let Some(solution) = part_generic(input.as_slice(), 6) {
            println!("Solution: {}", solution);
        } else {
            println!("No solution found");
        }
    }
}
