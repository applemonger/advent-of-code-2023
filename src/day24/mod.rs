use aocd::*;

#[derive(Debug, Clone, Copy)]
struct Vector {
    x: f64,
    y: f64,
    z: f64
}

impl Vector {
    fn from_str(s: &str) -> Vector {
        let vector: Vec<f64> = s
            .split(", ")
            .map(|x| {x.trim().parse::<f64>().unwrap()})
            .collect();
        Vector {
            x: vector[0],
            y: vector[1],
            z: vector[2]
        }
    }

    fn as_string(&self) -> String {
        let s = format!("{}, {}, {}", self.x, self.y, self.z);
        s
    }
}

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    position: Vector,
    velocity: Vector,
}

impl Hailstone {
    fn from_str(s: &str) -> Hailstone {
        let mut vectors: Vec<Vector> = s
            .split('@')
            .map(|x| Vector::from_str(x))
            .collect();
        let velocity = vectors.pop().unwrap();
        let position = vectors.pop().unwrap();
        Hailstone {
            position,
            velocity
        }
    }

    fn a(&self) -> f64 {
        self.velocity.y / self.velocity.x
    }

    fn b(&self) -> f64 {
        -1.
    }

    fn c(&self) -> f64 {
        -1. * self.position.x * self.a() + self.position.y
    }

    fn t_from_x(&self, x: f64) -> f64 {
        (x - self.position.x) / self.velocity.x
    }

    fn as_string(&self) -> String {
        let s = format!("{} @ {}", self.position.as_string().as_str(), self.velocity.as_string().as_str());
        s
    }

    fn collision(&self, other: &Hailstone) -> Collision {
        // println!("Hailstone A: {}", other.as_string());
        // println!("Hailstone B: {}", self.as_string());
        let epsilon = 1.0e-9;
        let denominator = self.a() * other.b() - other.a() * self.b();
        if denominator.abs() < epsilon {
            // println!("Will never collide.");
            Collision::None
        } else {
            let x = (self.b() * other.c() - other.b() * self.c()) / denominator;
            let y = (other.a() * self.c() - self.a() * other.c()) / denominator;
            if self.t_from_x(x) < epsilon || other.t_from_x(x) < epsilon {
                // println!("Have collided in the past at ({}, {})", x, y);
                Collision::Past(x, y)
            } else {
                // println!("{} {}", self.t_from_x(x), other.t_from_x(x));
                // println!("Will collide in the future at ({}, {})", x, y);
                Collision::Future(x, y)
            }
        }
    }
}

enum Collision {
    Future(f64, f64),
    Past(f64, f64),
    None
}

#[aocd(2023, 24)]
pub fn solution1() {
    let input_data = input!();
    let hailstones: Vec<Hailstone> = input_data
        .lines()
        .map(Hailstone::from_str)
        .collect();
    let n = hailstones.len();
    let mut collisions = 0;
    let x_range = (200000000000000.0, 400000000000000.0);
    let y_range = x_range;
    for i in 0..n {
        let a = &hailstones[i];
        for j in 0..i {
            let b = &hailstones[j];
            match a.collision(b) {
                Collision::Future(x, y) => {
                    if x >= x_range.0 && x <= x_range.1 && y >= y_range.0 && y <= y_range.1 {
                        collisions += 1;
                    }
                }
                Collision::Past(_, _) | Collision::None => {}
            };
        }
    }
    submit!(1, collisions);
}

#[aocd(2023, 24)]
pub fn solution2() {
    let input_data = input!();
}