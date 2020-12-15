use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;
use std::fmt::{Display, Formatter, Write};
use core::fmt;
use std::mem::swap;

enum Direction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Direction::North => f.write_char('N')?,
            Direction::South => f.write_char('S')?,
            Direction::East => f.write_char('E')?,
            Direction::West => f.write_char('W')?,
            Direction::Left => f.write_char('L')?,
            Direction::Right => f.write_char('R')?,
            Direction::Forward => f.write_char('F')?,
        }
        Ok(())
    }
}

impl Direction {
    fn from_char(c: char) -> Result<Self, ()> {
        Ok(match c {
            'N' => Direction::North,
            'S' => Direction::South,
            'E' => Direction::East,
            'W' => Direction::West,
            'L' => Direction::Left,
            'R' => Direction::Right,
            'F' => Direction::Forward,
            _ => return Err(()),
        })
    }

    fn rot_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
            _ => unreachable!(),
        }
    }

    fn rot_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
            _ => unreachable!(),
        }
    }
}

struct Action {
    direction: Direction,
    amount: usize,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Action {
            direction: Direction::from_char(s.chars().next().map_or(Err(()), Ok)?)?,
            amount: s[1..].parse().map_err(|_| ())?,
        })
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.direction.fmt(f)?;
        std::fmt::Display::fmt(&self.amount, f)?;
        Ok(())
    }
}

#[aoc_generator(day12)]
fn parse_input_day12(input: &str) -> Vec<Action> {
    input
        .lines()
        .filter_map(|line| Action::from_str(line).map_or(None, Some))
        .collect()
}

#[aoc(day12, part1)]
fn part1(input: &Vec<Action>) -> usize {
    let mut direction = Direction::East;
    let mut position = (0isize, 0isize);

    for action in input {
        let amount = action.amount as isize;
        let action_direction = match action.direction {
            Direction::Forward => &direction,
            _ => &action.direction,
        };
        match action_direction {
            Direction::North => position.0 += amount,
            Direction::South => position.0 -= amount,
            Direction::East => position.1 += amount,
            Direction::West => position.1 -= amount,
            Direction::Left => {
                let turns = amount / 90;
                for _ in 0..turns {
                    direction = direction.rot_left();
                }
            },
            Direction::Right => {
                let turns = amount / 90;
                for _ in 0..turns {
                    direction = direction.rot_right();
                }
            },
            _ => unreachable!(),
        }
    }

    (position.0.abs() + position.1.abs()) as usize
}

type DirectionalAmount = (isize, Direction);
type Waypoint = (DirectionalAmount, DirectionalAmount);

#[aoc(day12, part2)]
fn part2(input: &Vec<Action>) -> usize {
    let mut position = (0isize, 0isize);
    let mut waypoint: Waypoint = (
        (10isize, Direction::East),
        (1isize, Direction::North),
    );

    for action in input {
        let amount = action.amount as isize;
        match action.direction {
            Direction::North => waypoint.1.0 += match waypoint.1.1 {
                Direction::North => amount,
                Direction::South => -amount,
                _ => unreachable!(),
            },
            Direction::South => waypoint.1.0 += match waypoint.1.1 {
                Direction::South => amount,
                Direction::North => -amount,
                _ => unreachable!(),
            },
            Direction::East => waypoint.0.0 += match waypoint.0.1 {
                Direction::East => amount,
                Direction::West => -amount,
                _ => unreachable!(),
            },
            Direction::West => waypoint.0.0 += match waypoint.0.1 {
                Direction::West => amount,
                Direction::East => -amount,
                _ => unreachable!(),
            },
            Direction::Left => {
                let turns = amount / 90;
                for _ in 0..turns {
                    swap(&mut waypoint.0.0, &mut waypoint.1.0);
                    swap(&mut waypoint.0.1, &mut waypoint.1.1);
                    waypoint.0.1 = waypoint.0.1.rot_left();
                    waypoint.1.1 = waypoint.1.1.rot_left();
                }
            },
            Direction::Right => {
                let turns = amount / 90;
                for _ in 0..turns {
                    swap(&mut waypoint.0.0, &mut waypoint.1.0);
                    swap(&mut waypoint.0.1, &mut waypoint.1.1);
                    waypoint.0.1 = waypoint.0.1.rot_right();
                    waypoint.1.1 = waypoint.1.1.rot_right();
                }
            },
            Direction::Forward => {
                position.0 += match waypoint.0.1 {
                    Direction::East => amount * waypoint.0.0,
                    Direction::West => amount * -waypoint.0.0,
                    _ => unreachable!(),
                };
                position.1 += match waypoint.1.1 {
                    Direction::North => amount * waypoint.1.0,
                    Direction::South => amount * -waypoint.1.0,
                    _ => unreachable!(),
                };
            }
        }
        //println!("action={}\tway=({} {}, {} {})\tpos=({}, {})", action, waypoint.0.0, waypoint.0.1, waypoint.1.0, waypoint.1.1, position.0, position.1);
    }

    (position.0.abs() + position.1.abs()) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input_day12(INPUT)), 25);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input_day12(INPUT)), 286);
    }
}
