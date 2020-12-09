use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day9)]
fn parse_input_day9(input: &str) -> Vec<usize> {
    input
        .lines()
        .filter_map(|line| line.parse().map_or(None, Some))
        .collect::<Vec<usize>>()
}

struct CipherState {
    size: usize,
    window: Vec<usize>,
    lookup: HashSet<usize>,
}

impl CipherState {
    fn with_size(size: usize) -> Self {
        CipherState {
            size,
            window: Vec::with_capacity(size),
            lookup: HashSet::with_capacity(size),
        }
    }

    fn push(&mut self, n: usize) {
        if self.window.len() == self.size {
            let evicted = self.window.remove(0);
            self.lookup.remove(&evicted);
        }
        self.window.push(n);
        self.lookup.insert(n);
    }

    fn check(&self, n: &usize) -> bool {
        if self.window.len() < self.size {
            return true;
        }
        for candidate in self.window.iter() {
            if candidate > n {
                continue;
            }
            let remaining = n - candidate;
            if remaining != *candidate && self.lookup.contains(&remaining) {
                return true;
            }
        }
        false
    }
}

fn find_first_bad_num(input: &Vec<usize>, preamble_size: usize) -> Option<usize> {
    let mut state = CipherState::with_size(preamble_size);
    for n in input {
        if state.check(n) {
            state.push(*n);
        } else {
            return Some(*n);
        }
    }
    None
}

#[aoc(day9, part1)]
fn part1(input: &Vec<usize>) -> Option<usize> {
    find_first_bad_num(input, 25)
}

fn find_first_bad_num_sliced(input: &Vec<usize>, preamble_size: usize) -> Option<usize> {
    let mut lookup = HashSet::with_capacity(preamble_size);
    for (i, n) in input.iter().enumerate() {
        if i >= preamble_size {
            let window = &input[(i - preamble_size)..i];
            if window.iter().find(|m| **m <= *n && lookup.contains(&(*n - **m))).is_none() {
                return Some(*n);
            }

            lookup.remove(&input[i - preamble_size]);
        }
        lookup.insert(*n);
    }
    None
}

#[aoc(day9, part1, sliced)]
fn part1_sliced(input: &Vec<usize>) -> Option<usize> {
    find_first_bad_num_sliced(input, 25)
}

fn find_weakness(input: &Vec<usize>, preamble_size: usize) -> Option<usize> {
    let bad_num = find_first_bad_num(input, preamble_size)?;
    for (i, n) in input.iter().enumerate() {
        if *n == bad_num {
            return None;
        }
        let mut counter = *n;
        for (j, m) in input.iter().enumerate().skip(i + 1) {
            if *m == bad_num {
                break;
            }
            counter += m;
            if counter == bad_num {
                let contiguous = &input[i..=j];
                let min = contiguous.iter().min().unwrap();
                let max = contiguous.iter().max().unwrap();
                return Some(min + max);
            } else if counter > bad_num {
                break;
            }
        }
    }
    None
}

#[aoc(day9, part2)]
fn part2(input: &Vec<usize>) -> Option<usize> {
    find_weakness(input, 25)
}

fn find_weakness_sliding_window(input: &Vec<usize>, preamble_size: usize) -> Option<usize> {
    let bad_num = find_first_bad_num_sliced(input, preamble_size)?;

    let mut window_start = 0;
    let mut window_sum = 0;
    for (window_end, n) in input.iter().enumerate() {
        window_sum += n;
        while window_sum > bad_num {
            window_sum -= input[window_start];
            window_start += 1;
        }
        if window_sum == bad_num {
            let window = &input[window_start..=window_end];
            let min = window.iter().min().unwrap();
            let max = window.iter().max().unwrap();
            return Some(min + max);
        }
    }
    None
}

#[aoc(day9, part2, sliding_window)]
fn part2_sliding_window(input: &Vec<usize>) -> Option<usize> {
    find_weakness_sliding_window(input, 25)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn part1_example() {
        assert_eq!(find_first_bad_num(&parse_input_day9(INPUT), 5), Some(127));
    }

    #[test]
    fn part1_sliced_example() {
        assert_eq!(find_first_bad_num_sliced(&parse_input_day9(INPUT), 5), Some(127));
    }

    #[test]
    fn part2_example() {
        assert_eq!(find_weakness(&mut parse_input_day9(INPUT), 5), Some(62));
    }
    #[test]
    fn part2_sliding_window_example() {
        assert_eq!(find_weakness_sliding_window(&mut parse_input_day9(INPUT), 5), Some(62));
    }
}

