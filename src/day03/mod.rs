use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

struct Grid {
    i_limit: usize,
    j_limit: usize,
}

impl Grid {
    fn valid_offset(&self, i: usize, j: usize, i_offset: i32, j_offset: i32) -> bool {
        let i: i32 = i as i32 + i_offset;
        let j: i32 = j as i32 + j_offset;
        (i >= 0) && (i < self.i_limit as i32) && (j >= 0) && (j < self.j_limit as i32)
    }
}

pub fn solution1() {
    // Read data
    let contents: String = fs::read_to_string("src/day3/input.txt").expect("Unable to read file.");
    let mut matrix: Vec<Vec<char>> = contents
        .split('\n')
        .map(|s| {
            let vec: Vec<char> = s.chars().collect();
            vec
        })
        .collect();

    // Get row & column count
    let nrows = contents.split('\n').count();
    let ncols = contents
        .split('\n')
        .map(|s| s.chars().count())
        .max()
        .unwrap()
        - 1;
    let grid = Grid {
        i_limit: nrows,
        j_limit: ncols,
    };

    // If a digit is next to a symbol or an X, replace it with an X
    let digits: HashSet<char> = "0123456789".chars().collect();
    let nonsymbols: HashSet<char> = "0123456789.".chars().collect();
    for _ in 0..5 {
        for i in 0..nrows {
            for j in 0..ncols {
                if digits.contains(&matrix[i][j]) {
                    // Top left
                    if grid.valid_offset(i, j, -1, -1)
                        && !nonsymbols.contains(&matrix[i - 1][j - 1])
                    {
                        matrix[i][j] = 'X';
                    }

                    // Top middle
                    if grid.valid_offset(i, j, -1, 0) && !nonsymbols.contains(&matrix[i - 1][j]) {
                        matrix[i][j] = 'X';
                    }

                    // Top right
                    if grid.valid_offset(i, j, -1, 1) && !nonsymbols.contains(&matrix[i - 1][j + 1])
                    {
                        matrix[i][j] = 'X';
                    }

                    // Center left
                    if grid.valid_offset(i, j, 0, -1) && !nonsymbols.contains(&matrix[i][j - 1]) {
                        matrix[i][j] = 'X';
                    }

                    // Center right
                    if grid.valid_offset(i, j, 0, 1) && !nonsymbols.contains(&matrix[i][j + 1]) {
                        matrix[i][j] = 'X';
                    }

                    // Bottom left
                    if grid.valid_offset(i, j, 1, -1) && !nonsymbols.contains(&matrix[i + 1][j - 1])
                    {
                        matrix[i][j] = 'X';
                    }

                    // Bottom middle
                    if grid.valid_offset(i, j, 1, 0) && !nonsymbols.contains(&matrix[i + 1][j]) {
                        matrix[i][j] = 'X';
                    }

                    // Bottom right
                    if grid.valid_offset(i, j, 1, 1) && !nonsymbols.contains(&matrix[i + 1][j + 1])
                    {
                        matrix[i][j] = 'X';
                    }
                }
            }
        }
    }

    // Get original data
    let mut original_matrix: Vec<Vec<char>> = contents
        .split('\n')
        .map(|s| {
            let vec: Vec<char> = s.chars().collect();
            vec
        })
        .collect();

    // Mask the original data where it has not been marked with X
    for i in 0..nrows {
        for j in 0..ncols {
            if matrix[i][j] != 'X' {
                original_matrix[i][j] = '.';
            }
        }
    }

    // Sum numbers
    let re = Regex::new(r"([0-9]+)").unwrap();
    let mut total = 0;
    for r in original_matrix.iter().take(nrows) {
        let row: String = r.clone().into_iter().collect();
        for (_, [s]) in re.captures_iter(row.as_str()).map(|c| c.extract()) {
            let num: i32 = s.parse().unwrap();
            total += num;
        }
    }

    println!("Sum of parts: {}", total);
}

pub fn solution2() {
    // Read data
    let contents: String = fs::read_to_string("src/day3/input.txt").expect("Unable to read file.");
    let mut matrix: Vec<Vec<char>> = contents
        .split('\n')
        .map(|s| {
            let vec: Vec<char> = s.chars().collect();
            vec
        })
        .collect();

    // Get row & column count
    let nrows = contents.split('\n').count();
    let ncols = contents
        .split('\n')
        .map(|s| s.chars().count())
        .max()
        .unwrap()
        - 1;
    let grid = Grid {
        i_limit: nrows,
        j_limit: ncols,
    };

    // If a digit is next to a symbol or an X, replace it with an X
    let digits: HashSet<char> = "0123456789".chars().collect();
    let nonsymbols: HashSet<char> = "0123456789.".chars().collect();
    for _ in 0..5 {
        for i in 0..nrows {
            for j in 0..ncols {
                if digits.contains(&matrix[i][j]) {
                    // Top left
                    if grid.valid_offset(i, j, -1, -1)
                        && !nonsymbols.contains(&matrix[i - 1][j - 1])
                    {
                        matrix[i][j] = 'X';
                    }

                    // Top middle
                    if grid.valid_offset(i, j, -1, 0) && !nonsymbols.contains(&matrix[i - 1][j]) {
                        matrix[i][j] = 'X';
                    }

                    // Top right
                    if grid.valid_offset(i, j, -1, 1) && !nonsymbols.contains(&matrix[i - 1][j + 1])
                    {
                        matrix[i][j] = 'X';
                    }

                    // Center left
                    if grid.valid_offset(i, j, 0, -1) && !nonsymbols.contains(&matrix[i][j - 1]) {
                        matrix[i][j] = 'X';
                    }

                    // Center right
                    if grid.valid_offset(i, j, 0, 1) && !nonsymbols.contains(&matrix[i][j + 1]) {
                        matrix[i][j] = 'X';
                    }

                    // Bottom left
                    if grid.valid_offset(i, j, 1, -1) && !nonsymbols.contains(&matrix[i + 1][j - 1])
                    {
                        matrix[i][j] = 'X';
                    }

                    // Bottom middle
                    if grid.valid_offset(i, j, 1, 0) && !nonsymbols.contains(&matrix[i + 1][j]) {
                        matrix[i][j] = 'X';
                    }

                    // Bottom right
                    if grid.valid_offset(i, j, 1, 1) && !nonsymbols.contains(&matrix[i + 1][j + 1])
                    {
                        matrix[i][j] = 'X';
                    }
                }
            }
        }
    }

    // Get original data
    let mut masked_matrix: Vec<Vec<char>> = contents
        .split('\n')
        .map(|s| {
            let vec: Vec<char> = s.chars().collect();
            vec
        })
        .collect();

    // Mask the original data where it has not been marked with X
    for i in 0..nrows {
        for j in 0..ncols {
            if (matrix[i][j] != 'X') && (masked_matrix[i][j] != '*') {
                masked_matrix[i][j] = '.';
            }
        }
    }

    // Get list of number locations
    let mut numbers: Vec<Number> = Vec::new();
    for (i, row) in masked_matrix.iter().enumerate().take(nrows) {
        for j in 0..ncols {
            // If current character is a digit
            if digits.contains(&row[j]) {
                // Check if character to left is out of bounds
                let in_bounds = grid.valid_offset(i, j, 0, -1);

                // Check if character to the left is not a digit
                let not_a_digit = if !in_bounds {
                    true
                } else {
                    !digits.contains(&row[j - 1])
                };

                // If the character to the left is not a digit
                if not_a_digit {
                    let mut offset: usize = 0;
                    let mut values: Vec<char> = Vec::new();
                    loop {
                        if !grid.valid_offset(i, j, 0, offset as i32) {
                            break;
                        }
                        if digits.contains(&masked_matrix[i][j + offset]) {
                            values.push(masked_matrix[i][j + offset]);
                            offset += 1;
                        } else {
                            break;
                        }
                    }
                    let value = values
                        .into_iter()
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap();
                    numbers.push(Number {
                        i,
                        j,
                        value,
                        length: offset,
                        i_limit: nrows,
                        j_limit: ncols,
                    });
                }
            }
        }
    }

    // Print the map
    for line in &masked_matrix {
        let s: String = line.clone().into_iter().collect();
        println!("{}", s);
    }

    // Create list of asterisks with adjacent numbers
    let mut asterisks: HashMap<(usize, usize), HashSet<Number>> = HashMap::new();
    for number in numbers {
        for (i, j) in number.adjacent_points() {
            if masked_matrix[i][j] == '*' {
                if let Some(points) = asterisks.get_mut(&(i, j)) {
                    points.insert(number);
                } else {
                    asterisks.insert((i, j), HashSet::from([number]));
                }
            }
        }
    }

    // Filter on asterisks with only two adjacent numbers
    let total_gear_ratio: i32 = asterisks
        .into_iter()
        .filter(|(_, value)| value.len() == 2)
        .map(|(_, value)| {
            let gear_ratio: i32 = value.into_iter().map(|x| x.value).product();
            gear_ratio
        })
        .sum();

    println!("Total gear ratio: {}", total_gear_ratio);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Number {
    i: usize,
    j: usize,
    value: i32,
    length: usize,
    i_limit: usize,
    j_limit: usize,
}

impl Number {
    pub fn adjacent_points(&self) -> HashSet<(usize, usize)> {
        let mut points: HashSet<(usize, usize)> = HashSet::new();
        let number_points = self.points();
        for x in 0..self.length {
            let i = self.i as i32;
            let j = self.j as i32;
            let offset = x as i32;

            let candidates = [
                (i - 1, j - 1 + offset),
                (i - 1, j + offset),
                (i - 1, j + 1 + offset),
                (i, j - 1 + offset),
                (i, j + 1 + offset),
                (i + 1, j - 1 + offset),
                (i + 1, j + offset),
                (i + 1, j + 1 + offset),
            ];

            for (ci, cj) in candidates {
                if self.valid_point(ci, cj) && !number_points.contains(&(ci as usize, cj as usize))
                {
                    points.insert((ci as usize, cj as usize));
                }
            }
        }

        points
    }

    fn points(&self) -> HashSet<(usize, usize)> {
        let mut pts: HashSet<(usize, usize)> = HashSet::new();
        for x in 0..self.length {
            pts.insert((self.i, self.j + x));
        }
        pts
    }

    fn valid_point(&self, i: i32, j: i32) -> bool {
        (i >= 0) && (i < self.i_limit as i32) && (j >= 0) && (j < self.j_limit as i32)
    }
}
