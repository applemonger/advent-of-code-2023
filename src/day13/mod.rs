use aocd::*;

pub struct Map {
    h_data: Vec<Vec<u8>>,
    v_data: Vec<Vec<u8>>
}

impl Map {
    pub fn from_vec(v: Vec<&str>) -> Map {
        // Convert lines into vectors of integers
        let mut h_data: Vec<Vec<u8>> = Vec::new();
        for line in v.iter() {
            let mut row = Vec::new();
            for c in line.trim().chars() {
                if c == '.' {
                    row.push(0);
                } else {
                    row.push(1);
                }
            }
            h_data.push(row);
        }

        // Convert horizontal data into vertical data
        let mut v_data: Vec<Vec<u8>> = Vec::new();
        for j in 0..h_data[0].len() {
            let mut col = Vec::new();
            for row in h_data.iter() {
                col.push(row[j]);
            }
            v_data.push(col);
        }

        Map {
            h_data,
            v_data
        }
    }

    pub fn display_horizontal_map(&self) {
        for row in self.h_data.iter() {
            let s: String = row.iter().map(|n| format!("{n}")).collect();
            println!("{}", s);
        }
        println!("");
    }

    pub fn display_vertical_map(&self) {
        for row in self.v_data.iter() {
            let s: String = row.iter().map(|n| format!("{n}")).collect();
            println!("{}", s);
        }
        println!("");
    }

    pub fn find_horizontal_symmetry(&self) -> u64 {
        let n = self.h_data.len();
        for i in 1..n {
            let mut mirrored = Vec::<bool>::new();
            for j in 0..i {
                let a = self.h_data[j].clone();
                if let Some(b) = self.h_data.get(2*i-1-j) {
                    let row_mirrored = a.iter().zip(b.iter()).all(|(x, y)| x == y);
                    mirrored.push(row_mirrored);
                }
            }
            if mirrored.iter().all(|b| *b) {
                return i as u64 * 100
            }
        }
        0
    }
    
    pub fn find_vertical_symmetry(&self) -> u64 {
        let n = self.v_data.len();
        for i in 1..n {
            let mut mirrored = Vec::<bool>::new();
            for j in 0..i {
                let a = self.v_data[j].clone();
                if let Some(b) = self.v_data.get(2*i-1-j) {
                    let row_mirrored = a.iter().zip(b.iter()).all(|(x, y)| x == y);
                    mirrored.push(row_mirrored);
                }
            }
            if mirrored.iter().all(|b| *b) {
                return i as u64
            }
        }
        0
    }

    pub fn find_near_horizontal_symmetry(&self) -> u64 {
        let n = self.h_data.len();
        for i in 1..n {
            let mut mirrored = Vec::<u64>::new();
            for j in 0..i {
                let a = self.h_data[j].clone();
                if let Some(b) = self.h_data.get(2*i-1-j) {
                    let row_mirrored: u64 = a
                        .iter()
                        .zip(b.iter())
                        .map(|(x, y)| {
                            if x == y {
                                0
                            } else {
                                1
                            }
                        })
                        .sum();
                    mirrored.push(row_mirrored);
                }
            }
            if mirrored.iter().sum::<u64>() == 1 {
                return i as u64 * 100
            }
        }
        0
    } 

    pub fn find_near_vertical_symmetry(&self) -> u64 {
        let n = self.v_data.len();
        for i in 1..n {
            let mut mirrored = Vec::<u64>::new();
            for j in 0..i {
                let a = self.v_data[j].clone();
                if let Some(b) = self.v_data.get(2*i-1-j) {
                    let row_mirrored: u64 = a
                        .iter()
                        .zip(b.iter())
                        .map(|(x, y)| {
                            if x == y {
                                0
                            } else {
                                1
                            }
                        })
                        .sum();
                    mirrored.push(row_mirrored);
                }
            }
            if mirrored.iter().sum::<u64>() == 1 {
                return i as u64
            }
        }
        0
    }
    
    pub fn find_symmetry(&self) -> u64 {
        self.find_horizontal_symmetry() + self.find_vertical_symmetry()
    }

    pub fn find_near_symmetry(&self) -> u64 {
        self.find_near_horizontal_symmetry() + self.find_near_vertical_symmetry()
    }
}


#[aocd(2023, 13)]
pub fn solution1() {
    let input_data = input!();
    let lines: Vec<&str> = input_data.split("\n\n").collect();
    // let input_data = fs::read_to_string("src/day13/test.txt").expect("Unable to read file.");
    // let lines: Vec<&str> = input_data.split("\r\n\r\n").collect();

    // Read data
    let mut maps = Vec::<Map>::new();
    for line in lines.iter() {
        let map_rows: Vec<&str> = line.split('\n').collect();
        let map = Map::from_vec(map_rows);
        maps.push(map);
    }

    // Find symmetry scores
    let total: u64 = maps
        .iter()
        .map(|s| s.find_symmetry())
        .sum();
    
    submit!(1, total);
}

#[aocd(2023, 13)]
pub fn solution2() {
    let input_data = input!();
    let lines: Vec<&str> = input_data.split("\n\n").collect();
    // let input_data = fs::read_to_string("src/day13/test.txt").expect("Unable to read file.");
    // let lines: Vec<&str> = input_data.split("\r\n\r\n").collect();

    // Read data
    let mut maps = Vec::<Map>::new();
    for line in lines.iter() {
        let map_rows: Vec<&str> = line.split('\n').collect();
        let map = Map::from_vec(map_rows);
        maps.push(map);
    }

    // Find symmetry scores
    let total: u64 = maps
        .iter()
        .map(|s| s.find_near_symmetry())
        .sum();
    
    submit!(2, total);
}