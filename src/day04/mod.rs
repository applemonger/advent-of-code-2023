use regex::Regex;
use std::{collections::HashSet, fs};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Card {
    winning_numbers: Vec<u8>,
    card_numbers: Vec<u8>,
    id: u8,
}

impl Card {
    pub fn extract_from_line(line: &str) -> Card {
        // Extract id
        let id_regex = Regex::new(r"Card\s+(\d+):").unwrap();
        let captures = id_regex.captures(line).unwrap();
        let id: u8 = captures[1].parse().unwrap();

        // Replace spaces with commas
        let space_regex = Regex::new(r"( +)").unwrap();
        let line = space_regex.replace_all(line, ",").to_string();

        // Cleanly format
        let comma_regex = Regex::new(r":,").unwrap();
        let line = comma_regex.replace_all(line.as_str(), ":").to_string();
        let pipe_regex = Regex::new(r",\|,").unwrap();
        let line = pipe_regex.replace_all(line.as_str(), "|").to_string();
        let end_regex = Regex::new(r",$").unwrap();
        let line = end_regex.replace_all(line.as_str(), "").to_string();

        // Extract winning numbers
        let win_regex = Regex::new(r":([\d|,]+)\|").unwrap();
        let winning_numbers = win_regex.captures(line.as_str()).unwrap();
        let winning_numbers: Vec<u8> = winning_numbers[1]
            .split(',')
            .map(|s| {
                let num: u8 = s.parse().unwrap();
                num
            })
            .collect();

        // Extract card numbers
        let card_regex = Regex::new(r"\|([\d|,]+)").unwrap();
        let card_numbers = card_regex.captures(line.as_str()).unwrap();
        let card_numbers: Vec<u8> = card_numbers[1]
            .split(',')
            .map(|s| {
                let num: u8 = s.parse().unwrap();
                num
            })
            .collect();

        Card {
            winning_numbers,
            card_numbers,
            id,
        }
    }

    pub fn winning_card_numbers(&self) -> Vec<u8> {
        let card_hashset: HashSet<u8> = HashSet::from_iter(self.card_numbers.clone());
        let win_hashset: HashSet<u8> = HashSet::from_iter(self.winning_numbers.clone());
        let winning_card_numbers: Vec<u8> =
            card_hashset.intersection(&win_hashset).copied().collect();
        winning_card_numbers
    }

    pub fn number_matching(&self) -> u32 {
        let winning_card_numbers = self.winning_card_numbers();
        let winning_count: u32 = winning_card_numbers.len().try_into().unwrap();
        winning_count
    }

    pub fn score_points(&self) -> u32 {
        let winning_count = self.number_matching();
        let base: u32 = 2;
        if winning_count == 0 {
            0
        } else {
            base.pow(winning_count - 1)
        }
    }

    pub fn winning_indices(&self, limit: u8) -> Vec<u8> {
        let winning_count = self.number_matching();
        let mut winning_indices: Vec<u8> = Vec::with_capacity(winning_count as usize);
        for i in 1..=winning_count {
            let offset = i as u8;
            let index = self.id - 1 + offset;
            if index < limit {
                winning_indices.push(index);
            }
        }
        winning_indices
    }
}

fn read_input() -> String {
    fs::read_to_string("src/day4/input.txt").expect("Unable to read file.")
}

pub fn solution1() {
    let contents = read_input();
    let cards: Vec<Card> = contents.split('\n').map(Card::extract_from_line).collect();

    let mut total: u32 = 0;
    for card in cards {
        total += card.score_points();
    }

    println!("Total: {}", total);
}

pub fn solution2() {
    let contents = read_input();
    let cards: Vec<Card> = contents.split('\n').map(Card::extract_from_line).collect();
    let cards_limit = cards.len();

    let mut total_cards: u32 = 0;
    for card in &cards {
        total_cards += 1;
        let mut winning_indices = card.winning_indices(cards_limit as u8);
        let mut pile: Vec<u8> = Vec::new();
        pile.append(&mut winning_indices);
        loop {
            if !pile.is_empty() {
                let draw_index = pile.pop().unwrap();
                total_cards += 1;
                let mut draw_winning_indices =
                    cards[draw_index as usize].winning_indices(cards_limit as u8);
                pile.append(&mut draw_winning_indices);
            } else {
                break;
            }
        }
    }

    println!("Total cards: {}", total_cards);
}
