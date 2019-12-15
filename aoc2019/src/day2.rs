use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<usize> {
    input.split(",").map(|l| l.parse().unwrap_or(0)).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[usize]) -> usize {
    let mut program = input.to_vec();
    let mut mem_ptr = 0;
    loop {
        let op = program[mem_ptr];
        match op {
            1 => {
                let src1 = program[mem_ptr + 1];
                let src2 = program[mem_ptr + 2];
                let dest = program[mem_ptr + 3];
                program[dest] = program[src1] + program[src2];
                mem_ptr += 4;
            }
            2 => {
                let src1 = program[mem_ptr + 1];
                let src2 = program[mem_ptr + 2];
                let dest = program[mem_ptr + 3];
                program[dest] = program[src1] * program[src2];
                mem_ptr += 4;
            }
            99 => break,
            _ => unreachable!(),
        }
    }
    program[0]
}

// #[aoc(day2, part2)]
// pub fn part2(masses: &[usize]) -> usize {
//     masses.iter().map(|m| part2_fuel(*m)).sum()
// }

// pub fn run_op(op_index: usize, program: &mut [usize]) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(part1(&[1, 0, 0, 0, 99]), 2);
        assert_eq!(part1(&[2, 3, 0, 3, 99]), 2);
        assert_eq!(part1(&[2, 4, 4, 5, 99, 0]), 2);
        assert_eq!(part1(&[1, 1, 1, 4, 99, 5, 6, 0, 99]), 30);
        // Supplementary test.
        assert_eq!(part1(&[2, 4, 4, 0, 99]), 9801);
    }

    // #[test]
    // fn part2_examples() {
    //     assert_eq!(part2(&[12]), 2);
    //     assert_eq!(part2(&[14]), 2);
    //     assert_eq!(part2(&[1969]), 966);
    //     assert_eq!(part2(&[100756]), 50346);
    // }
}
