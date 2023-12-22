use aocd::*;
use std::collections::HashMap;

/// Manhattan distance heuristic
fn heuristic(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.1 - b.1).abs() + (a.0 - b.0).abs()
}

/// Given a node, and a hashmap of nodes and their parents, backtrack into the full
/// path that arrives at the given node.
fn reconstruct_path(came_from: HashMap<(i32, i32), (i32, i32)>, current: Node) -> Vec<(i32, i32)> {
    let mut current_position = current.position;
    let mut path = vec![current_position];
    while came_from.contains_key(&current_position) {
        current_position = *came_from.get(&current_position).unwrap();
        path.push(current_position);
    }
    path.into_iter().rev().collect()
}

/// Store the maze as a grid. Each tile has an associated "heat loss" value.
#[derive(Debug, Clone)]
pub struct Grid {
    grid: Vec<Vec<u8>>,
}

impl Grid {
    /// Display the grid with the path highlighted
    pub fn display(&self, path: &[(i32, i32)]) {
        let mut loss: u64 = 0;
        for (i, row) in self.grid.iter().enumerate() {
            let mut row_str = String::new();
            for (j, value) in row.iter().enumerate() {
                if path.contains(&(i as i32, j as i32)) {
                    loss += *value as u64;
                    row_str += format!("[{value}]").to_string().as_str();
                } else {
                    row_str += format!(".{value}.").to_string().as_str();
                }
            }
            println!("{}", row_str);
        }
        println!("Heat loss: {}", loss);
    }
}

/// Vector of Nodes that is always sorted by f-score
#[derive(Default)]
pub struct NodeStack {
    nodes: Vec<Node>,
}

impl NodeStack {
    pub fn add_node(&mut self, node: Node, f_scores: &HashMap<(i32, i32), i32>) {
        self.nodes.push(node);
        self.nodes.sort_by(|a, b| {
            let a_score = *f_scores.get(&a.position).unwrap_or(&(i32::MAX / 2));
            let b_score = *f_scores.get(&b.position).unwrap_or(&(i32::MAX / 2));
            b_score.cmp(&a_score)
        });
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns the node with the minimum f-score
    pub fn pop(&mut self) -> Node {
        self.nodes.pop().unwrap()
    }

    pub fn contains(&self, position: (i32, i32)) -> bool {
        self.nodes.iter().any(|n| n.position == position)
    }
}

/// Struct to contain a position
pub struct Node {
    position: (i32, i32),
}

impl Node {
    /// Helper function for calculating positions of adjacent nodes
    pub fn adjacent(&self, i_limit: usize, j_limit: usize) -> Vec<(i32, i32)> {
        let i_limit = i_limit as i32;
        let j_limit = j_limit as i32;
        let candidates = [
            (self.position.0, self.position.1 - 1),
            (self.position.0, self.position.1 + 1),
            (self.position.0 - 1, self.position.1),
            (self.position.0 + 1, self.position.1),
        ];
        let mut adjacent = Vec::<(i32, i32)>::new();
        for c in candidates {
            if c.0 >= 0 && c.0 < i_limit && c.1 >= 0 && c.1 < j_limit {
                adjacent.push(c);
            }
        }
        adjacent
    }
}

/// Enum to describe which direction the path is going for a given tile
#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

/// Helper function to determine the direction from a position and a hashmap of node-parents
fn direction(position: (i32, i32), came_from: &HashMap<(i32, i32), (i32, i32)>) -> Direction {
    if let Some(came_from_position) = came_from.get(&position) {
        if came_from_position.0 < position.0 {
            Direction::Down
        } else if came_from_position.0 > position.0 {
            Direction::Up
        } else if came_from_position.1 < position.1 {
            Direction::Right
        } else if came_from_position.1 > position.1 {
            Direction::Left
        } else {
            unreachable!("Impossible position configuration.")
        }
    } else {
        Direction::None
    }
}

/// Get the number of times the path has gone in the same direction up until a given point
fn same_direction_count(position: (i32, i32), came_from: &HashMap<(i32, i32), (i32, i32)>) -> i32 {
    let mut current_position = position;
    let mut came_from_position = if let Some(pos) = came_from.get(&position) {
        *pos
    } else {
        return 1;
    };
    let mut count = 1;
    'chain: while direction(current_position, came_from) == direction(came_from_position, came_from)
    {
        count += 1;
        current_position = came_from_position;
        came_from_position = if let Some(pos) = came_from.get(&came_from_position) {
            *pos
        } else {
            break 'chain;
        }
    }
    count
}

/// A* path-finding algorithm
fn a_star(maze: &Grid, start: (i32, i32), end: (i32, i32)) -> Vec<(i32, i32)> {
    let i_limit = maze.grid.len();
    let j_limit = maze.grid[0].len();

    let mut f_scores = HashMap::<(i32, i32), i32>::new();
    f_scores.insert(start, heuristic(start, end));

    let mut open_set = NodeStack::default();
    open_set.add_node(Node { position: start }, &f_scores);

    let mut came_from = HashMap::<(i32, i32), (i32, i32)>::new();

    let mut g_scores = HashMap::<(i32, i32), i32>::new();
    g_scores.insert(start, 0);

    while !open_set.is_empty() {
        let current_node = open_set.pop();

        if current_node.position == end {
            return reconstruct_path(came_from, current_node);
        }

        let neighbors = current_node.adjacent(i_limit, j_limit);

        'consideration: for neighbor in neighbors.iter() {
            let current_g_score = *g_scores
                .get(&current_node.position)
                .unwrap_or(&(i32::MAX / 2));
            let edge_weight = maze.grid[neighbor.0 as usize][neighbor.1 as usize] as i32;
            let tentative_g_score = current_g_score + edge_weight;
            let neighbor_g_score = *g_scores.get(neighbor).unwrap_or(&(i32::MAX / 2));
            if tentative_g_score < neighbor_g_score {
                let prior_came_from_option = came_from.insert(*neighbor, current_node.position);
                if same_direction_count(*neighbor, &came_from) > 3 {
                    if let Some(prior) = prior_came_from_option {
                        came_from.insert(*neighbor, prior);
                    } else {
                        came_from.remove(neighbor);
                    }
                    continue 'consideration;
                }
                g_scores.insert(*neighbor, tentative_g_score);
                let h = heuristic(*neighbor, end);
                f_scores.insert(*neighbor, tentative_g_score + h);
                if !open_set.contains(*neighbor) {
                    open_set.add_node(
                        Node {
                            position: *neighbor,
                        },
                        &f_scores,
                    )
                }
            }
        }
    }

    Vec::new()
}

#[aocd(2023, 17, "src/day17/test.txt")]
pub fn solution1() {
    let input_data: Vec<Vec<u8>> = input!()
        .split('\n')
        .map(|s| {
            let nodes = s
                .trim()
                .chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect();
            nodes
        })
        .collect();

    let maze = Grid { grid: input_data };
    let i_limit = maze.grid.len() as i32;
    let j_limit = maze.grid[0].len() as i32;

    let path = a_star(&maze, (0, 0), (i_limit - 1, j_limit - 1));
    maze.display(&path);
}

#[aocd(2023, 17)]
pub fn solution2() {}
