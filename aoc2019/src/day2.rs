use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<usize> {
    input.split(",").map(|l| l.parse().unwrap_or(0)).collect()
}

pub fn execute_intcode(program: &mut [usize]) {
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
}

pub fn run_program_with_no_args(input: &[usize]) -> usize {
    let mut program = input.to_vec();
    execute_intcode(&mut program);
    program[0]
}

pub fn run_program_with_args(input: &[usize], arg1: usize, arg2: usize) -> usize {
    let mut program = input.to_vec();
    program[1] = arg1;
    program[2] = arg2;
    execute_intcode(&mut program);
    program[0]
}

#[aoc(day2, part1)]
pub fn part1(input: &[usize]) -> usize {
    run_program_with_args(input, 12, 2)
}

#[aoc(day2, part2)]
pub fn part2(input: &[usize]) -> usize {
    let goal = 19690720;
    for noun in 0..100 {
        for verb in 0..100 {
            let result = run_program_with_args(input, noun, verb);
            if result == goal {
                return noun * 100 + verb
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(run_program_with_no_args(&[1, 0, 0, 0, 99]), 2);
        assert_eq!(run_program_with_no_args(&[2, 3, 0, 3, 99]), 2);
        assert_eq!(run_program_with_no_args(&[2, 4, 4, 5, 99, 0]), 2);
        assert_eq!(run_program_with_no_args(&[1, 1, 1, 4, 99, 5, 6, 0, 99]), 30);
        // Supplementary test.
        assert_eq!(run_program_with_no_args(&[2, 4, 4, 0, 99]), 9801);
    }
}
