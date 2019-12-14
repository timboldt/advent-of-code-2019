use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(masses: &[i32]) -> i32 {
    masses.iter().map(|m| m / 3 - 2).sum()
}

#[aoc(day1, part2)]
pub fn part2(masses: &[i32]) -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&[12]), 2);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&[14]), 2);
    }

    #[test]
    fn part1_example3() {
        assert_eq!(part1(&[1969]), 654);
    }

    #[test]
    fn part1_example4() {
        assert_eq!(part1(&[100756]), 33583);
    }
}
