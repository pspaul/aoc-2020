use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day15)]
fn parse_input_day15(input: &str) -> Vec<usize> {
    input.split(',').map(|num| num.parse().unwrap()).collect()
}

fn find_nth_number(numbers: &Vec<usize>, n: usize) -> usize {
    let mut map = numbers
        .iter()
        .map(|n| (*n, (0, 0)))
        .collect::<HashMap<usize, (usize, usize)>>();
    let mut last = 0;
    for turn in 1..=n {
        if let Some(num) = numbers.get(turn - 1) {
            last = *num;
            map.insert(last, (turn, 0));
        } else {
            if let Some(last_turns) = map.get(&last) {
                if last_turns.1 == 0 {
                    // it was the first time the number was spoken
                    last = 0;
                } else {
                    last = last_turns.0 - last_turns.1;
                }
            } else {
                // the number was never spoken before
                last = 0;
            }
            // shift the last 2 turns, starting with (0, 0)
            let last_turns = map.get(&last).unwrap_or(&(0, 0));
            map.insert(last, (turn, last_turns.0));
        }
    }
    last
}

#[aoc(day15, part1)]
fn part1(numbers: &Vec<usize>) -> usize {
    find_nth_number(numbers, 2020)
}

#[aoc(day15, part2)]
fn part2(numbers: &Vec<usize>) -> usize {
    find_nth_number(numbers, 30_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        let input = "0,3,6";
        assert_eq!(part1(&parse_input_day15(input)), 436);
    }

    #[test]
    fn part1_example_2() {
        let input = "1,3,2";
        assert_eq!(part1(&parse_input_day15(input)), 1);
    }

    #[test]
    fn part1_example_3() {
        let input = "2,1,3";
        assert_eq!(part1(&parse_input_day15(input)), 10);
    }

    #[test]
    fn part1_example_4() {
        let input = "1,2,3";
        assert_eq!(part1(&parse_input_day15(input)), 27);
    }

    #[test]
    fn part1_example_5() {
        let input = "2,3,1";
        assert_eq!(part1(&parse_input_day15(input)), 78);
    }

    #[test]
    fn part1_example_6() {
        let input = "3,2,1";
        assert_eq!(part1(&parse_input_day15(input)), 438);
    }

    #[test]
    fn part1_example_7() {
        let input = "3,1,2";
        assert_eq!(part1(&parse_input_day15(input)), 1836);
    }

    #[test]
    fn part2_example_1() {
        let input = "0,3,6";
        assert_eq!(part2(&parse_input_day15(input)), 175594);
    }

    #[test]
    fn part2_example_2() {
        let input = "1,3,2";
        assert_eq!(part2(&parse_input_day15(input)), 2578);
    }

    #[test]
    fn part2_example_3() {
        let input = "2,1,3";
        assert_eq!(part2(&parse_input_day15(input)), 3544142);
    }

    #[test]
    fn part2_example_4() {
        let input = "1,2,3";
        assert_eq!(part2(&parse_input_day15(input)), 261214);
    }

    #[test]
    fn part2_example_5() {
        let input = "2,3,1";
        assert_eq!(part2(&parse_input_day15(input)), 6895259);
    }

    #[test]
    fn part2_example_6() {
        let input = "3,2,1";
        assert_eq!(part2(&parse_input_day15(input)), 18);
    }

    #[test]
    fn part2_example_7() {
        let input = "3,1,2";
        assert_eq!(part2(&parse_input_day15(input)), 362);
    }
}
