use aocd::*;
use regex::Regex;
use std::collections::HashMap;

#[aocd(2023, 8)]
pub fn solution1() {
    let mut lines: Vec<String> = input!().split('\n').map(|s| s.to_string()).collect();

    // Get commands
    let commands = lines[0].clone();

    // Remove first two lines
    lines.remove(0);
    lines.remove(0);

    // Get nodes
    let nodes: HashMap<String, (String, String)> = lines
        .iter()
        .map(|s| {
            let id_regex = Regex::new(r"(\w{3}) =").unwrap();
            let left_regex = Regex::new(r"= \((\w{3})").unwrap();
            let right_regex = Regex::new(r", (\w{3})").unwrap();

            let id = id_regex.captures(s.as_str()).unwrap()[1].to_string();
            let left = left_regex.captures(s.as_str()).unwrap()[1].to_string();
            let right = right_regex.captures(s.as_str()).unwrap()[1].to_string();

            (id, left, right)
        })
        .fold(
            HashMap::<String, (String, String)>::new(),
            |mut m, (id, left, right)| {
                m.insert(id, (left, right));
                m
            },
        );

    // Navigate to ZZZ and count steps
    let mut steps: u64 = 0;
    let mut current_node: String = String::from("AAA");
    'outer: loop {
        for command in commands.chars() {
            if current_node == "ZZZ" {
                println!("found!");
                break 'outer;
            }
            if command == 'L' {
                steps += 1;
                current_node = nodes.get(&current_node).unwrap().0.clone();
            } else if command == 'R' {
                steps += 1;
                current_node = nodes.get(&current_node).unwrap().1.clone();
            }
        }
    }

    submit!(1, steps);
}

#[aocd(2023, 8)]
pub fn solution2() {
    let mut lines: Vec<String> = input!().split('\n').map(|s| s.to_string()).collect();

    // Get commands
    let commands: Vec<usize> = lines[0]
        .chars()
        .map(|s| if s == 'L' { 0 } else { 1 })
        .collect();

    println!("Command length: {}", commands.len());

    // Remove first two lines
    lines.remove(0);
    lines.remove(0);

    // Get nodes
    let nodes: HashMap<String, [String; 2]> = lines
        .iter()
        .map(|s| {
            let id_regex = Regex::new(r"(\w{3}) =").unwrap();
            let left_regex = Regex::new(r"= \((\w{3})").unwrap();
            let right_regex = Regex::new(r", (\w{3})").unwrap();

            let id = &id_regex.captures(s).unwrap()[1];
            let left = &left_regex.captures(s).unwrap()[1];
            let right = &right_regex.captures(s).unwrap()[1];

            (id.to_string(), left.to_string(), right.to_string())
        })
        .fold(
            HashMap::<String, [String; 2]>::new(),
            |mut m, (id, left, right)| {
                m.insert(id, [left, right]);
                m
            },
        );

    // Get A nodes
    let mut current_nodes: Vec<String> =
        nodes.keys().filter(|s| s.ends_with('A')).cloned().collect();

    // List of first occurrence of a Z node and cycle duration until next one
    let mut z_data: Vec<u64> = Vec::new();

    // Navigate the map
    for node in current_nodes.iter_mut() {
        let mut steps: u64 = 0;
        'outer: loop {
            for command in commands.iter() {
                steps += 1;
                *node = nodes.get(node).unwrap()[*command].clone();
                if node.ends_with('Z') {
                    z_data.push(steps);
                    break 'outer;
                }
            }
        }
    }

    // Find the least common multiple
    let mut fct = 1;
    let largest = *z_data.iter().max().unwrap();
    'lcm: loop {
        let all_divisible = z_data.iter().all(|z| (fct * largest) % z == 0);
        if all_divisible {
            break 'lcm;
        } else {
            fct += 1;
        }
    }

    submit!(2, fct * largest);
}
