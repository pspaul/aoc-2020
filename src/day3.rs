use aoc_runner_derive::{aoc, aoc_generator};
use std::ops::Add;

#[derive(Eq, PartialEq)]
enum Cell {
    Open,
    Tree,
}

impl From<char> for Cell {
    fn from(symbol: char) -> Self {
        match symbol {
            '.' => Cell::Open,
            '#' => Cell::Tree,
            _ => unreachable!()
        }
    }
}

struct Map {
    height: usize,
    width: usize,
    cells: Vec<Vec<Cell>>,
}

impl Map {
    fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        if !(0..self.height).contains(&y) {
            return None;
        }
        let cell = self.cells.get(y)?.get(x % self.width)?;
        Some(cell)
    }
}

#[aoc_generator(day3)]
fn parse_input_day3(input: &str) -> Map {
    let cells: Vec<Vec<Cell>> = input
        .lines()
        .map(|line| line.chars().map(Cell::from).collect())
        .collect();

    Map {
        height: cells.len(),
        width: cells[0].len(),
        cells,
    }
}

#[derive(Clone)]
struct Point2D {
    x: usize,
    y: usize,
}

impl Point2D {
    fn new(x: usize, y: usize) -> Self {
        Point2D { x, y }
    }
}

impl Add for &Point2D {
    type Output = Point2D;

    fn add(self, rhs: Self) -> Self::Output {
        Point2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct MapIterator<'a> {
    map: &'a Map,
    current: Point2D,
    slope: Point2D,
}

impl MapIterator<'_> {
    fn slope(mut self, dx: usize, dy: usize) -> Self {
        self.slope = Point2D::new(dx, dy);
        self
    }
}

impl<'a> IntoIterator for &'a Map {
    type Item = &'a Cell;
    type IntoIter = MapIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MapIterator {
            map: &self,
            current: Point2D::new(0, 0),
            slope: Point2D::new(1, 1),
        }
    }
}

impl<'a> Iterator for MapIterator<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<&'a Cell> {
        self.current = &self.current + &self.slope;
        if self.current.y >= self.map.height {
            return None;
        }
        Some(self.map.get(self.current.x, self.current.y).unwrap())
    }
}

fn count_trees_on_slope(map: &Map, slope_dx: usize, slope_dy: usize) -> usize {
    map
        .into_iter()
        .slope(slope_dx, slope_dy)
        .filter(|c| **c == Cell::Tree)
        .count()
}

#[aoc(day3, part1)]
fn part1(input: &Map) -> usize {
    count_trees_on_slope(input, 3, 1)
}

#[aoc(day3, part2)]
fn part2(input: &Map) -> usize {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes
        .iter()
        .map(|slope| count_trees_on_slope(input, slope.0, slope.1))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn part1_example() {
        let output = 7;
        assert_eq!(part1(&parse_input_day3(INPUT)), output);
    }

    #[test]
    fn part2_example() {
        let output = 336;
        assert_eq!(part2(&parse_input_day3(INPUT)), output);
    }
}
