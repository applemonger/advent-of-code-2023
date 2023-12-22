use aocd::*;
use regex::Regex;

#[derive(Debug)]
pub struct Command<'a> {
    direction: char,
    length: usize,
    color: &'a str
}

impl<'a> Command<'a> {
    fn from_str(s: &str) -> Command {
        let data: Vec<&str> = s.split(' ').collect();
        Command {
            direction: data[0].chars().next().unwrap(),
            length: data[1].parse::<usize>().unwrap(),
            color: data[2]
        }
    }

    fn extract_direction_from_color(&self) -> char {
        let color_vec = self.color.clone().chars().collect::<Vec<char>>();
        let nchars = color_vec.len();
        // Second to last character is the direction digit (parentheses is last char)
        match color_vec[nchars - 2] {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            _ => unreachable!("Undefined color.")
        }
    }

    fn extract_length_from_color(&self) -> usize {
        let mut color_vec = self.color.clone().chars().collect::<Vec<char>>();
        // Remove leading parentheses
        color_vec.remove(0);
        // Remove hashtag
        color_vec.remove(0);
        // Remove last parentheses
        color_vec.pop();
        // Remove last digit
        color_vec.pop();
        // Convert to str
        let color_str = color_vec.iter().collect::<String>();
        // Convert hex string to usize
        usize::from_str_radix(color_str.as_str(), 16).unwrap()
    }
}

#[derive(PartialEq, Eq)]
pub enum TileType {
    Empty,
    Inner,
    Edge
}

pub struct Tile {
    tile_type: TileType
}

pub struct Grid {
    grid: Vec<Vec<Tile>>,
    start: (usize, usize)
}

impl Grid {
    pub fn from_commands(commands: &Vec<Command>) -> Grid {
        // Initial minimum and maximum positions
        let mut i_min: i32 = 0;
        let mut j_min: i32 = 0;
        let mut i_max: i32 = 0;
        let mut j_max: i32 = 0;

        // Track up/down, left/right movement through the commands
        let mut i_pos: i32 = 0;
        let mut j_pos: i32 = 0;
        
        // If our current vertical/horizontal position exceeds our current row/column
        // limits, expand them.
        for command in commands.iter() {
            match command.direction {
                'R' => {
                    j_pos += command.length as i32;
                    j_max = j_max.max(j_pos);
                    j_min = j_min.min(j_pos);
                },
                'L' => {
                    j_pos -= command.length as i32;
                    j_max = j_max.max(j_pos);
                    j_min = j_min.min(j_pos);
                },
                'U' => {
                    i_pos -= command.length as i32;
                    i_max = i_max.max(i_pos);
                    i_min = i_min.min(i_pos);
                },
                'D' => {
                    i_pos += command.length as i32;
                    i_max = i_max.max(i_pos);
                    i_min = i_min.min(i_pos);
                },
                _ => unreachable!("Undefined direction.")
            }
        }

        // Populate grid with empty tiles
        let i_size = i_max - i_min + 1;
        let j_size = j_max - j_min + 1;
        let start = (i_min.abs() as usize, j_min.abs() as usize);
        let mut grid = Vec::new();
        for _i in 0..i_size {
            let mut row = Vec::<Tile>::new();
            for _j in 0..j_size {
                row.push(Tile { 
                    tile_type: TileType::Empty
                });
            }
            grid.push(row);
        }

        Grid { 
            grid,
            start
        }
    }

    pub fn display(&self) {
        println!("({}, {})", self.grid.len(), self.grid[0].len());
        for row in self.grid.iter() {
            let row_string = row.iter()
                .map(|t| {
                    match t.tile_type {
                        TileType::Edge => '#',
                        TileType::Empty => '.',
                        TileType::Inner => 'o'
                    }
                })
                .collect::<String>();
            println!("{}", row_string);
        }
    }

    pub fn nrows(&self) -> usize {
        self.grid.len()
    }

    pub fn ncols(&self) -> usize {
        self.grid[0].len()
    }

    pub fn traversal(&mut self, commands: &Vec<Command>) {
        let mut i_pos: i32 = self.start.0 as i32;
        let mut j_pos: i32 = self.start.1 as i32;
        let mut tiles = vec![(i_pos as usize, j_pos as usize)];
        let mut starboards = Vec::<(usize, usize)>::new();

        // Traverse the grid using the commands
        for command in commands.iter() {
            let (i_incr, j_incr, i_offset, j_offset) = match command.direction {
                'R' => (0 as i32, 1 as i32, 1 as i32, 0 as i32),
                'L' => (0, -1, -1, 0),
                'U' => (-1, 0, 0, 1),
                'D' => (1, 0, 0, -1),
                _ => unreachable!("Undefined direction.")
            };
            for _ in 0..command.length {
                // Add starboard tile next to current tile
                let i_starboard = i_pos + i_offset;
                let j_starboard = j_pos + j_offset;
                starboards.push((i_starboard as usize, j_starboard as usize));

                // Move position to next tile
                i_pos += i_incr;
                j_pos += j_incr;
                tiles.push((i_pos as usize, j_pos as usize));
                
                // Add starboard tile of that one
                let i_starboard = i_pos + i_offset;
                let j_starboard = j_pos + j_offset;
                starboards.push((i_starboard as usize, j_starboard as usize));
            }
        }

        // Label tiles
        for (i, j) in starboards.iter() {
            self.grid[*i][*j].tile_type = TileType::Inner;
        }
        for (i, j) in tiles.iter() {
            self.grid[*i][*j].tile_type = TileType::Edge;
        }

        // Grow inner tiles to any adjacent empty tiles
        let nrows = self.nrows();
        let ncols = self.ncols();
        let i_offsets = [-1, 1];
        let j_offsets = [-1, 1];
        let mut growing = true;
        while growing {
            growing = false;
            for i in 0..nrows {
                for j in 0..ncols {
                    if self.grid[i][j].tile_type == TileType::Inner {
                        for i_offset in i_offsets {
                            for j_offset in j_offsets {
                                let i_index = (i as i32 + i_offset) as usize;
                                let j_index = (j as i32 + j_offset) as usize;
                                if self.grid[i_index][j_index].tile_type == TileType::Empty {
                                    self.grid[i_index][j_index].tile_type = TileType::Inner;
                                    growing = true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn area(&self) -> u64 {
        self.grid
            .iter()
            .map(|row| {
                let row_score: u64 = row
                    .iter()
                    .map(|t| {
                        let score = match t.tile_type {
                            TileType::Edge | TileType::Inner => 1,
                            TileType::Empty => 0
                        };
                        score
                    })
                    .sum();
                row_score
            })
            .sum()
    }
}

#[aocd(2023, 18)]
pub fn solution1() {
    let input_data = input!();
    let commands: Vec<Command> = input_data
        .split('\n')
        .map(|s| Command::from_str(s.trim()))
        .collect();

    let mut grid = Grid::from_commands(&commands);
    grid.traversal(&commands);
    grid.display();
    submit!(1, grid.area());
}

#[aocd(2023, 18, "src/day18/test.txt")]
pub fn solution2() {
    let input_data = input!();
    let mut commands: Vec<Command> = input_data
        .split('\n')
        .map(|s| Command::from_str(s.trim()))
        .collect();

    for command in commands.iter_mut() {
        command.direction = command.extract_direction_from_color();
        command.length = command.extract_length_from_color();
    }

    let mut grid = Grid::from_commands(&commands);
    grid.traversal(&commands);
    grid.display();
    submit!(2, grid.area());
}