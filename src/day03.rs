use rayon::prelude::*;
use std::str::FromStr;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> (Vec<(WireLine, usize)>, Vec<(WireLine, usize)>) {
    let mut lines = input.lines().map(|l| l.split(','));
    let mut origin = Point { x: 0, y: 0 };
    let mut len = 0;
    let first_wire = lines
        .next()
        .unwrap()
        .map(|twist| parse_wireline(&mut origin, &mut len, twist))
        .collect();
    origin = Point { x: 0, y: 0 };
    len = 0;
    let second_wire = lines
        .next()
        .unwrap()
        .map(|twist| parse_wireline(&mut origin, &mut len, twist))
        .collect();
    (first_wire, second_wire)
}

fn parse_wireline(current_pos: &mut Point, len: &mut usize, input: &str) -> (WireLine, usize) {
    let (direction, value) = input.split_at(1);
    let value = isize::from_str(value).unwrap();
    let previous_len = *len;
    *len += value as usize;
    match direction.chars().next().unwrap() {
        'R' => {
            let result = WireLine::Hori(HLine {
                going_east: true,
                ordinate: current_pos.y,
                west: current_pos.x,
                east: current_pos.x + value,
            });
            current_pos.x += value;
            (result, previous_len)
        }
        'L' => {
            let result = WireLine::Hori(HLine {
                going_east: false,
                ordinate: current_pos.y,
                west: current_pos.x - value,
                east: current_pos.x,
            });
            current_pos.x -= value;
            (result, previous_len)
        }
        'U' => {
            let result = WireLine::Vert(VLine {
                going_north: true,
                abscissa: current_pos.x,
                south: current_pos.y,
                north: current_pos.y + value,
            });
            current_pos.y += value;
            (result, previous_len)
        }
        'D' => {
            let result = WireLine::Vert(VLine {
                going_north: false,
                abscissa: current_pos.x,
                south: current_pos.y - value,
                north: current_pos.y,
            });
            current_pos.y -= value;
            (result, previous_len)
        }
        _ => panic!("unexpected input {}", input),
    }
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn manhattan_distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

/// A vertical line, requiring south < north
#[derive(Clone, Copy, Debug)]
pub struct VLine {
    going_north: bool,
    abscissa: isize,
    south: isize,
    north: isize,
}

impl VLine {
    fn distance_to(&self, p: Point) -> usize {
        if self.going_north {
            (p.y - self.south) as usize
        } else {
            (self.north - p.y) as usize
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HLine {
    going_east: bool,
    ordinate: isize,
    west: isize,
    east: isize,
}

impl HLine {
    fn distance_to(&self, p: Point) -> usize {
        if self.going_east {
            (p.x - self.west) as usize
        } else {
            (self.east - p.x) as usize
        }
    }
}

fn cross_hvline(hline: &HLine, vline: &VLine) -> Option<(Point, usize)> {
    if hline.ordinate >= vline.south && hline.ordinate <= vline.north {
        if vline.abscissa >= hline.west && vline.abscissa <= hline.east {
            let intersect = Point {
                y: hline.ordinate,
                x: vline.abscissa,
            };
            return Some((
                intersect,
                hline.distance_to(intersect) + vline.distance_to(intersect),
            ));
        }
    }
    None
}

#[derive(Clone, Copy, Debug)]
pub enum WireLine {
    Vert(VLine),
    Hori(HLine),
}

impl WireLine {
    fn cross(&self, other: &Self) -> Option<(Point, usize)> {
        match (self, other) {
            (Self::Hori(l1), Self::Vert(l2)) => cross_hvline(l1, l2),
            (Self::Hori(_), Self::Hori(_)) => None,
            (Self::Vert(_), Self::Vert(_)) => None,
            (Self::Vert(l1), Self::Hori(l2)) => cross_hvline(l2, l1),
        }
        .filter(|&(p, _)| p.x != 0 || p.y != 0)
    }
}

#[aoc(day3, part1)]
fn part1(input: &(Vec<(WireLine, usize)>, Vec<(WireLine, usize)>)) -> usize {
    let len0 = input.0.len();
    let len1 = input.1.len();
    (0..len0 * len1)
        .into_par_iter()
        .filter_map(|n| {
            let p0 = n / len1;
            let p1 = n % len1;
            input.0[p0]
                .0
                .cross(&input.1[p1].0)
                .map(|(p, _)| p.manhattan_distance())
        })
        .min()
        .unwrap()
}

#[aoc(day3, part2)]
fn part2(input: &(Vec<(WireLine, usize)>, Vec<(WireLine, usize)>)) -> usize {
    let len0 = input.0.len();
    let len1 = input.1.len();
    (0..len0 * len1)
        .into_par_iter()
        .filter_map(|n| {
            let p0 = n / len1;
            let p1 = n % len1;
            input.0[p0]
                .0
                .cross(&input.1[p1].0)
                .map(|(_, l)| l + input.0[p0].1 + input.1[p1].1)
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "R8,U5,L5,D3\nU7,R6,D4,L4";
    const INPUT2: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    const INPUT3: &str =
        "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    #[test]
    fn part1_test() {
        assert_eq!(part1(&input_generator(&INPUT1)), 6);
        assert_eq!(part1(&input_generator(&INPUT2)), 159);
        assert_eq!(part1(&input_generator(&INPUT3)), 135);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(&input_generator(&INPUT1)), 30);
        assert_eq!(part2(&input_generator(&INPUT2)), 610);
        assert_eq!(part2(&input_generator(&INPUT3)), 410);
    }
}
