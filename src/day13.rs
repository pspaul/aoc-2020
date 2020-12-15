use aoc_runner_derive::{aoc, aoc_generator};

struct Bus {
    offset: usize,
    interval: usize,
}

struct Notes {
    earliest: usize,
    busses: Vec<Bus>,
}

#[aoc_generator(day13)]
fn parse_input_day13(input: &str) -> Notes {
    let lines = input.lines().collect::<Vec<&str>>();
    let earliest = lines[0].parse().unwrap();
    let busses = lines[1]
        .split(",")
        .enumerate()
        .filter_map(|(offset, bus)| {
            Some(Bus {
                offset,
                interval: bus.parse().ok()?,
            })
        })
        .collect();
    Notes { earliest, busses }
}

#[aoc(day13, part1)]
fn part1(notes: &Notes) -> usize {
    let mut t = notes.earliest;
    loop {
        for bus in &notes.busses {
            if t % bus.interval == 0 {
                return bus.interval * (t - notes.earliest);
            }
        }
        t += 1;
    }
}

#[aoc(day13, part2)]
fn part2(notes: &Notes) -> usize {
    let mut t = 0;
    let mut common_interval = 1;

    for bus in &notes.busses {
        while (t + bus.offset) % bus.interval != 0 {
            // increase time in steps that is a multiple of all previous bus intervals
            t += common_interval;
        }
        // make the common interval include the current bus interval
        common_interval *= bus.interval;
    }

    t
}

fn extended_euclid(a: isize, b: isize) -> (isize, isize, isize) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (d_, s_, t_) = extended_euclid(b, a % b);
    let (d, s, t) = (d_, t_, s_ - (a / b) * t_);
    // d = s * a + t * b
    return (d, s, t);
}

fn chinese_remainder_theorem(equations: &Vec<(isize, isize)>) -> usize {
    // least common multiple of moduli
    let lcm = equations.iter().map(|(m, _x)| m).product::<isize>();

    let solution = equations
        .iter()
        .map(|(eq_m, eq_x)| {
            let m = lcm / eq_m;
            // modular inverse of m
            let e = extended_euclid(m, *eq_m).1;
            // partial solution for equation
            e * m * eq_x
        })
        .sum::<isize>();

    solution.rem_euclid(lcm) as usize
}

#[aoc(day13, part2, equations)]
fn part2_equations(notes: &Notes) -> usize {
    let equations = notes
        .busses
        .iter()
        .map(|bus| {
            let m = bus.interval as isize;
            let x = (-(bus.offset as isize)).rem_euclid(m);
            // t mod m ≡ x
            (m, x)
        })
        .collect();

    chinese_remainder_theorem(&equations)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input_day13(INPUT)), 295);
    }

    #[test]
    fn part2_crt_test() {
        assert_eq!(chinese_remainder_theorem(&vec![(3, 2), (4, 3), (5, 2)]), 47);
    }

    #[test]
    fn part2_equations_example() {
        assert_eq!(part2_equations(&parse_input_day13(INPUT)), 1068781);
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(part2(&parse_input_day13(INPUT)), 1068781);
    }

    #[test]
    fn part2_example_2() {
        assert_eq!(part2(&parse_input_day13("0\n17,x,13,19")), 3417);
    }

    #[test]
    fn part2_example_3() {
        assert_eq!(part2(&parse_input_day13("0\n67,7,59,61")), 754018);
    }

    #[test]
    fn part2_example_4() {
        assert_eq!(part2(&parse_input_day13("0\n67,x,7,59,61")), 779210);
    }

    #[test]
    fn part2_example_5() {
        assert_eq!(part2(&parse_input_day13("0\n67,7,x,59,61")), 1261476);
    }

    #[test]
    fn part2_example_6() {
        assert_eq!(part2(&parse_input_day13("0\n1789,37,47,1889")), 1202161486);
    }
}
