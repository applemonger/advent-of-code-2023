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

#[aocd(2023, 16)]
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

    grid.display();

    // Create first photon
    let initial_photon = Photon {
        position: (0, 0),
        direction: Direction::Right
    };
    
    // List to store all photons
    let mut photons = vec![initial_photon];

    // Traverse the grid
    while !photons.is_empty() {
        // Transform directions based on tile position
        photons = photons
            .iter_mut()
            .flat_map(|p| {
                let tile = grid.tile(p.position).unwrap();
                let vec_photons_t = tile.transform(p);
                vec_photons_t
            })
            .collect();
        
        // Move each photon
        photons.retain_mut(|p| p.travel(&mut grid));
    }

    // Submit score
    submit!(1, grid.score());
}

#[aocd(2023, 16)]
pub fn solution2() {

}