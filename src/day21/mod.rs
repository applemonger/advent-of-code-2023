use aocd::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coord(i32, i32);

impl Coord {
    fn from_i32(i: i32, j: i32) -> Coord {
        Coord(i, j)
    }

    fn from_usize(i: usize, j: usize) -> Coord {
        Coord(i as i32, j as i32)
    }

    fn index(&self) -> (usize, usize) {
        (self.0 as usize, self.1 as usize)
    }

    fn adjacent(&self) -> [Coord; 4] {
        [
            Coord::from_i32(self.0 - 1, self.1),
            Coord::from_i32(self.0 + 1, self.1),
            Coord::from_i32(self.0, self.1 - 1),
            Coord::from_i32(self.0, self.1 + 1)
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileType {
    Start,
    Terrain,
    Empty
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    tile_type: TileType,
    position: Coord,
    on: bool
}

impl Tile {
    fn new(c: char, i: usize, j: usize) -> Tile {
        let tile_type = match c {
            '#' => TileType::Terrain,
            '.' => TileType::Empty,
            'S' => TileType::Start,
            _ => unreachable!("Undefined tile type")
        };
        Tile {
            tile_type,
            position: Coord::from_usize(i, j),
            on: false
        }
    }

    fn str(&self) -> char {
        match (self.tile_type, self.on) {
            (TileType::Terrain, _) => '#',
            (TileType::Empty, true) => 'O',
            (TileType::Empty, false) => '.',
            (TileType::Start, _) => 'S' 
        }
    }
}

struct Grid {
    data: Vec<Vec<Tile>>,
    on: Vec<Coord>,
    nrows: usize,
    ncols: usize
}

impl Grid {
    fn read_from_string(input_data: String) -> Grid {
        let mut start_i = 0;
        let mut start_j = 0;
        let data: Vec<Vec<Tile>> = input_data
            .lines()
            .enumerate()
            .map(|(i, s)| {
                s.chars()
                    .enumerate()
                    .map(|(j, c)| {
                        if c == 'S' {
                            start_i = i;
                            start_j = j;
                        }
                        Tile::new(c, i, j)
                    })
                    .collect()
            })
            .collect();
        let nrows = data.len();
        let ncols = data[0].len();
        let start_coord = Coord::from_usize(start_i, start_j);
        Grid {
            data,
            on: vec![start_coord],
            nrows,
            ncols
        }
    }

    fn tile(&self, coord: &Coord) -> &Tile {
        let (i, j) = coord.index();
        &self.data[i][j]
    }

    fn out_of_bounds(&self, coord: &Coord) -> bool {
        coord.0 < 0 || coord.0 >= self.nrows as i32 || coord.1 < 0 || coord.1 >= self.ncols as i32
    }

    fn display(&self) {
        for row in self.data.iter() {
            let row_str: String = row
                .iter()
                .map(|t| t.str())
                .collect();
            println!("{}", row_str);
        }
    }

    fn traverse(&mut self) {
        self.on = self.on
            .iter()
            .flat_map(|coord| {
                coord.adjacent()
            })
            .filter(|coord| {
                !self.out_of_bounds(coord) && self.tile(coord).tile_type != TileType::Terrain
            })
            .collect();
        self.on.sort();
        self.on.dedup();
    }

    fn sync_on(&mut self) {
        for row in self.data.iter_mut() {
            for tile in row.iter_mut() {
                tile.on = false;
            }
        }
        for coord in self.on.iter() {
            let (i, j) = coord.index();
            self.data[i][j].on = true;
        }
    }

    fn count_on(&self) -> usize {
        self.on.len()
    }
}

#[aocd(2023, 21)]
pub fn solution1() {
    let input_data = input!();
    let mut grid = Grid::read_from_string(input_data);
    for _ in 0..64 {
        grid.traverse();
    }
    grid.sync_on();
    grid.display();
    submit!(1, grid.count_on());
}   

#[aocd(2023, 21)]
pub fn solution2() {
    let input_data = input!();
}