use super::computer::*;
use rayon::prelude::*;
use std::str::FromStr;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(|value| isize::from_str(value).unwrap())
        .collect()
}

#[aoc(day2, part1)]
fn part1(inpute: &Vec<isize>) -> isize {
    let mut input = inpute.clone();
    input[1] = 12;
    input[2] = 2;
    let mut computer = Computer {
        ip: 0,
        memory: input,
        inputs: None.into_iter(),
    };
    computer.run().unwrap();
    computer.memory[0]
}

#[aoc(day2, part2)]
fn part2(inpute: &Vec<isize>) -> isize {
    (0isize..10000)
        .into_par_iter()
        .find_map_any(|x| {
            let mut input = inpute.clone();
            input[1] = x / 100;
            input[2] = x % 100;
            let mut computer = Computer {
                ip: 0,
                memory: input,
                inputs: None.into_iter(),
            };
            if computer.run().is_ok() {
                if computer.memory[0] == 19690720 {
                    Some(x)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &[isize] = &[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    const INPUT2: &[isize] = &[1, 0, 0, 0, 99];
    const INPUT3: &[isize] = &[2, 3, 0, 3, 99];
    const INPUT4: &[isize] = &[2, 4, 4, 5, 99, 0];
    const INPUT5: &[isize] = &[1, 1, 1, 4, 99, 5, 6, 0, 99];

    #[test]
    fn run() {
        let input1 = INPUT1.to_owned();
        let mut computer = Computer {
            ip: 0,
            memory: input1,
            inputs: None.into_iter(),
        };
        computer.run().expect("Error while running program");
        assert_eq!(
            computer.memory,
            [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50].to_owned()
        );

        let input2 = INPUT2.to_owned();
        let mut computer = Computer {
            ip: 0,
            memory: input2,
            inputs: None.into_iter(),
        };
        computer.run().expect("Error while running program");
        assert_eq!(computer.memory, &[2, 0, 0, 0, 99].to_owned());

        let input3 = INPUT3.to_owned();
        let mut computer = Computer {
            ip: 0,
            memory: input3,
            inputs: None.into_iter(),
        };
        computer.run().expect("Error while running program");
        assert_eq!(computer.memory, &[2, 3, 0, 6, 99].to_owned());

        let input4 = INPUT4.to_owned();
        let mut computer = Computer {
            ip: 0,
            memory: input4,
            inputs: None.into_iter(),
        };
        computer.run().expect("Error while running program");
        assert_eq!(computer.memory, [2, 4, 4, 5, 99, 9801].to_owned());

        let input5 = INPUT5.to_owned();
        let mut computer = Computer {
            ip: 0,
            memory: input5,
            inputs: None.into_iter(),
        };
        computer.run().expect("Error while running program");
        assert_eq!(computer.memory, [30, 1, 1, 4, 2, 5, 6, 0, 99].to_owned());
    }
}
