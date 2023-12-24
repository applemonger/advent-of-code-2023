use std::collections::HashMap;
use aocd::*;

const LARGE_NUMBER: i32 = i32::MAX / 2;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Coord {
    i: i32,
    j: i32,
}

impl Coord {
    fn new(i: i32, j: i32) -> Coord {
        Coord { i, j }
    }

    fn manhattan(&self, other: &Coord) -> i32 {
        // (self.i - other.i).abs() + (self.j - other.j).abs()
        0
    }

    fn neighbors(&self) -> [Coord; 4] {
        [
            Coord::new(self.i - 1, self.j),
            Coord::new(self.i + 1, self.j),
            Coord::new(self.i, self.j - 1),
            Coord::new(self.i, self.j + 1)
        ]
    }

    fn as_usize(&self) -> (usize, usize) {
        (self.i as usize, self.j as usize)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TileType {
    Forest,
    Path,
    SlopeUp,
    SlopeDown,
    SlopeLeft,
    SlopeRight
}

impl TileType {
    fn from_char(c: char) -> TileType {
        match c {
            '#' => TileType::Forest,
            '.' => TileType::Path,
            '^' => TileType::SlopeUp,
            'v' => TileType::SlopeDown,
            '>' => TileType::SlopeRight,
            '<' => TileType::SlopeLeft,
            _ => unreachable!("Undefined tile type.")
        }
    }
}

#[derive(Clone, Copy)]
struct Tile {
    tile_type: TileType,
    position: Coord,
    visited: bool
}

impl Tile {
    fn new(c: char, i: usize, j: usize) -> Tile {
        Tile {
            tile_type: TileType::from_char(c),
            position: Coord::new(i as i32, j as i32),
            visited: false
        }
    }

    fn as_char(&self) -> char {
        match (&self.tile_type, &self.visited) {
            (_, true) => 'O',
            (TileType::Forest, _) => '#',
            (TileType::Path, _) => '.',
            (TileType::SlopeUp, _) => '^',
            (TileType::SlopeDown, _) => 'v',
            (TileType::SlopeLeft, _) => '<',
            (TileType::SlopeRight, _) => '>'
        }
    }
}


/// Vector of Nodes that is always sorted by f-score
#[derive(Default)]
struct CoordHeap {
    coords: Vec<Coord>,
}

impl CoordHeap {
    fn add_coord(&mut self, node: Coord, f_scores: &HashMap<Coord, i32>) {
        self.coords.push(node);
        self.coords.sort_by(|a, b| {
            let a_score = *f_scores.get(&a).unwrap();
            let b_score = *f_scores.get(&b).unwrap();
            a_score.cmp(&b_score)
        });
    }

    fn is_empty(&self) -> bool {
        self.coords.is_empty()
    }

    /// Returns the node with the minimum f-score
    fn pop(&mut self) -> Coord {
        self.coords.pop().unwrap()
    }

    fn contains(&self, position: &Coord) -> bool {
        self.coords.iter().any(|n| n == position)
    }
}


struct Grid {
    data: Vec<Vec<Tile>>,
    nrows: usize,
    ncols: usize,
    start: Coord,
    end: Coord
}

impl Grid {
    fn from_string(s: String) -> Grid {
        // Extract tiles from string
        let data: Vec<Vec<Tile>> = s.lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| {
                        Tile::new(c, i, j)
                    })
                    .collect::<Vec<Tile>>()
            })
            .collect();

        // Count rows and columns
        let nrows = data.len();
        let ncols = data[0].len();

        // Find start tile
        let mut start = Coord::new(0, 0);
        for tile in data[0].iter() {
            if tile.tile_type == TileType::Path {
                start = tile.position;
            }
        }

        // Find end tile
        let mut end = Coord::new(0, 0);
        for tile in data[nrows-1].iter() {
            if tile.tile_type == TileType::Path {
                end = tile.position;
            }
        }

        Grid {
            data,
            nrows,
            ncols,
            start,
            end
        }
    }

    fn display(&self) {
        for row in self.data.iter() {
            let row_str: String = row
                .iter()
                .map(|t| t.as_char())
                .collect();
            println!("{}", row_str);
        }
        println!("{}", self.score() - 1);
    }

    fn visit(&mut self, coord: &Coord) {
        let (i, j) = coord.as_usize();
        self.data[i][j].visited = true;
    }

    fn out_of_bounds(&self, coord: &Coord) -> bool {
        coord.i < 0 || coord.i > self.nrows as i32 || coord.j < 0 || coord.j > self.ncols as i32
    }

    fn tile(&self, coord: &Coord) -> &Tile {
        let (i, j) = coord.as_usize();
        &self.data[i][j]
    }
    
    fn a_star(&mut self) {
        let mut f_scores = HashMap::<Coord, i32>::new();
        for row in self.data.iter() {
            for tile in row.iter() {
                f_scores.insert(tile.position, LARGE_NUMBER);
            }
        }
        f_scores.insert(self.start, self.start.manhattan(&self.end));

        let mut open_set = CoordHeap::default();
        open_set.add_coord(self.start, &f_scores);

        let mut came_from = HashMap::<Coord, Coord>::new();

        let mut g_scores = HashMap::<Coord, i32>::new();
        for row in self.data.iter() {
            for tile in row.iter() {
                g_scores.insert(tile.position, LARGE_NUMBER);
            }
        }
        g_scores.insert(self.start, 0);

        while !open_set.is_empty() {
            // Get the node with the lowest f-score
            let current = open_set.pop();

            // If we're at the end, mark the path that was used to arrive there
            if current == self.end {
                let mut path_node = current;
                loop {
                    self.visit(&path_node);
                    if let Some(parent) = came_from.get(&path_node) {
                        path_node = *parent;
                    } else {
                        return;
                    }
                }
            }

            'traversal: for neighbor in current.neighbors() {
                // Skip neighbors that are out of bounds
                if self.out_of_bounds(&neighbor) {
                    continue 'traversal;
                }
                // Skip neighbors that are Forest (obstacles)
                if self.tile(&neighbor).tile_type == TileType::Forest {
                    continue 'traversal;
                }
                // Skip neighbors that slope against current position
                if self.tile(&neighbor).tile_type == TileType::SlopeUp && neighbor.i > current.i {
                    continue 'traversal;
                }
                if self.tile(&neighbor).tile_type == TileType::SlopeDown && neighbor.i < current.i {
                    continue 'traversal;
                }
                if self.tile(&neighbor).tile_type == TileType::SlopeLeft && neighbor.j > current.j {
                    continue 'traversal;
                }
                if self.tile(&neighbor).tile_type == TileType::SlopeRight && neighbor.j < current.j {
                    continue 'traversal;
                }
                // Scoring
                let tentative_g_score = g_scores.get(&current).unwrap() + 1;
                let neighbor_g_score = *g_scores.get(&neighbor).unwrap();
                if tentative_g_score < neighbor_g_score {
                    came_from.insert(neighbor, current);
                    g_scores.insert(neighbor, tentative_g_score);
                    let h = neighbor.manhattan(&self.end);
                    f_scores.insert(neighbor, tentative_g_score + h);
                    if !open_set.contains(&neighbor) {
                        open_set.add_coord(neighbor, &f_scores);
                    }
                }
            }
        }
    }

    fn score(&self) -> u32 {
        self.data
            .iter()
            .map(|row| {
                row.iter()
                    .map(|t| if t.visited { 1 } else { 0 })
                    .sum::<u32>()
            })
            .sum()
    }
}

#[aocd(2023, 23, "src/day23/test.txt")]
pub fn solution1() {
    let input_data = input!();
    let mut grid = Grid::from_string(input_data);
    grid.a_star();
    grid.display();
}

#[aocd(2023, 23)]
pub fn solution2() {
    let input_data = input!();
}

// #[cfg(tests)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_slope_up() {
//         let input_data = String::from(".##");
//         let mut grid = Grid::from_string("")
//     }
// }