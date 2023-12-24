use aocd::*;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
struct Coord {
    x: u32,
    y: u32,
    z: u32
}

impl Coord {
    fn from_str(s: &str) -> Coord {
        let values: Vec<u32> = s
            .split(',')
            .map(|c| c.parse::<u32>().unwrap())
            .collect();
        Coord {
            x: values[0],
            y: values[1],
            z: values[2]
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Brick {
    a: Coord,
    b: Coord
}

impl Brick {
    fn from_str(s: &str) -> Brick {
        let mut coords: Vec<Coord> = s.split('~').map(Coord::from_str).collect();
        let b = coords.pop().unwrap();
        let a = coords.pop().unwrap();
        Brick { a, b }
    }

    fn x_range(&self) -> (u32, u32) {
        let max = self.a.x.max(self.b.x);
        let min = self.a.x.min(self.b.x);
        (min, max)
    }

    fn y_range(&self) -> (u32, u32) {
        let max = self.a.y.max(self.b.y);
        let min = self.a.y.min(self.b.y);
        (min, max)
    }

    fn z_range(&self) -> (u32, u32) {
        let max = self.a.z.max(self.b.z);
        let min = self.a.z.min(self.b.z);
        (min, max)
    }

    fn overlaps(range_a: (u32, u32), range_b: (u32, u32)) -> u8 {
        if range_a.0 <= range_b.1 && range_a.1 >= range_b.0 {
            1
        } else {
            0
        }
    }

    fn intersects(&self, other: &Brick) -> bool {
        let overlap_x = Brick::overlaps(self.x_range(), other.x_range());
        let overlap_y = Brick::overlaps(self.y_range(), other.y_range());
        let overlap_z = Brick::overlaps(self.z_range(), other.z_range());
        let overlap = overlap_x + overlap_y + overlap_z;
        overlap == 3
    }

    fn min_z(&self) -> u32 {
        self.a.z.min(self.b.z)
    }

    fn max_z(&self) -> u32 {
        self.a.z.max(self.b.z)
    }

    fn drop(&mut self, spaces: u32) {
        self.a.z -= spaces;
        self.b.z -= spaces;
    }
}

fn drop_bricks(mut bricks: Vec<Brick>) -> (u32, Vec<Brick>) {
    // Number of bricks fallen
    let mut bricks_fallen = 0;

    // Sort by highest to lowest
    bricks.sort_by_key(|b| b.min_z());

    // Drop first brick
    let brick = bricks.get_mut(0).unwrap();
    let min_z = brick.min_z();
    let drop_distance = min_z - 1;
    brick.drop(drop_distance);
    if drop_distance > 0 {
        bricks_fallen += 1;
    }

    // Drop the rest
    let mut index = 1;
    'iterbricks: loop {
        let mut brick_fell = false;

        // Past last brick, stop loop
        if index == bricks.len() {
            break 'iterbricks;
        }

        // Get the bottom of the current brick
        let z_min = bricks[index].min_z();

        // Stop if at the bottom already
        if z_min == 1 {
            index += 1;
            continue 'iterbricks;
        }

        // Get the top of all the bricks below the current brick
        let z_max_option = bricks
            .iter()
            .filter(|b| b.max_z() < z_min)
            .map(|b| b.max_z())
            .max();

        // If there is a max, drop the current brick to one above that one
        if let Some(z_max) = z_max_option {
            let drop_distance = z_min - z_max - 1;
            bricks[index].drop(drop_distance);
            if drop_distance > 0 {
                brick_fell = true;
            }
        }

        // Keep dropping it until it is blocked by another brick or the floor
        // Initial drop distance is 0
        let mut drop_distance = 0;
        // Get a copy of the current brick
        let mut brick = bricks[index].clone();
        'dropping: loop {
            // Drop it 1 space down
            brick.drop(1);
            let min_z = brick.min_z();

            // If we've hit the floor, stop dropping
            if min_z == 0 {
                break 'dropping;
            }

            // Otherwise, check for collisions with any of the bricks immediately below it
            let collision = bricks
                .iter()
                .filter(|b| b.max_z() == min_z)
                .any(|b| brick.intersects(b));

            // If there is a collision, stop dropping
            if collision {
                break 'dropping;
            }

            // Keep track of how far we've dropped
            drop_distance += 1;
        }

        // Apply the fall to the actual brick
        bricks[index].drop(drop_distance);
        if drop_distance > 0 {
            brick_fell = true;
        }

        // Count fallen brick
        if brick_fell {
            bricks_fallen += 1;
        }

        // Go the next brick
        index += 1;
    }

    (bricks_fallen, bricks)
}

fn is_packed(bricks: Vec<Brick>) -> bool {
    let n_bricks = bricks.len();
    // For each brick...
    'simulation: for i in 0..n_bricks {
        // Create a copy of the current brick for simulating
        let mut simulate_brick = bricks[i].clone();
        let min_z = simulate_brick.min_z();
        // If the current brick is on the floor, go to the next brick
        if min_z == 1 {
            continue 'simulation;
        }
        // Drop the brick one space
        simulate_brick.drop(1);
        // Check if there is a collision with any other bricks immediately below it
        let collision = bricks
            .iter()
            .filter(|b| b.max_z() == min_z - 1)
            .any(|b| simulate_brick.intersects(b));
        // If there isn't, then the brick can move, and the stack is not packed
        if !collision {
            return false;
        }
    }
    // The loop has gone through each brick without detecting any free room
    true
}

#[aocd(2023, 22)]
pub fn solution1() {
    let input_data = input!();

    // Read bricks
    let bricks: Vec<Brick> = input_data
        .lines()
        .map(|line| Brick::from_str(line))
        .collect();

    // Initial brick drop
    let (_, settled_bricks) = drop_bricks(bricks);
    let n_bricks = settled_bricks.len();

    // For each brick, check if it can be removed
    let mut destroyable = 0;
    for i in 0..n_bricks {
        let mut simulation_bricks = settled_bricks.clone();
        simulation_bricks.remove(i);
        let packed = is_packed(simulation_bricks);
        if packed {
            destroyable += 1;
        }
    }

    submit!(1, destroyable);
}

#[aocd(2023, 22)]
pub fn solution2() {
    let input_data = input!();

    // Read bricks
    let bricks: Vec<Brick> = input_data
        .lines()
        .map(|line| Brick::from_str(line))
        .collect();

    // Initial brick drop
    let (_, settled_bricks) = drop_bricks(bricks);
    let n_bricks = settled_bricks.len();

    // For each brick, check if it can be removed
    let mut bricks_fallen = 0;
    for i in 0..n_bricks {
        println!("Simulation {i}/{n_bricks}");
        let mut simulation_bricks = settled_bricks.clone();
        simulation_bricks.remove(i);
        let (bricks_fallen_incr, _) = drop_bricks(simulation_bricks);
        bricks_fallen += bricks_fallen_incr;
    }

    submit!(2, bricks_fallen);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersects() {
        let brick_a = Brick::from_str("0,1,0~2,1,0");
        let brick_b = Brick::from_str("1,0,0~1,2,0");
        assert!(brick_a.intersects(&brick_b));
    }

    #[test]
    fn does_not_intersect() {
        let brick_a = Brick::from_str("0,1,0~2,1,0");
        let brick_b = Brick::from_str("10,0,0~10,2,0");
        assert!(!brick_a.intersects(&brick_b));
    }

    #[test]
    fn intersects_cube() {
        let brick_a = Brick::from_str("1,1,1~1,1,1");
        let brick_b = Brick::from_str("0,1,1~2,1,1");
        assert!(brick_a.intersects(&brick_b));
    }

    #[test]
    fn does_not_intersect_cube() {
        let brick_a = Brick::from_str("1,1,1~1,1,1");
        let brick_b = Brick::from_str("0,1,0~2,1,0");
        assert!(!brick_a.intersects(&brick_b));
    }
}