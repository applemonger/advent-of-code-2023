use aocd::*;
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Hash)]
pub struct Tile {
    tile_type: char,
    energized: bool,
    history: Vec<Photon>
}

impl Tile {
    pub fn new(tile_type: char) -> Tile {
        Tile {
            tile_type,
            energized: false,
            history: Vec::new()
        }
    }

    pub fn energize(&mut self) {
        self.energized = true;
    }

    pub fn transform(&mut self, photon: &Photon) -> Vec<Photon> {
        // A photon has visited the tile, so the tile is now energized
        self.energize();

        // If a photon with this particular direction has already visited this tile 
        // before, do not propagate it
        if self.history.contains(photon) {
            return Vec::<Photon>::new()
        } else {
            self.history.push(photon.clone());
        }

        // Alter the photon's trajectory
        match self.tile_type {
            '.' => {
                let photon_t = photon.clone();
                vec![photon_t]
            },
            '/' => {
                let mut photon_t = photon.clone();
                photon_t.direction = match photon.direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Right => Direction::Up,
                    Direction::Left => Direction::Down
                };
                vec![photon_t]
            },
            '\\' => {
                let mut photon_t = photon.clone();
                photon_t.direction = match photon.direction {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Left => Direction::Up
                };
                vec![photon_t]
            },
            '|' => {
                match photon.direction {
                    Direction::Up | Direction::Down => {
                        let photon_t = photon.clone();
                        vec![photon_t]
                    },
                    Direction::Left | Direction::Right => {
                        let mut photon_t1 = photon.clone();
                        let mut photon_t2 = photon.clone();
                        photon_t1.direction = Direction::Up;
                        photon_t2.direction = Direction::Down;
                        vec![photon_t1, photon_t2]
                    }
                }
            },
            '-' => {
                match photon.direction {
                    Direction::Left | Direction::Right => {
                        let photon_t = photon.clone();
                        vec![photon_t]
                    },
                    Direction::Up | Direction::Down => {
                        let mut photon_t1 = photon.clone();
                        let mut photon_t2 = photon.clone();
                        photon_t1.direction = Direction::Left;
                        photon_t2.direction = Direction::Right;
                        vec![photon_t1, photon_t2]
                    }
                }
            },
            _ => unreachable!("Non-existent tile type.")
        }
    }

    pub fn clear(&mut self) {
        self.energized = false;
        self.history = Vec::new();
    }
}

#[derive(Hash)]
pub struct Grid {
    grid: Vec<Vec<Tile>>
}

impl Grid {
    pub fn display(&self) {
        for row in self.grid.iter() {
            let s: String = row.iter().map(|t| {
                if t.energized {
                    '#'
                } else {
                    t.tile_type
                }
            }).collect();
            println!("{}", s);
        }
    }

    pub fn tile(&mut self, position: (i32, i32)) -> Option<&mut Tile> {
        if !self.out_of_bounds(position) {
            Some(&mut self.grid[position.0 as usize][position.1 as usize])
        } else {
            None
        }
    }

    pub fn out_of_bounds(&self, position: (i32, i32)) -> bool {
        (position.0 < 0) || (position.0 >= self.grid.len() as i32) || (position.1 < 0) || (position.1 >= self.grid[0].len() as i32)
    }

    pub fn beam_photon_at(&mut self, position: (i32, i32), direction: Direction) {
        // Create first photon
        let initial_photon = Photon {
            position,
            direction
        };
        
        // List to store all photons
        let mut photons = vec![initial_photon];

        // Traverse the grid
        while !photons.is_empty() {
            // Move each photon
            photons.retain_mut(|p| p.travel(self));

            // Transform directions based on tile position
            photons = photons
                .iter_mut()
                .flat_map(|p| {
                    if let Some(tile) = self.tile(p.position) {
                        let vec_photons_t = tile.transform(p);
                        vec_photons_t
                    } else {
                        Vec::new()
                    }
                })
                .collect();
        }
    }

    pub fn score(&self) -> u64 {
        let mut score: u64 = 0;
        for row in self.grid.iter() {
            for tile in row.iter() {
                if tile.energized {
                    score += 1;
                }
            }
        }
        score
    }

    pub fn clear(&mut self) {
        for row in self.grid.iter_mut() {
            for tile in row.iter_mut() {
                tile.clear();
            }
        }
    }
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Photon {
    position: (i32, i32),
    direction: Direction
}

impl Photon {
    pub fn trajectory(&self) -> (i32, i32) {
        match self.direction {
            Direction::Up => (self.position.0 - 1, self.position.1),
            Direction::Down => (self.position.0 + 1, self.position.1),
            Direction::Left => (self.position.0, self.position.1 - 1),
            Direction::Right => (self.position.0, self.position.1 + 1)
        }
    }

    pub fn travel(&mut self, grid: &mut Grid) -> bool {
        if grid.tile(self.trajectory()).is_some() {
            self.position = self.trajectory();
            true
        } else {
            false
        }
    }
}

#[aocd(2023, 16, "src/day16/test.txt")]
pub fn solution1() {
    let input_data: Vec<Vec<Tile>> = input!()
        .split('\n')
        .map(|s| {
            s.trim().chars()
                .map(|c| Tile::new(c))
                .collect::<Vec<Tile>>()
        })
        .collect();

    // Initialize grid
    let mut grid = Grid { grid: input_data };

    // Display grid
    grid.display();

    // Shoot a photon
    grid.beam_photon_at((0, -1), Direction::Right);

    // Submit score
    submit!(1, grid.score());
}

#[aocd(2023, 16)]
pub fn solution2() {
    let input_data: Vec<Vec<Tile>> = input!()
        .split('\n')
        .map(|s| {
            s.trim().chars()
                .map(|c| Tile::new(c))
                .collect::<Vec<Tile>>()
        })
        .collect();

    // Initialize grid
    let mut grid = Grid { grid: input_data };

    // Display grid
    grid.display();

    // Get rows and columns
    let nrows = grid.grid.len();
    let ncols = grid.grid[0].len();

    // Store energy scores for each grid configuration
    let mut scores: Vec<u64> = Vec::new();

    // From top
    for i in 0..ncols {
        grid.beam_photon_at((-1, i as i32), Direction::Down);
        scores.push(grid.score());
        grid.clear();
    }

    // From left
    for i in 0..nrows {
        grid.beam_photon_at((i as i32, -1), Direction::Right);
        scores.push(grid.score());
        grid.clear();
    }

    // From right
    for i in 0..nrows {
        grid.beam_photon_at((i as i32, ncols as i32), Direction::Left);
        scores.push(grid.score());
        grid.clear();
    }

    // From bottom
    for i in 0..ncols {
        grid.beam_photon_at((nrows as i32, i as i32), Direction::Up);
        scores.push(grid.score());
        grid.clear();
    }

    // Shoot a photon
    let max_score = *scores.iter().max().unwrap();

    // Submit score
    submit!(2, max_score);
}