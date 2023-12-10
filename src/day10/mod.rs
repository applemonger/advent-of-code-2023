use std::collections::HashSet;

use aocd::*;

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    value: char,
    gates: u8, // up, left, right, down
    connects_to_start: bool,
    port_flag: bool,
    starboard_flag: bool
}

impl Tile {
    pub fn from_char(c: char) -> Tile {
        let gates = match c {
            'L' => 0b1010,
            '7' => 0b0101,
            'J' => 0b1100,
            'F' => 0b0011,
            '.' => 0b0000,
            '-' => 0b0110,
            '|' => 0b1001,
            'S' => 0b1111,
            _ => unreachable!("Character not present in input data.")
        };
        let connects_to_start = c == 'S';
        Tile {
            value: c,
            gates,
            connects_to_start,
            port_flag: false,
            starboard_flag: false
        }
    }

    pub fn connects_to(&self, tile: &Tile, from_gate: u8) -> bool {
        let from_gate_open = (self.gates & from_gate) == from_gate;
        let to_gate_open = match from_gate {
            0b1000 => (tile.gates & 0b0001) == 0b0001,
            0b0100 => (tile.gates & 0b0010) == 0b0010,
            0b0010 => (tile.gates & 0b0100) == 0b0100,
            0b0001 => (tile.gates & 0b1000) == 0b1000,
            _ => unreachable!("Invalid from_gate.")
        };
        from_gate_open && to_gate_open
    }
}

pub struct Grid {
    grid: Vec<Vec<Tile>>
}

impl Grid {
    pub fn tile(&self, position: (usize, usize)) -> Tile {
        self.grid[position.0][position.1]
    }

    pub fn connect_to_loop(&mut self, position: (usize, usize)) {
        self.grid[position.0][position.1].connects_to_start = true;
    }

    pub fn flag_port(&mut self, position: (usize, usize)) {
        self.grid[position.0][position.1].port_flag = true;
    }

    pub fn flag_starboard(&mut self, position: (usize, usize)) {
        self.grid[position.0][position.1].starboard_flag = true;
    }

    pub fn delete_tile(&mut self, position: (usize, usize)) {
        self.grid[position.0][position.1] = Tile::from_char('.');
    }

    pub fn print(&self) {
        for row in &self.grid {
            let row_string: String = row.iter()
                .map(|s| s.value)
                .collect();

            println!("{}", row_string);
        }
    }
}

#[aocd(2023, 10)]
pub fn solution1() {
    // Read in grid map
    // Add in margin of . tiles around grid so we don't have to care about out-of-bounds indexing
    let mut grid: Vec<Vec<Tile>> = input!()
        .split('\n')
        .map(|s| {
            let mut row = Vec::<Tile>::new();
            row.push(Tile::from_char('.')); // margin
            for c in s.chars() {
                row.push(Tile::from_char(c))
            }
            row.push(Tile::from_char('.')); // margin
            row
        })
        .collect();

    // Add margin rows to top and bottom
    let mut margin_row = Vec::<Tile>::new();
    let ncols = grid[0].len();
    for _ in 0..ncols {
        margin_row.push(Tile::from_char('.'));
    }
    grid.insert(0, margin_row.clone());
    grid.push(margin_row.clone());

    // Determine grid size
    let nrows = grid.len() - 1;
    let ncols = grid[0].len() - 1;
    
    // Find starting position
    let mut position = (0, 0);
    for i in 1..nrows {
        for j in 1..ncols {
            if grid[i][j].value == 'S' {
                position = (i, j);
            }
        }
    }

    // Helper struct
    let mut grid = Grid { grid };

    // Traverse tiles until no more main loop tiles are present
    let mut main_loop_count = 1;
    'traverse: loop {
        // Check adjacent tiles for connectivity
        // Current position
        let i = position.0;
        let j = position.1;
        // Position of adjacent tiles
        let positions = [(i-1, j), (i+1, j), (i, j-1), (i, j+1)];
        // Gates that those respective adjacent tiles can connect to
        let from_gates: [u8; 4] = [0b1000, 0b0001, 0b0100, 0b0010];
        // Iterate through adjacent tiles
        for (pos, from_gate) in positions.iter().zip(from_gates.iter()) {
            // Check if it connects to the current tile
            let connects = grid.tile((i, j)).connects_to(&grid.tile(*pos), *from_gate);
            // Check if it has been flagged as added to the main loop (connects to start)
            let added_to_loop = grid.tile(*pos).connects_to_start;
            // If it connects and hasn't been added, flag it as added and move our current position
            if connects && !added_to_loop {
                position = *pos;
                grid.connect_to_loop(*pos);
                main_loop_count += 1;
                continue 'traverse;
            }
        }
        break 'traverse;
    }

    // Remove non main loop tiles (just for displaying)
    for i in 1..nrows {
        for j in 1..ncols {
            if !grid.tile((i, j)).connects_to_start {
                grid.delete_tile((i, j));
            }
        }
    }

    // Display grid
    grid.print();

    // Submit solution
    submit!(1, main_loop_count / 2);
}

#[aocd(2023, 10)]
pub fn solution2() {
// Read in grid map
    // Add in margin of . tiles around grid so we don't have to care about out-of-bounds indexing
    let mut grid: Vec<Vec<Tile>> = input!()
        .split('\n')
        .map(|s| {
            let mut row = Vec::<Tile>::new();
            row.push(Tile::from_char('.')); // margin
            for c in s.chars() {
                row.push(Tile::from_char(c))
            }
            row.push(Tile::from_char('.')); // margin
            row
        })
        .collect();

    // Add margin rows to top and bottom
    let mut margin_row = Vec::<Tile>::new();
    let ncols = grid[0].len();
    for _ in 0..ncols {
        margin_row.push(Tile::from_char('.'));
    }
    grid.insert(0, margin_row.clone());
    grid.push(margin_row.clone());

    // Determine grid size
    let nrows = grid.len() - 1;
    let ncols = grid[0].len() - 1;
    
    // Find starting position
    let mut position = (0, 0);
    for i in 1..nrows {
        for j in 1..ncols {
            if grid[i][j].value == 'S' {
                position = (i, j);
            }
        }
    }

    // Helper struct
    let mut grid = Grid { grid };

    // Traverse tiles
    // Visualize the traversal as a ship moving through a sea :)
    // It has a port and starboard; we will track the tiles on its port and starboard as we go
    let mut port = HashSet::<(usize, usize)>::new();
    let mut starboard = HashSet::<(usize, usize)>::new();
    let mut main_loop_count = 1;
    'traverse: loop {
        // Check adjacent tiles for connectivity
        // Current position
        let i = position.0;
        let j = position.1;
        // Position of adjacent tiles
        let positions = [(i-1, j), (i+1, j), (i, j-1), (i, j+1)];
        // Gates that those respective adjacent tiles can connect to
        let from_gates: [u8; 4] = [0b1000, 0b0001, 0b0100, 0b0010];
        // Iterate through adjacent tiles
        for (pos, from_gate) in positions.iter().zip(from_gates.iter()) {
            // Check if it connects to the current tile
            let connects = grid.tile((i, j)).connects_to(&grid.tile(*pos), *from_gate);
            // Check if it has been flagged as added to the main loop (connects to start)
            let added_to_loop = grid.tile(*pos).connects_to_start;
            // If it connects and hasn't been added, flag it as added and move our current position
            if connects && !added_to_loop {
                // Mark the port & starboard of new position
                let (new_port, new_starboard) = if *pos == (i-1, j) {
                    // Moving up
                    ((i-1, j-1), (i-1, j+1))
                } else if *pos == (i+1, j) {
                    // Moving down
                    ((i+1, j+1), (i+1, j-1))
                } else if *pos == (i, j-1) {
                    // Moving left
                    ((i+1, j-1), (i-1, j-1))
                } else {
                    // Moving right
                    ((i-1, j+1), (i+1, j+1))
                };
                let is_margin = (new_port.0 == 0) || (new_port.0 == nrows) || (new_port.1 == 0) || (new_port.1 == 1);
                if !is_margin {
                    port.insert(new_port);
                }
                let is_margin = (new_starboard.0 == 0) || (new_starboard.0 == nrows) || (new_starboard.1 == 0) || (new_starboard.1 == 1);
                if !is_margin {
                    starboard.insert(new_starboard);
                }
                position = *pos;
                grid.connect_to_loop(*pos);
                main_loop_count += 1;
                continue 'traverse;
            }
        }
        break 'traverse;
    }

    // Flag tiles for port & starboard
    for pos in port {
        if !grid.tile(pos).connects_to_start {
            grid.flag_port(pos);
        }
    }
    for pos in starboard {
        if !grid.tile(pos).connects_to_start {
            grid.flag_starboard(pos);
        }
    }

    // Iterate through tiles. If an empty tile is adjacent to a port or starboard tile, flag them as well
    'flagging: loop {
        let mut added_flag = false;
        for i in 1..nrows {
            for j in 1..ncols {
                if !grid.tile((i, j)).connects_to_start {
                    let positions = [(i-1, j), (i+1, j), (i, j-1), (i, j+1)];
                    for pos in positions {
                        if grid.tile(pos).port_flag == true && !grid.tile((i, j)).port_flag {
                            grid.flag_port((i, j));
                            added_flag = true;
                        }
                        if grid.tile(pos).starboard_flag == true && !grid.tile((i, j)).starboard_flag {
                            grid.flag_starboard((i, j));
                            added_flag = true;
                        }
                    }
                }
            }
        }
        if !added_flag {
            break 'flagging;
        }
    }

    // Display grid
    let mut starboard_count = 0;
    let mut port_count = 0;
    for row in grid.grid {
        let row_string: String = row
            .iter()
            .map(|s| {
                if s.port_flag && s.starboard_flag {
                    panic!("Should not be possible!");
                }
                if (s.port_flag || s.starboard_flag) && s.connects_to_start {
                    panic!("Inner/outer tiles cannot be on the main loop!");
                }
                if s.port_flag {
                    port_count += 1;
                    '@'
                } else if s.starboard_flag {
                    starboard_count += 1;
                    '#'
                } else {
                    s.value
                }
            })
            .collect();

        println!("{}", row_string);
    }

    // Submit solution
    println!("Main loop count: {}", main_loop_count / 2);
    println!("{} {}", port_count, starboard_count);
}