use aoc_runner_derive::{aoc, aoc_generator};

struct Group {
    size: usize,
    answers: [usize; 26],
}

impl Group {
    fn new() -> Self {
        Group {
            size: 0,
            answers: [0; 26],
        }
    }
}

#[aoc_generator(day6)]
fn parse_input_day6(input: &str) -> Vec<Group> {
    input
        .split("\n\n")
        .map(|s| {
            let mut group = Group::new();
            for line in s.lines() {
                group.size += 1;
                for c in line.chars() {
                    group.answers[c as usize - 'a' as usize] += 1;
                }
            }
            group
        })
        .collect()
}

#[aoc(day6, part1)]
fn part1(input: &Vec<Group>) -> usize {
    input
        .iter()
        .map(|group| group.answers.iter().filter(|answers| **answers > 0).count())
        .sum()
}

#[aoc(day6, part2)]
fn part2(input: &Vec<Group>) -> usize {
    input
        .iter()
        .map(|group| {
            group
                .answers
                .iter()
                .filter(|answers| **answers == group.size)
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input_day6(INPUT)), 11);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input_day6(INPUT)), 6);
    }
}
