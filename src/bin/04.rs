advent_of_code::solution!(4);

use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NUM_RE: Regex = Regex::new(r"\d+").unwrap();
    static ref GAME_NUM_RE: Regex = Regex::new(r"Card\s*(\d+)").unwrap();
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split_terminator('\n')
            .map(get_num_winning_in_line)
            .map(|n| match n {
                0 => 0,
                _ => 2u32.pow(n - 1),
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut game_map: HashMap<u32, u32> = HashMap::new();
    let res: u32 = input
        .split_terminator('\n')
        .map(|s| {
            let game_num = GAME_NUM_RE.captures(s).unwrap()[1].parse::<u32>().unwrap();
            let num_winning = get_num_winning_in_line(s);
            let num_copies = *game_map.get(&game_num).unwrap_or(&0) + 1;

            for i in game_num + 1..game_num + num_winning + 1 {
                match game_map.get_mut(&i) {
                    Some(x) => *x += num_copies,
                    None => {
                        game_map.insert(i, num_copies);
                    }
                };
            }
            num_copies
        })
        .sum();

    Some(res)
}

fn get_num_winning_in_line(input: &str) -> u32 {
    let input = &input[input.find(':').unwrap()..];
    let split_pos = input.find('|').unwrap();
    let winning_nums = &input[..split_pos];
    let owned_nums = &input[split_pos..];

    let mut winning_set = HashSet::new();
    NUM_RE.captures_iter(winning_nums).for_each(|c| {
        winning_set.insert(c[0].to_string());
    });

    NUM_RE
        .captures_iter(owned_nums)
        .filter(|c| winning_set.contains(&c[0]))
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
