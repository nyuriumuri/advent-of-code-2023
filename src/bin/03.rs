advent_of_code::solution!(3);
use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    lazy_static! {
        static ref NUM_RE: Regex = Regex::new(r"\d+").unwrap();
        static ref SYM_RE: Regex = Regex::new(r"[^\d\.]").unwrap();
    }

    let line_len = input.find(char::is_whitespace).unwrap();
    let input =
        &(".".repeat(line_len) + &input.replace(char::is_whitespace, "") + &".".repeat(line_len));

    let res: u32 = NUM_RE
        .captures_iter(input)
        .map(|c| {
            let m = c.get(0).unwrap();
            let start = m.start();
            let end = m.end();

            let left = if start % line_len != 0 {
                start - 1
            } else {
                start
            };

            let right = if end % line_len < line_len - 1 {
                end
            } else {
                end - 1
            };

            let valid = SYM_RE.is_match(&input[left..left + 1])
                || SYM_RE.is_match(&input[right..right + 1])
                || SYM_RE.is_match(&input[left - line_len..right - line_len + 1])
                || SYM_RE.is_match(&input[left + line_len..right + line_len + 1]);
            match valid {
                true => c[0].parse::<u32>().expect("Couldn't convert to u32"),
                false => 0u32,
            }
        })
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    lazy_static! {
        static ref NUM_RE: Regex = Regex::new(r"\d+").unwrap();
        static ref GEAR_RE: Regex = Regex::new(r"\*").unwrap();
    }

    let mut map: HashMap<usize, Vec<u32>> = HashMap::new();
    let line_len = input.find(char::is_whitespace).unwrap();
    let input =
        &(".".repeat(line_len) + &input.replace(char::is_whitespace, "") + &".".repeat(line_len));

    NUM_RE.captures_iter(input).for_each(|c| {
        let m = c.get(0).unwrap();
        let start = m.start();
        let end = m.end();

        let left = if start % line_len != 0 {
            start - 1
        } else {
            start
        };

        let right = if end % line_len < line_len - 1 {
            end
        } else {
            end - 1
        };

        let val = c[0].parse().unwrap();
        [
            (&input[left..left + 1], left),
            (&input[right..right + 1], right),
            (
                &input[left - line_len..right - line_len + 1],
                left - line_len,
            ),
            (
                &input[left + line_len..right + line_len + 1],
                left + line_len,
            ),
        ]
        .iter()
        .for_each(|&s| {
            GEAR_RE
                .find_iter(s.0)
                .for_each(|m| match map.get_mut(&(s.1 + m.start())) {
                    Some(v) => v.push(val),
                    None => {
                        map.insert(s.1 + m.start(), vec![val]);
                    }
                });
        })
    });

    let res: u32 = map
        .values()
        .filter(|&v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
