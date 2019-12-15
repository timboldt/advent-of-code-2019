use aoc_runner_derive::{aoc, aoc_generator};

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
    masses.iter().map(|m| part2_fuel(*m)).sum()
}

pub fn part2_fuel(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel <= 0 {
        0
    } else {
        fuel + part2_fuel(fuel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(part1(&[12]), 2);
        assert_eq!(part1(&[14]), 2);
        assert_eq!(part1(&[1969]), 654);
        assert_eq!(part1(&[100756]), 33583);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2(&[12]), 2);
        assert_eq!(part2(&[14]), 2);
        assert_eq!(part2(&[1969]), 966);
        assert_eq!(part2(&[100756]), 50346);
    }
}
