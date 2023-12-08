advent_of_code::solution!(8);
use std::collections::HashMap;

use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;

lazy_static! {
    static ref MAP_RE: Regex =
        Regex::new(r"(?<Location>\w+)\s*=\s*\((?<Left>\w+)\s*,\s*(?<Right>\w+)\)").unwrap();
}
pub fn part_one(input: &str) -> Option<u64> {
    let (instructions, map_str) = input.split_once('\n').unwrap();
    let instructions = instructions.trim();
    let map: HashMap<String, (String, String)> = MAP_RE
        .captures_iter(map_str)
        .map(|c| {
            (
                c["Location"].to_owned(),
                (c["Left"].to_owned(), c["Right"].to_owned()),
            )
        })
        .collect();

    let mut cur = &"AAA".to_owned();
    let zzz = "ZZZ";
    let steps = instructions
        .chars()
        .cycle()
        .take_while(|&dir| {
            let next = match dir {
                'R' => &map.get(cur).unwrap().1,
                'L' => &map.get(cur).unwrap().0,
                _ => unimplemented!(),
            };
            cur = next;
            cur != zzz
        })
        .count() as u64
        + 1;
    Some(steps)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (instructions, map_str) = input.split_once('\n').unwrap();
    let instructions = instructions.trim();
    let map: HashMap<String, (String, String)> = MAP_RE
        .captures_iter(map_str)
        .map(|c| {
            (
                c["Location"].to_owned(),
                (c["Left"].to_owned(), c["Right"].to_owned()),
            )
        })
        .collect();

    let res = map
        .keys()
        .filter(|key| key.ends_with('A'))
        .par_bridge()
        .map(|start: &String| {
            let mut cur = start.clone();
            let cloned_map = map.clone();
            instructions
                .chars()
                .cycle()
                .take_while(move |dir| {
                    cur = match dir {
                        'R' => cloned_map.get(&cur).unwrap().1.clone(),
                        'L' => cloned_map.get(&cur).unwrap().0.clone(),
                        _ => unimplemented!(),
                    };
                    !cur.ends_with('Z')
                })
                .count() as u64
                + 1
        })
        .reduce(|| 1, lcm);

    Some(res)
    // None
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}
#[cfg(test)]
mod tests {
    use std::{env, fs};

    use super::*;

    #[test]
    fn test_part_one() {
        let cwd = env::current_dir().unwrap();
        let filepath = cwd.join("data").join("examples").join("08-1.txt");
        let f = fs::read_to_string(filepath).unwrap();
        let result = part_one(f.as_str());
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
