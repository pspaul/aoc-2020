use aoc_runner_derive::{aoc, aoc_generator};

struct PasswordInfo {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

macro_rules! split {
    ( $target:expr, $separator:expr ) => {{
        let tmp = $target.split($separator).collect::<Vec<&str>>();
        (tmp[0], tmp[1])
    }};
}

#[aoc_generator(day2)]
fn parse_input_day2(input: &str) -> Vec<PasswordInfo> {
    input
        .lines()
        .map(|l| {
            let (policy, password) = split!(l, ": ");
            let (range, letter) = split!(policy, " ");
            let (min, max) = split!(range, "-");
            return PasswordInfo {
                min: min.parse().expect("min must be usize"),
                max: max.parse().expect("max must be usize"),
                letter: letter.parse().expect("letter must be a char"),
                password: password.to_string(),
            };
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[PasswordInfo]) -> usize {
    input
        .iter()
        .filter(|item| {
            let count = item
                .password
                .chars()
                .into_iter()
                .filter(|&c| c == item.letter)
                .count();
            return (item.min..=item.max).contains(&count);
        })
        .count()
}

#[aoc(day2, part2)]
fn part2(input: &[PasswordInfo]) -> usize {
    input
        .iter()
        .filter(|item| {
            let first = item.password.chars().nth(item.min - 1);
            let second = item.password.chars().nth(item.max - 1);
            let (first, second) = match (first, second) {
                (Some(first), Some(second)) => (first, second),
                _ => return false,
            };
            return (first == item.letter) ^ (second == item.letter);
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = [
            PasswordInfo {
                min: 1,
                max: 3,
                letter: 'a',
                password: "abcde".to_string(),
            },
            PasswordInfo {
                min: 1,
                max: 3,
                letter: 'b',
                password: "cdefg".to_string(),
            },
            PasswordInfo {
                min: 2,
                max: 9,
                letter: 'c',
                password: "ccccccccc".to_string(),
            },
        ];
        let output = 2;
        assert_eq!(part1(&input), output);
    }

    #[test]
    fn part2_example() {
        let input = [
            PasswordInfo {
                min: 1,
                max: 3,
                letter: 'a',
                password: "abcde".to_string(),
            },
            PasswordInfo {
                min: 1,
                max: 3,
                letter: 'b',
                password: "cdefg".to_string(),
            },
            PasswordInfo {
                min: 2,
                max: 9,
                letter: 'c',
                password: "ccccccccc".to_string(),
            },
        ];
        let output = 1;
        assert_eq!(part2(&input), output);
    }
}
