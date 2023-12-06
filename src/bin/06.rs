advent_of_code::solution!(6);

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref DIGIT_RE: Regex = Regex::new(r"\d+").unwrap();
}
pub fn part_one(input: &str) -> Option<u32> {
    let (time_str, dist_str) = input.split_once('\n').unwrap();
    let res: u32 = DIGIT_RE
        .captures_iter(time_str)
        .map(|c| c[0].parse::<u32>().unwrap())
        .zip(
            DIGIT_RE
                .captures_iter(dist_str)
                .map(|c| c[0].parse::<u32>().unwrap()),
        )
        .map(|pair| Race {
            time: pair.0,
            distance: pair.1,
        })
        .map(|race| race.solve())
        .product();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Debug, Clone, Copy)]
struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn solve(&self) -> u32 {
        let b = self.time as f32;
        let c = -(self.distance as f32);
        let root = f32::sqrt(b.powf(2f32) + 4f32 * c);
        let x1 = (root - b) / -2f32;
        let x2 = (-root - b) / -2f32;

        let x1 = if x1.fract() == 0f32 {
            x1 as u32 + 1
        } else {
            x1.ceil() as u32
        };

        let x2 = if x2.fract() == 0f32 {
            x2 as u32 - 1
        } else {
            x2.floor() as u32
        };

        x2 - x1 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
