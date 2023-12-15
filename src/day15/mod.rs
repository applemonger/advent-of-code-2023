use aocd::*;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Sequence {
    original: String,
    values: Vec<u8>
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseSequenceError;

impl FromStr for Sequence {
    type Err = ParseSequenceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<u8> = s.chars().map(|c| c as u8).collect();
        Ok(Sequence {
            original: s.to_owned(),
            values
        })
    }
}

impl Sequence {
    pub fn hash(&self) -> u64 {
        self.values
            .iter()
            .fold(0u64, |mut acc, el| {
                acc += *el as u64;
                acc *= 17;
                acc = acc % 256;
                acc
            })
    }
}

#[aocd(2023, 15)]
pub fn solution1() {
    let sequences: Vec<Sequence> = input!()
        .split(',')
        .map(|s| {
            Sequence::from_str(s.trim()).unwrap()
        })
        .collect();

    let total: u64 = sequences
        .iter()
        .map(|seq| seq.hash())
        .sum();

    submit!(1, total);
}

#[derive(Debug, Clone)]
pub struct Lens {
    label: Sequence,
    focal_length: u8,
    operator: char
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseStepError;

impl FromStr for Lens {
    type Err = ParseStepError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('=') {
            let mut s_vec: Vec<char> = s.chars().collect();
            let focal_length = s_vec.pop().unwrap().to_string().parse::<u8>().unwrap();
            let operator = s_vec.pop().unwrap(); // Remove operator
            let s = s_vec.iter().collect::<String>();
            let label = Sequence::from_str(s.as_str()).unwrap();
            Ok(Lens {
                label, 
                focal_length,
                operator
            })
        } else {
            let mut s_vec: Vec<char> = s.chars().collect();
            let focal_length: u8 = 0;
            let operator = s_vec.pop().unwrap();
            let s = s_vec.iter().collect::<String>();
            let label = Sequence::from_str(s.as_str()).unwrap();
            Ok(Lens {
                label, 
                focal_length,
                operator
            })
        }
    }
}

pub struct LensBox {
    index: u8,
    lenses: Vec<Lens>
}

impl LensBox {
    pub fn new(index: u8) -> LensBox {
        LensBox {
            index,
            lenses: Vec::<Lens>::new()
        }
    }

    pub fn score(&self) -> u64 {
        self.lenses.iter().enumerate().map(|(i, lens)| {
            (self.index as u64 + 1) * (i as u64 + 1) * lens.focal_length as u64
        }).sum()
    }

    pub fn handle_lens(&mut self, lens: Lens) {
        if lens.operator == '-' {
            let mut remove_lens = false;
            let mut remove_lens_index = 0;
            'search: for (i, l) in self.lenses.iter_mut().enumerate() {
                if l.label.original == lens.label.original {
                    remove_lens = true;
                    remove_lens_index = i;
                    break 'search;
                }
            }
            if remove_lens {
                self.lenses.remove(remove_lens_index);
            }
        } else if lens.operator == '=' {
            let mut already_exists = false;
            'search: for (i, l) in self.lenses.iter_mut().enumerate() {
                if l.label.original == lens.label.original {
                    already_exists = true;
                    l.focal_length = lens.focal_length;
                    break 'search;
                }
            }
            if !already_exists {
                self.lenses.push(lens)
            }
        }
    }
}

#[aocd(2023, 15)]
pub fn solution2() {
    // Create boxes
    let mut boxes: Vec<LensBox> = Vec::new();
    for i in 0..=255 {
        boxes.push(LensBox::new(i));
    }

    // Move lenses into boxes
    for s in input!().split(',') {
        let lens = Lens::from_str(s.trim()).unwrap();
        let box_no = lens.label.hash() as usize;
        boxes[box_no].handle_lens(lens);
    }

    // Score lenses
    let total: u64 = boxes.iter().map(|b| b.score()).sum();
    submit!(2, total);
}