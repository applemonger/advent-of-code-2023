use aocd::*;
use regex::Regex;
use std::str::FromStr;
// use std::fs;

pub struct Record {
    row: Vec<char>,
    broken_groups: Vec<u64>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RecordParseErr;

impl FromStr for Record {
    type Err = RecordParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<&str> = s.split(' ').collect();
        let row: Vec<char> = data[0].chars().collect();
        let broken_groups: Vec<u64> = data[1]
            .trim()
            .split(',')
            .map(|c| c.parse::<u64>().unwrap())
            .collect();
        Ok(Record { row, broken_groups })
    }
}

impl Record {
    pub fn generate_candidate_rows(&self) -> Vec<String> {
        let mut rows: Vec<Vec<char>> = vec![vec!['.']];
        for c in self.row.iter() {
            let nrows = rows.len();
            if *c == '?' {
                for i in 0..nrows {
                    let mut row_copy = rows[i].clone();
                    rows[i].push('.');
                    row_copy.push('#');
                    rows.push(row_copy);
                }
            } else {
                for i in 0..nrows {
                    rows[i].push(*c);
                }
            }
        }
        rows.iter().map(|v| v.iter().collect::<String>()).collect()
    }

    pub fn generate_regex_pattern(&self) -> String {
        let mut pattern = String::new();
        for (i, group) in self.broken_groups.iter().enumerate() {
            if i == 0 {
                pattern = pattern.clone() + format!("^\\.*#{{{}}}", group).as_str();
            } else {
                pattern = pattern.clone() + format!("\\.+#{{{}}}", group).as_str();
            }
        }
        pattern = pattern.clone() + "\\.*$";
        pattern
    }

    pub fn count_possible_rows(&self) -> u64 {
        let candidates = self.generate_candidate_rows();
        let pattern = self.generate_regex_pattern();
        let re = Regex::new(pattern.as_str()).unwrap();
        candidates
            .iter()
            .map(|s| {
                if re.find(s.as_str()).is_some() {
                    1
                } else {
                    0
                }
            })
            .sum()
    }

    pub fn display_row(&self) -> String {
        self.row.iter().collect::<String>()
    }
}

// No idea how to do this without brute force :(
#[aocd(2023, 12)]
pub fn solution1() {
    // let input_data = fs::read_to_string("src/day12/test.txt").expect("Unable to read file.");
    let input_data = input!();
    let records: Vec<Record> = input_data
        .split('\n')
        .map(|s| Record::from_str(s).unwrap())
        .collect();

    let total: u64 = records.iter().map(|r| r.count_possible_rows()).sum();

    submit!(1, total);
}

#[aocd(2023, 12)]
pub fn solution2() {}
