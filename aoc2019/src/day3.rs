use aoc_runner_derive::{aoc};
use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point(i32, i32);

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub struct WireSegment {
    direction: Direction,
    distance: usize,
}

#[derive(Debug, PartialEq)]
pub struct Wire(Vec<WireSegment>);

impl FromStr for WireSegment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dir_str = String::from(s);
        let dist_str = dir_str.split_off(1);
        let dir = match dir_str.chars().next().unwrap_or('*') {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        };
        let dist = dist_str.parse()?;

        Ok(WireSegment {
            direction: dir,
            distance: dist,
        })
    }
}

pub fn manhattan(pt: &Point) -> usize {
    (pt.0.abs() + pt.1.abs()) as usize
}

pub fn get_next_point(start: Point, direction: Direction) -> Point {
    match direction {
        Direction::Up => Point(start.0, start.1 + 1),
        Direction::Down => Point(start.0, start.1 - 1),
        Direction::Left => Point(start.0 - 1, start.1),
        Direction::Right => Point(start.0 + 1, start.1),
    }
}

pub fn get_points_for_wire(wire: Wire) -> HashSet<Point> {
    let mut visited = HashSet::new();
    let mut current = Point(0, 0);
    for segment in wire.0.iter() {
        for _ in 0..segment.distance {
            current = get_next_point(current, segment.direction);
            visited.insert(current);
        }
    }
    visited
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let wire1 = Wire(
        lines
            .next()
            .unwrap_or("")
            .split(",")
            .map(|n| WireSegment::from_str(n).unwrap())
            .collect(),
    );
    let wire2 = Wire(
        lines
            .next()
            .unwrap_or("")
            .split(",")
            .map(|n| WireSegment::from_str(n).unwrap())
            .collect(),
    );

    let pts1 = get_points_for_wire(wire1);
    let pts2 = get_points_for_wire(wire2);
    let intersect = pts1.intersection(&pts2);
    let closest = intersect.min_by(|a, b| manhattan(a).cmp(&manhattan(b))).unwrap();
    manhattan(closest)
}

// #[aoc(day3, part2)]
// pub fn part2(input: &[usize]) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(
            WireSegment::from_str("U99").unwrap(),
            WireSegment {
                direction: Direction::Up,
                distance: 99
            }
        );
        assert_eq!(
            WireSegment::from_str("D1").unwrap(),
            WireSegment {
                direction: Direction::Down,
                distance: 1
            }
        );
        assert_eq!(
            WireSegment::from_str("L12345").unwrap(),
            WireSegment {
                direction: Direction::Left,
                distance: 12345
            }
        );
        assert_eq!(
            WireSegment::from_str("R0").unwrap(),
            WireSegment {
                direction: Direction::Right,
                distance: 0
            }
        );
    }
}
