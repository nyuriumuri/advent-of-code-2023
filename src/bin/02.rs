advent_of_code::solution!(2);

use lazy_static::lazy_static;
use regex::Regex;
pub fn part_one(input: &str) -> Option<u32> {
    let max = ColorData {
        red: 12,
        green: 13,
        blue: 14,
    };

    let res: u32 = input
        .split_terminator('\n')
        .map(GameData::from)
        .filter(|c| {
            c.colors.red <= max.red && c.colors.blue <= max.blue && c.colors.green <= max.green
        })
        .map(|g| g.id)
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let res: u32 = input
        .split_terminator('\n')
        .map(GameData::from)
        .map(to_power)
        .sum();

    Some(res)
}

fn to_power(game: GameData) -> u32 {
    game.colors.red * game.colors.blue * game.colors.green
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
struct ColorData {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
struct GameData {
    id: u32,
    colors: ColorData,
}

impl From<&str> for GameData {
    fn from(value: &str) -> Self {
        lazy_static! {
            static ref GAME_RE: Regex = Regex::new(r"Game\s(\d+)").unwrap();
        }
        let id = GAME_RE.captures(value).unwrap()[1]
            .parse::<u32>()
            .unwrap_or(0);
        GameData {
            id,
            colors: ColorData::from(value),
        }
    }
}

impl From<&str> for ColorData {
    fn from(value: &str) -> Self {
        lazy_static! {
            static ref RED_RE: Regex = Regex::new(r"(\d+)\sred").unwrap();
            static ref BLUE_RE: Regex = Regex::new(r"(\d+)\sblue").unwrap();
            static ref GREEN_RE: Regex = Regex::new(r"(\d+)\sgreen").unwrap();
        }

        let red = RED_RE
            .captures_iter(value)
            .map(|c| c[1].parse::<u32>().unwrap())
            .max()
            .unwrap_or(0);

        let blue = BLUE_RE
            .captures_iter(value)
            .map(|c| c[1].parse::<u32>().unwrap())
            .max()
            .unwrap_or(0);

        let green = GREEN_RE
            .captures_iter(value)
            .map(|c| c[1].parse::<u32>().unwrap())
            .max()
            .unwrap_or(0);

        ColorData { red, green, blue }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_colors_from_line() {
        let result = ColorData::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(
            ColorData {
                red: 4,
                blue: 6,
                green: 2
            },
            result
        )
    }

    #[test]
    fn test_game_data_from_line() {
        let result = GameData::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(
            GameData {
                id: 1,
                colors: ColorData {
                    red: 4,
                    blue: 6,
                    green: 2
                }
            },
            result
        );

        let result = GameData::from("Game 36: 1 blue, 9 red, 2 green; 11 red, 3 blue, 2 green; 2 green, 6 red; 8 green, 11 red, 3 blue; 4 green, 7 blue, 11 red; 9 green");
        assert_eq!(
            GameData {
                id: 36,
                colors: ColorData {
                    red: 11,
                    blue: 7,
                    green: 9
                }
            },
            result
        )
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
