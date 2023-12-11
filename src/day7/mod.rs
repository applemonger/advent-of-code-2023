use aocd::*;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseHandError;

impl FromStr for HandType {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Convert string to count of cards
        let mut hand_map: HashMap<char, usize> =
            s.chars()
                .into_iter()
                .fold(HashMap::<char, usize>::new(), |mut m, x| {
                    *m.entry(x).or_default() += 1;
                    m
                });

        // Get 'J' counts
        let joker_count = if let Some(n) = hand_map.remove(&'J') {
            n
        } else {
            0
        };

        // Exception: Hand is all J's
        if hand_map.is_empty() {
            hand_map.insert('J', 0);
        }

        // Find card type with most counts
        let max_card = hand_map
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _v)| k)
            .unwrap()
            .clone();

        // Add jokers to card type with most counts
        *hand_map.get_mut(&max_card).unwrap() += joker_count;

        // Get counts
        let counts: Vec<usize> = hand_map.values().cloned().collect();

        // Determine type
        let number_of_distinct_cards = counts.len();
        let max_count: usize = *counts.iter().max().unwrap();
        match (number_of_distinct_cards, max_count) {
            (1, 5) => Ok(Self::FiveOfAKind),
            (2, 4) => Ok(Self::FourOfAKind),
            (2, 3) => Ok(Self::FullHouse),
            (3, 3) => Ok(Self::ThreeOfAKind),
            (3, 2) => Ok(Self::TwoPair),
            (4, 2) => Ok(Self::OnePair),
            (5, 1) => Ok(Self::HighCard),
            _ => unreachable!("Impossible hand"),
        }
    }
}

#[derive(Debug)]
struct Play {
    hand_type: HandType,
    hand: String,
    bid: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePlayError;

impl FromStr for Play {
    type Err = ParsePlayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hand_regex = Regex::new(r"^(.{5}) ").unwrap();
        let hand = hand_regex.captures(s).unwrap()[1].to_string();
        let bid_regex = Regex::new(r" (\d*)$").unwrap();
        let bid: u32 = bid_regex.captures(s).unwrap()[1]
            .to_string()
            .parse()
            .unwrap();
        Ok(Play {
            hand_type: HandType::from_str(hand.as_str()).unwrap(),
            hand,
            bid,
        })
    }
}

impl Play {
    pub fn encode_hand(&self) -> (HandType, u8, u8, u8, u8, u8) {
        let encoded_cards: Vec<u8> = self
            .hand
            .chars()
            .into_iter()
            .map(|c| match &c {
                'J' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'T' => 10,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => unreachable!("Unreachable card rank."),
            })
            .collect();

        (
            self.hand_type,
            encoded_cards[0],
            encoded_cards[1],
            encoded_cards[2],
            encoded_cards[3],
            encoded_cards[4],
        )
    }
}

#[aocd(2023, 7)]
pub fn solution2() {
    // Get hands
    let mut hands: Vec<Play> = input!()
        .split('\n')
        .map(|s| Play::from_str(s).unwrap())
        .collect();

    // Sort
    hands.sort_by_key(|hand| hand.encode_hand());

    // Get bid-product
    let total: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum();

    submit!(2, total);
}

/* Part 1 Solution
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseHandError;

impl FromStr for HandType {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Convert string to count of cards
        let hand_map: HashMap<char, usize> = s.chars()
            .into_iter()
            .fold(HashMap::<char, usize>::new(), |mut m, x| {
                *m.entry(x).or_default() += 1;
                m
            });

        // Get counts
        let counts: Vec<usize> = hand_map.values().cloned().collect();

        // Determine type
        let number_of_distinct_cards = counts.len();
        let max_count: usize = *counts.iter().max().unwrap();
        match (number_of_distinct_cards, max_count) {
            (1, 5) => Ok(Self::FiveOfAKind),
            (2, 4) => Ok(Self::FourOfAKind),
            (2, 3) => Ok(Self::FullHouse),
            (3, 3) => Ok(Self::ThreeOfAKind),
            (3, 2) => Ok(Self::TwoPair),
            (4, 2) => Ok(Self::OnePair),
            (5, 1) => Ok(Self::HighCard),
            _ => unreachable!("Impossible hand")
        }
    }
}

#[derive(Debug)]
struct Play {
    hand_type: HandType,
    hand: String,
    bid: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePlayError;

impl FromStr for Play {
    type Err = ParsePlayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hand_regex = Regex::new(r"^(.{5}) ").unwrap();
        let hand = hand_regex.captures(s).unwrap()[1].to_string();
        let bid_regex = Regex::new(r" (\d*)$").unwrap();
        let bid: u32 = bid_regex.captures(s).unwrap()[1].to_string().parse().unwrap();
        Ok(Play {
            hand_type: HandType::from_str(hand.as_str()).unwrap(),
            hand,
            bid
        })
    }
}

impl Play {
    pub fn encode_hand(&self) -> (HandType, u8, u8, u8, u8, u8) {
        let encoded_cards: Vec<u8> = self.hand.chars()
            .into_iter()
            .map(|c| {
                match &c {
                    '2' => 2,
                    '3' => 3,
                    '4' => 4,
                    '5' => 5,
                    '6' => 6,
                    '7' => 7,
                    '8' => 8,
                    '9' => 9,
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => unreachable!("Unreachable card rank.")
                }
            })
            .collect();

        (
            self.hand_type,
            encoded_cards[0],
            encoded_cards[1],
            encoded_cards[2],
            encoded_cards[3],
            encoded_cards[4],
        )
    }
}


#[aocd(2023, 7)]
pub fn solution1() {
    // Get hands
    let mut hands: Vec<Play> = input!()
        .split('\n')
        .map(|s| Play::from_str(s).unwrap())
        .collect();

    // Sort
    hands.sort_by_key(|hand| hand.encode_hand());

    // Get bid-product
    let total: u32 = hands.iter()
        .enumerate()
        .map(|(i, hand)| {
            (i as u32 + 1) * hand.bid
        })
        .sum();

    submit!(1, total);
}
*/
