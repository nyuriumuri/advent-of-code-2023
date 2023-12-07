use itertools::Itertools;
use std::collections::HashMap;
advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let mut card_bid_pairs: Vec<(Hand, u32)> = input
        .trim()
        .lines()
        .map(|l| l.trim())
        .map(|l| l.split_whitespace().next_tuple().unwrap())
        .map(|tup: (&str, &str)| (Hand::from_str_p1(tup.0), tup.1.parse::<u32>().unwrap()))
        .collect();

    card_bid_pairs.sort_by(|a, b| a.0.cmp(&b.0));

    let res = card_bid_pairs
        .iter()
        .enumerate()
        .fold(0, |acc, (i, pair)| ((i as u32 + 1) * pair.1) + acc);

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut card_bid_pairs: Vec<(Hand, u32)> = input
        .trim()
        .lines()
        .map(|l| l.trim())
        .map(|l| l.split_whitespace().next_tuple().unwrap())
        .map(|tup: (&str, &str)| (Hand::from(tup.0), tup.1.parse::<u32>().unwrap()))
        .collect();

    card_bid_pairs.sort_by(|a, b| a.0.cmp(&b.0));

    let res = card_bid_pairs
        .iter()
        .enumerate()
        .fold(0, |acc, (i, pair)| ((i as u32 + 1) * pair.1) + acc);

    Some(res)
}

// Five of a kind, where all five cards have the same label: AAAAA
// Four of a kind, where four cards have the same label and one card has a different label: AA8AA
// Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
// Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
// Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
// One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
// High card, where all cards' labels are distinct: 23456

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOK,
    FullHouse,
    FourOK,
    FiveOK,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy)]
struct Hand {
    hand_type: HandType,
    cards: Cards,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy)]
struct Cards([u8; 5]);

impl Hand {
    fn from_str_p1(value: &str) -> Self {
        let char_to_val = |x: char| -> u8 {
            if let Some(d) = x.to_digit(10) {
                return d as u8;
            }

            match x {
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => unimplemented!(),
            }
        };
        let cards: Vec<u8> = value.trim().chars().map(char_to_val).collect();

        let mut map = HashMap::new();

        for card in &cards {
            map.entry(card).and_modify(|v| *v += 1).or_insert(1u8);
        }

        let mut occ = map.into_values().collect::<Vec<u8>>();
        occ.sort();

        let hand_type = match occ.len() {
            1 => HandType::FiveOK,
            2 => match occ[..] {
                [1, 4] => HandType::FourOK,
                [2, 3] => HandType::FullHouse,
                _ => unimplemented!(),
            },
            3 => match occ[..] {
                [1, 1, 3] => HandType::ThreeOK,
                [1, 2, 2] => HandType::TwoPair,
                _ => unimplemented!(),
            },

            4 => HandType::OnePair,

            5 => HandType::HighCard,

            _ => unimplemented!(),
        };

        Hand {
            cards: Cards(cards.try_into().expect("Vec size isn't 5")),
            hand_type,
        }
    }
}

// part 2
impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let char_to_val = |x: char| -> u8 {
            if let Some(d) = x.to_digit(10) {
                return d as u8;
            }

            match x {
                'J' => 1,
                'T' => 10,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => unimplemented!(),
            }
        };
        let cards: Vec<u8> = value.trim().chars().map(char_to_val).collect();

        let mut map = HashMap::new();

        let jokers: u8 = cards.iter().filter(|&&c| c == 1).count() as u8;

        let mapped_cards: Vec<&u8> = cards.iter().filter(|&&c| c != 1).collect();

        for card in &mapped_cards {
            map.entry(card).and_modify(|v| *v += 1).or_insert(1u8);
        }

        let mut occ = map.into_values().collect::<Vec<u8>>();
        occ.sort();

        occ.last_mut()
            .map(|v| {
                *v += jokers;
            })
            .or_else(|| {
                occ.push(jokers);
                Some(())
            });

        let hand_type = match occ.len() {
            1 => HandType::FiveOK,
            2 => match occ[..] {
                [1, 4] => HandType::FourOK,
                [2, 3] => HandType::FullHouse,
                _ => unimplemented!(),
            },
            3 => match occ[..] {
                [1, 1, 3] => HandType::ThreeOK,
                [1, 2, 2] => HandType::TwoPair,
                _ => unimplemented!(),
            },

            4 => HandType::OnePair,

            5 => HandType::HighCard,

            _ => unimplemented!(),
        };

        Hand {
            cards: Cards(cards.try_into().expect("Vec size isn't 5")),
            hand_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
