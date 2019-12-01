use std::str::FromStr;
use rayon::prelude::*;

#[aoc(day1, part1)]
fn part1(input: &str) -> usize {
    input
        .par_lines()
        .map(|l| {
                let mass = usize::from_str(l).unwrap();
                mass / 3 - 2
            })
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &str) -> usize {
    input
        .par_lines()
        .map(|l| {
                let mut mass = usize::from_str(l).unwrap();
                let mut fuel = 0;
                while mass > 8 {
                    mass = mass / 3 - 2;
                    fuel += mass;
                }
                fuel
             })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
12
14
1969
100756
";
    const FUEL_1: [usize; 4] = [2, 2, 654, 33583];
    const FUEL_2: [usize; 4] = [2, 2, 966, 50346];

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), FUEL_1.iter().sum())
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), FUEL_2.iter().sum())
    }
}
