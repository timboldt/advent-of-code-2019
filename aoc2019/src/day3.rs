use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
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

    println!("Wire2 {:?}", wire2);
    42
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
