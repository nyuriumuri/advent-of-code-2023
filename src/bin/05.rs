advent_of_code::solution!(5);

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MAP_RE: Regex =
        Regex::new(r"(?<dest_start>\d+)\s+(?<source_start>\d+)\s+(?<range>\d+)").unwrap();
    static ref DIGIT_RE: Regex = Regex::new(r"\d+").unwrap();
    static ref TITLE_RE: Regex = Regex::new(r".*:").unwrap();
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.trim();
    let (seeds_line, input) = input.split_once('\n').unwrap();

    let seeds: Vec<u32> = DIGIT_RE
        .captures_iter(seeds_line)
        .map(|c| c[0].parse::<u32>().unwrap())
        .collect();

    let layers: Vec<MapLayer> = TITLE_RE.split(input).map(MapLayer::from).collect();

    let convert_seed = |seed: u32| {
        layers
            .iter()
            .fold(seed, |acc, layer: &MapLayer| layer.convert(acc))
    };

    let res = seeds
        .into_iter()
        .map(convert_seed)
        .min()
        .expect("Seeds is empty");
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Debug, Clone)]
struct MapLayer(Vec<Map>);

#[derive(Debug, Clone, Copy)]
struct Map {
    source_start: usize,
    dest_start: usize,
    range: usize,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let caps = MAP_RE.captures(value).expect("Invalid Map String");
        Map {
            source_start: caps["source_start"].parse::<usize>().unwrap(),
            dest_start: caps["dest_start"].parse::<usize>().unwrap(),
            range: caps["range"].parse::<usize>().unwrap(),
        }
    }
}

impl From<&str> for MapLayer {
    fn from(value: &str) -> Self {
        // let (_title, nums) = value.split_once("|\n").unwrap();
        let value = value.trim();
        MapLayer(value.lines().map(Map::from).collect())
    }
}

impl MapLayer {
    fn convert(&self, input: u32) -> u32 {
        for map in &self.0 {
            if (map.source_start..=map.source_start + map.range).contains(&(input as usize)) {
                return (map.dest_start + (input as usize - map.source_start)) as u32;
            }
        }
        input
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
