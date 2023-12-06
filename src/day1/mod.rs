use regex::{Captures, Regex};
use std::{fs, ops::Deref};

pub fn solution1() {
    let contents = fs::read_to_string("src/day1/input.txt").expect("Unable to read file.");
    let re = Regex::new(r"(\d)").unwrap();
    let lines: Vec<i32> = contents
        .split('\n')
        .map(|s| {
            let digits: Vec<&str> = re.find_iter(s).map(|m| m.as_str()).collect();
            let first: i32 = digits.first().unwrap().deref().parse().unwrap();
            let last: i32 = digits.last().unwrap().deref().parse().unwrap();
            first * 10 + last
        })
        .collect();
    println!("{:?}", lines);
}

pub fn solution2() {
    let contents = fs::read_to_string("src/day1/input.txt").expect("Unable to read file.");
    let digit_regex = Regex::new(r"(\d)").unwrap();
    let text_regex = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let reverse_regex = Regex::new(r"(enin|thgie|neves|xis|evif|ruof|eerht|owt|eno)").unwrap();
    let total: i32 = contents
        .split('\n')
        .map(|s| {
            println!("{}", s);
            let text = text_regex.replace(s, |c: &Captures| match &c[0] {
                "one" => "1one",
                "two" => "2two",
                "three" => "3three",
                "four" => "4four",
                "five" => "5five",
                "six" => "6six",
                "seven" => "7seven",
                "eight" => "8eight",
                "nine" => "9nine",
                _ => unreachable!("Unreachable error."),
            });

            let text: String = text.chars().rev().collect();

            let text = reverse_regex.replace(text.as_str(), |c: &Captures| match &c[0] {
                "eno" => "1eno",
                "owt" => "2owt",
                "eerht" => "3eerht",
                "ruof" => "4ruof",
                "evif" => "5evif",
                "xis" => "6xis",
                "neves" => "7neves",
                "thgie" => "8thgie",
                "enin" => "9enin",
                _ => unreachable!("Unreachable error."),
            });

            let text: String = text.chars().rev().collect();

            let digits: Vec<&str> = digit_regex
                .find_iter(text.as_str())
                .map(|m| m.as_str())
                .collect();
            let first: i32 = digits.first().unwrap().deref().parse().unwrap();
            let last: i32 = digits.last().unwrap().deref().parse().unwrap();
            let sum = first * 10 + last;
            println!("{}", sum);
            sum
        })
        .sum();
    println!("Total: {}", total);
}
