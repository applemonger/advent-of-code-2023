use std::{fs, collections::{hash_map::DefaultHasher, HashSet}, hash::{Hash, Hasher}};
use aocd::*;

pub struct Grid {
    pub data: Vec<Vec<char>>
}

impl Grid {
    pub fn from_vec_string(v: Vec<String>) -> Grid {
        let data: Vec<Vec<char>> = v
            .iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect();
        Grid {
            data
        }
    }

    pub fn rotate_clockwise(&mut self) {
        let mut new_data: Vec<Vec<char>> = Vec::new();
        for j in 0..self.data[0].len() {
            let mut new_row = Vec::<char>::new();
            for row in self.data.iter().rev() {
                new_row.push(row[j]);
            }
            new_data.push(new_row);
        }
        self.data = new_data;
    }

    pub fn display(&self) {
        for line in self.data.iter() {
            let s = line.iter().collect::<String>();
            println!("{}", s);
        }
    }

    pub fn tilt(&mut self) {
        let tilted_data: Vec<Vec<char>> = self.data
            .iter()
            .map(|v| {
                let s: String = v.iter().collect();
                let mut lines = s.split('#')
                    .map(|s| {
                        let mut line_vec = s.chars().collect::<Vec<char>>();
                        line_vec.sort();
                        line_vec.push('#');
                        line_vec
                    })
                    .flatten()
                    .collect::<Vec<char>>();
                lines.pop();
                lines
            })
            .collect();
        self.data = tilted_data;
    }

    pub fn score_load(&self) -> u64 {
        let mut total: u64 = 0;
        for row in self.data.iter() {
            for (i, c) in row.iter().enumerate() {
                if *c == 'O' {
                    total += i as u64 + 1;
                }
            }
        }
        total
    }

    pub fn cycle(&mut self) {
        self.rotate_clockwise();
        self.tilt();
        self.rotate_clockwise();
        self.tilt();
        self.rotate_clockwise();
        self.tilt();
        self.rotate_clockwise();
        self.tilt();
    }

    pub fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.data.hash(&mut hasher);
        hasher.finish()
    }
}

#[aocd(2023, 14)]
pub fn solution1() {
    let input_data = input!();
    //let input_data = fs::read_to_string("src/day14/test.txt").expect("Unable to read file.");
    let lines: Vec<String> = input_data.split('\n').map(|s| s.to_string()).collect();
    let mut grid = Grid::from_vec_string(lines);
    grid.rotate_clockwise();
    grid.tilt();
    submit!(1, grid.score_load());
}

#[aocd(2023, 14)]
pub fn solution2() {
    let input_data = input!();
    let lines: Vec<String> = input_data.split('\n').map(|s| s.to_string()).collect();
    let mut grid = Grid::from_vec_string(lines.clone());

    // Store hashes of grid after each cycle to identify 
    let mut hashes: Vec<u64> = Vec::new();
    'cycles: loop {
        // Cycle the grid
        grid.cycle();
        // Compute the hash
        let hash = grid.hash();
        // Store the hash, break if we've seen this hash before (reached the end of the phase)
        if hashes.contains(&hash) {
            hashes.push(hash);
            break 'cycles;
        }
        hashes.push(hash);
    }

    // Get the element at which we start repeating grid states
    let cycle_hash = *hashes.last().unwrap();

    // Get the index of the first time we see that grid state (beginning of phase)
    let index = hashes.iter().position(|x| *x == cycle_hash).unwrap();
    let initial_cycle = index + 1;

    // Get the length of the phase
    let cycle_duration = hashes.len() - initial_cycle;
    
    // Set the number of cycles for which we want to find the score
    let n = 1_000_000_000; // Get the score at the billionth cycle

    // Shortened number of cycles we need to run
    let n_cycles = initial_cycle + (n - initial_cycle) % cycle_duration;
    
    // Put it through abbreviated cycles
    let mut grid = Grid::from_vec_string(lines.clone());
    for _ in 1..=n_cycles {
        grid.cycle();
    }

    // Rotate one more time to get it in the right position
    grid.rotate_clockwise();
    submit!(2, grid.score_load());
}
