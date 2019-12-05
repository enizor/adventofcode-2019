use super::computer::*;
use std::str::FromStr;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(|value| isize::from_str(value).unwrap())
        .collect()
}

#[aoc(day5, part1)]
pub fn part1(input: &Vec<isize>) -> isize {
    let mut computer = Computer {
        ip: 0,
        memory: input.clone(),
        inputs: std::iter::once(1),
        outputs: Vec::new(),
    };
    computer.run().expect("Error while running program")
}

#[aoc(day5, part2)]
pub fn part2(input: &Vec<isize>) -> isize {
    let mut computer = Computer {
        ip: 0,
        memory: input.clone(),
        inputs: std::iter::once(5),
        outputs: Vec::new(),
    };
    computer.run().expect("Error while running program")
}
