use std::str::FromStr;
use rayon::prelude::*;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|value| usize::from_str(value).unwrap())
        .collect()
}

fn run_code(input: &mut [usize]) {
    let mut pos = 0;
    let end = input.len();
    while pos < end {
        match input[pos] {
            1 => input[input[pos+3]] = input[input[pos+1]] + input[input[pos+2]],
            2 => input[input[pos+3]] = input[input[pos+1]] * input[input[pos+2]],
            99 => pos = end,
            _ => panic!("Error encountered"),
        }
        pos += 4;
    }
}

#[aoc(day2, part1)]
fn part1(inpute:  &Vec<usize>) -> usize {
    let mut input= inpute.clone();
    input[1] = 12;
    input[2] = 2;
    run_code(&mut input);
    input[0]
}


#[aoc(day2, part2)]
fn part2(inpute:  &Vec<usize>) -> usize {
    (0usize..10000).into_par_iter()
        .find_map_any(|x| {
            let mut input = inpute.clone();
            input[1] = x /100;
            input[2] = x % 100;
            run_code(&mut input);
            if input[0] == 19690720 {
                Some(x)
            } else { None }
        }).unwrap()
}



#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &[usize] = &[1,9,10,3,2,3,11,0,99,30,40,50];
    const INPUT2: &[usize] = &[1,0,0,0,99];
    const INPUT3: &[usize] = &[2,3,0,3,99];
    const INPUT4: &[usize] = &[2,4,4,5,99,0];
    const INPUT5: &[usize] = &[1,1,1,4,99,5,6,0,99];

    #[test]
    fn run() {
        let mut input1 = INPUT1.to_owned();
        run_code(&mut input1);
        assert_eq!(input1, &[3500,9,10,70,2,3,11,0,99,30,40,50]);

        let mut input2 = INPUT2.to_owned();
        run_code(&mut input2);
        assert_eq!(input2, &[2,0,0,0,99]);

        let mut input3 = INPUT3.to_owned();
        run_code(&mut input3);
        assert_eq!(input3, &[2,3,0,6,99]);

        let mut input4 = INPUT4.to_owned();
        run_code(&mut input4);
        assert_eq!(input4, &[2,4,4,5,99,9801]);

        let mut input5 = INPUT5.to_owned();
        run_code(&mut input5);
        assert_eq!(input5, &[30,1,1,4,2,5,6,0,99]);

    }
}


