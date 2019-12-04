use rayon::prelude::*;
use std::str::FromStr;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> (usize, usize) {
    let mut range = input.split('-');
    let a = usize::from_str(range.next().unwrap()).unwrap();
    let b = usize::from_str(range.next().unwrap()).unwrap();
    (a, b)
}

fn criteria(mut x: usize) -> bool {
    let mut has_double = false;
    let mut prev = 10;
    for _ in 0..6 {
        let next_digit = x % 10;
        if next_digit > prev {
            return false;
        } else if next_digit == prev {
            has_double = true
        }
        prev = next_digit;
        x /= 10;
    }
    has_double
}

fn criteria2(mut x: usize) -> bool {
    let mut has_double = false;
    let mut nb_repeats = 1;
    let mut prev = 10;
    for _ in 0..7 {
        let next_digit = x % 10;
        if next_digit > prev {
            return false;
        } else if next_digit == prev {
            nb_repeats += 1;
        } else {
            if nb_repeats == 2 {
                has_double = true;
            }
            nb_repeats = 1;
        }
        prev = next_digit;
        x /= 10;
    }
    has_double
}

#[aoc(day4, part1)]
fn part1(input: &(usize, usize)) -> usize {
    (input.0..=input.1)
        .into_par_iter()
        .filter(|&x| criteria(x))
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &(usize, usize)) -> usize {
    (input.0..=input.1)
        .into_par_iter()
        .filter(|&x| criteria2(x))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: usize = 111111;
    const INPUT_2: usize = 223450;
    const INPUT_3: usize = 123789;

    const INPUT_4: usize = 112233;
    const INPUT_5: usize = 123444;
    const INPUT_6: usize = 112222;

    #[test]
    fn criteria1_test() {
        assert!(criteria(INPUT_1));
        assert!(!criteria(INPUT_2));
        assert!(!criteria(INPUT_3));

        assert!(criteria(INPUT_4));
        assert!(criteria(INPUT_5));
        assert!(criteria(INPUT_6));
    }

    #[test]
    fn criteria2_test() {
        assert!(!criteria2(INPUT_1));
        assert!(!criteria2(INPUT_2));
        assert!(!criteria2(INPUT_3));

        assert!(criteria2(INPUT_4));
        assert!(!criteria2(INPUT_5));
        assert!(criteria2(INPUT_6));
    }
}
