use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    let res = input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| {
                    s.parse::<i32>()
                        .unwrap_or_else(|_| panic!("{} is not a number", s))
                })
                .rev()
                .collect()
        })
        .map(get_next)
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<i32> {
    let res = input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| {
                    s.parse::<i32>()
                        .unwrap_or_else(|_| panic!("{} is not a number", s))
                })
                // .rev()
                .collect()
        })
        .map(get_next)
        .sum();
    Some(res)
}
fn get_next(input: Vec<i32>) -> i32 {
    if let (Some(0) | None, Some(0) | None) = (input.last(), input.first()) {
        return 0;
    }
    let end = *input.first().unwrap_or(&0);

    let diffs: Vec<i32> = input
        .into_iter()
        .tuple_windows()
        .map(|(a, b)| a - b)
        .collect();
    end + get_next(diffs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
