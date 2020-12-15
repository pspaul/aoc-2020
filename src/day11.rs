use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt::{Display, Formatter, Write};
use core::fmt;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl Display for Seat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Seat::Floor => f.write_char('.'),
            Seat::Empty => f.write_char('L'),
            Seat::Occupied => f.write_char('#'),
        }
    }
}

impl Seat {
    fn from_char(s: &char) -> Result<Self, ()> {
        Ok(match s {
            '.' => Seat::Floor,
            'L' => Seat::Empty,
            '#' => Seat::Occupied,
            _ => return Err(()),
        })
    }
}

type Seats = Vec<Vec<Seat>>;

#[aoc_generator(day11)]
fn parse_input_day11(input: &str) -> Seats {
    input
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|c| Seat::from_char(&c).unwrap())
                .collect()
        })
        .collect()
}

fn count_adjacent_occupied_seats(seats: &Seats, y: usize, x: usize) -> usize {
    const OFFSETS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut counter = 0;
    for (dx, dy) in OFFSETS.iter() {
        if let Some(current_y) = (y as i32).checked_add(*dy) {
            let current_y = current_y as usize;
            if !(0..seats.len()).contains(&current_y) {
                continue
            }
            if let Some(current_x) = (x as i32).checked_add(*dx) {
                let current_x = current_x as usize;
                if !(0..seats[current_y].len()).contains(&current_x) {
                    continue;
                }
                if seats[current_y][current_x] == Seat::Occupied {
                    counter += 1;
                }
            }
        }
    }
    counter
}

fn simulate_round(seats: &Seats, count_occupied_seats: fn(&Seats, usize, usize) -> usize, occupied_threshold: usize) -> (Vec<Vec<Seat>>, bool) {
    let mut result = Vec::new();
    let mut changed = false;
    for (y, row) in seats.iter().enumerate() {
        let mut result_row = Vec::new();
        for (x, seat) in row.iter().enumerate() {
            let adjacent_occupied = count_occupied_seats(seats, y, x);
            if *seat == Seat::Empty && adjacent_occupied == 0 {
                result_row.push(Seat::Occupied);
                changed = true;
            } else if *seat == Seat::Occupied && adjacent_occupied >= occupied_threshold {
                result_row.push(Seat::Empty);
                changed = true;
            } else {
                result_row.push(*seat);
            }
        }
        result.push(result_row);
    }
    (result, changed)
}

#[aoc(day11, part1)]
fn part1(seats: &Seats) -> usize {
    let mut current_seats = seats.clone();
    loop {
        let (round_seats, changed) = simulate_round(&current_seats, count_adjacent_occupied_seats, 4);
        if !changed {
            break;
        }
        current_seats = round_seats;
    }

    current_seats.iter().flatten().filter(|seat| **seat == Seat::Occupied).count()
}

fn count_visible_occupied_seats(seats: &Seats, y: usize, x: usize) -> usize {
    const OFFSETS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    OFFSETS
        .iter()
        .map(|(dx, dy)| {
            let mut factor = 1;
            loop {
                if let Some(current_y) = (y as i32).checked_add(*dy * factor) {
                    let current_y = current_y as usize;
                    if !(0..seats.len()).contains(&current_y) {
                        return 0;
                    }
                    if let Some(current_x) = (x as i32).checked_add(*dx * factor) {
                        let current_x = current_x as usize;
                        if !(0..seats[current_y].len()).contains(&current_x) {
                            return 0;
                        }

                        match seats[current_y][current_x] {
                            Seat::Occupied => return 1,
                            Seat::Empty => return 0,
                            _ => {},
                        }
                    }
                }
                factor += 1;
            }
        })
        .sum()
}

#[aoc(day11, part2)]
fn part2(seats: &Seats) -> usize {
    let mut current_seats = seats.clone();
    loop {
        let (round_seats, changed) = simulate_round(&current_seats, count_visible_occupied_seats, 5);
        if !changed {
            break;
        }
        current_seats = round_seats;
    }

    current_seats.iter().flatten().filter(|seat| **seat == Seat::Occupied).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input_day11(INPUT)), 37);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input_day11(INPUT)), 26);
    }
}
