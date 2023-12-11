use std::collections::HashSet;

use aocd::*;

#[aocd(2023, 11)]
pub fn solution1() {
    // Read input
    let mut grid: Vec<Vec<char>> = input!()
        .split('\n')
        .map(|s| {
            let chars: Vec<char> = s.chars().collect();
            chars
        })
        .collect();

    // Cosmic expansion:
    // Duplicate empty rows
    let mut i = 0;
    while i < grid.len() {
        if grid[i].iter().all(|c| *c == '.') {
            grid.insert(i, grid[i].clone());
            i += 2;
        } else {
            i += 1;
        }
    }

    // Duplicate empty columns
    let mut j = 0;
    while j < grid[0].len() {
        if grid.iter().all(|row| row[j] == '.') {
            for row in grid.iter_mut() {
                row.insert(j, '.');
            }
            j += 2;
        } else {
            j += 1;
        }
    }

    // Get galaxy positions
    let mut coordinates: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '#' {
                coordinates.insert((i, j));
            }
        }
    }

    // Get galaxy pairs
    let mut pairs: HashSet<[(usize, usize); 2]> = HashSet::new();
    for a_coord in &coordinates {
        for b_coord in &coordinates {
            if *a_coord != *b_coord {
                let mut coords = [*a_coord, *b_coord];
                coords.sort();
                pairs.insert(coords);
            }
        }
    }

    // Get distance between pairs
    let mut dists: Vec<u64> = Vec::new();
    for pair in pairs {
        let x_dist = (pair[0].0 as i32 - pair[1].0 as i32).abs();
        let y_dist = (pair[0].1 as i32 - pair[1].1 as i32).abs();
        dists.push((x_dist + y_dist) as u64);
    }

    // Get sum of distances
    let total: u64 = dists.iter().sum();
    submit!(1, total);
}

#[aocd(2023, 11)]
pub fn solution2() {
    // Read input
    let grid: Vec<Vec<char>> = input!()
        .split('\n')
        .map(|s| {
            let chars: Vec<char> = s.chars().collect();
            chars
        })
        .collect();

    // Cosmic expansion:
    // Get indices of empty rows
    let mut empty_rows: Vec<usize> = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        if row.iter().all(|c| *c == '.') {
            empty_rows.push(i);
        }
    }
    
    // Get indices of empty columns
    let mut empty_cols: Vec<usize> = Vec::new();
    for j in 0..grid[0].len() {
        if grid.iter().all(|row| row[j] == '.') {
            empty_cols.push(j);
        }
    }

    // Get galaxy positions
    let mut coordinates: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '#' {
                coordinates.insert((i, j));
            }
        }
    }

    // Get galaxy pairs
    let mut pairs: HashSet<[(usize, usize); 2]> = HashSet::new();
    for a_coord in &coordinates {
        for b_coord in &coordinates {
            if *a_coord != *b_coord {
                let mut coords = [*a_coord, *b_coord];
                coords.sort();
                pairs.insert(coords);
            }
        }
    }

    // Get distance between pairs
    let mut dists: Vec<u64> = Vec::new();
    for pair in pairs {
        let row_min = pair[0].0.min(pair[1].0);
        let row_max = pair[0].0.max(pair[1].0);
        let col_min = pair[0].1.min(pair[1].1);
        let col_max = pair[0].1.max(pair[1].1);

        let empty_row_count: u64 = empty_rows
            .iter()
            .map(|row| {
                if *row > row_min && *row < row_max {
                    1
                } else {
                    0
                }
            })
            .sum();

        let empty_col_count: u64 = empty_cols
            .iter()
            .map(|col| {
                if *col > col_min && *col < col_max {
                    1
                } else {
                    0
                }
            })
            .sum();

        let row_dist = (row_max as u64 - row_min as u64) + (empty_row_count * 999_999);
        let col_dist = (col_max as u64 - col_min as u64) + (empty_col_count * 999_999);
        dists.push((row_dist + col_dist) as u64);
    }

    // Get sum of distances
    let total: u64 = dists.iter().sum();
    submit!(2, total);
}