advent_of_code::solution!(5);

use std::{cmp::Ordering, thread};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MAP_RE: Regex =
        Regex::new(r"(?<dest_start>\d+)\s+(?<source_start>\d+)\s+(?<range>\d+)").unwrap();
    static ref DIGIT_RE: Regex = Regex::new(r"\d+").unwrap();
    static ref TITLE_RE: Regex = Regex::new(r".*:").unwrap();
    static ref SEEDS_PART2_RE: Regex = Regex::new(r"(?<start>\d+)\s+(?<range>\d+)").unwrap();
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

pub fn part_two(input: &str) -> Option<usize> {
    let input = input.trim();
    let (seeds_line, input) = input.split_once('\n').unwrap();
    let layers: Vec<MapLayer> = TITLE_RE.split(input).map(MapLayer::from).collect();

    let convert_seed = |seed: SeedRange, layers: &Vec<MapLayer>| {
        layers
            .iter()
            .fold(vec![seed], |acc, layer: &MapLayer| {
                acc.iter()
                    .flat_map(|s| layer.convert_range(s.clone()))
                    .collect()
            })
            .iter()
            .map(|s| s.start)
            .min()
            .unwrap()
    };

    let thread_handles: Vec<thread::JoinHandle<usize>> = SEEDS_PART2_RE
        .captures_iter(seeds_line)
        .map(|c| {
            let start = c["start"].parse::<usize>().unwrap();
            let range = c["range"].parse::<usize>().unwrap();
            let seed = SeedRange {
                start,
                end: start + range - 1,
            };
            let layers = layers.clone();
            thread::spawn(move || convert_seed(seed, &layers))
        })
        .collect();

    let res = thread_handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .min()
        .unwrap();
    Some(res)
    // None
}

#[derive(Debug, Clone)]
struct MapLayer(Vec<Map>);

#[derive(Debug, Clone, Copy)]
struct Map {
    source_start: usize,
    dest_start: usize,
    range: usize,
}

#[derive(Debug, Clone)]
struct SeedRange {
    start: usize,
    // range: usize,
    end: usize,
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
        let value = value.trim();
        MapLayer(value.lines().map(Map::from).collect())
    }
}

impl Map {
    fn split_range(&self, s: &SeedRange) -> Vec<SeedRange> {
        // splits the original rangev (potentiall partially overlapping range) into multiple ranges
        // which either completely overlap or avoid the range the Map maps over.
        let m_end = self.source_start + self.range - 1;
        match (
            s.start.cmp(&self.source_start),
            s.end.cmp(&self.source_start),
            s.start.cmp(&m_end),
            s.end.cmp(&m_end),
        ) {
            (Ordering::Greater | Ordering::Equal, _, Ordering::Less, Ordering::Greater) => {
                vec![
                    SeedRange {
                        start: s.start,
                        end: m_end,
                    },
                    SeedRange {
                        start: m_end + 1,
                        end: s.end,
                    },
                ]
            }

            (Ordering::Less, Ordering::Greater, _, Ordering::Less | Ordering::Equal) => {
                vec![
                    SeedRange {
                        start: s.start,
                        end: self.source_start - 1,
                    },
                    SeedRange {
                        start: self.source_start,
                        end: s.end,
                    },
                ]
            }
            (Ordering::Less, _, _, Ordering::Greater) => {
                vec![
                    SeedRange {
                        start: s.start,
                        end: self.source_start - 1,
                    },
                    SeedRange {
                        start: self.source_start,
                        end: m_end,
                    },
                    SeedRange {
                        start: m_end + 1,
                        end: s.end,
                    },
                ]
            }

            _ => vec![s.clone()],
        }
    }

    fn convert_range(&self, s: &SeedRange) -> Option<SeedRange> {
        if s.start >= self.source_start && s.end <= self.source_start + self.range - 1 {

            Some(SeedRange {
                start: self.dest_start + s.start - self.source_start,
                end: s.end - self.source_start + self.dest_start,
            })
        } else {
            None
        }
    }
}

impl MapLayer {
    fn convert(&self, input: u32) -> u32 {
        for map in &self.0 {
            if (map.source_start..map.source_start + map.range).contains(&(input as usize)) {
                return (map.dest_start + (input as usize - map.source_start)) as u32;
            }
        }
        input
    }

    fn convert_range(&self, input: SeedRange) -> Vec<SeedRange> {
        let splitted = self.0.iter().fold(vec![input], |acc, m| {
            acc.into_iter()
                .flat_map(|seed| m.split_range(&seed))
                .collect()
        });

        // we split over all the maps in the layer before we start converting to avoid
        // re-converting/re-splitting ranges
        //
        let helper_fn = |s: SeedRange| {
            for map in &self.0 {
                if let Some(new_range) = map.convert_range(&s) {
                    return new_range;
                }
            }
            s
        };

        splitted.into_iter().map(helper_fn).collect()
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
        assert_eq!(result, Some(46));
    }
}
