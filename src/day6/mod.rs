use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    fn calculate_ways_to_win(&self) -> f64 {
        let t = self.time as f64;
        let d = self.distance as f64;
        let lower = ((-t + (t.powi(2) - 4. * d).sqrt()) / -2.).ceil();
        let upper = ((-t - (t.powi(2) - 4. * d).sqrt()) / -2.).floor();
        upper - lower + 1.
    }
}

fn read_data(path: &str) -> Vec<Race> {
    // Read file
    let contents = fs::read_to_string(path).unwrap();
    let lines: Vec<&str> = contents.split('\n').collect();

    // Read times
    let time_regex = Regex::new(r"(\d+)").unwrap();
    let times: Vec<i64> = time_regex
        .captures_iter(lines[0])
        .map(|c| {
            let time_str = c.get(0).unwrap().as_str();
            let time: i64 = time_str.parse().unwrap();
            time
        })
        .collect();

    // Read distances
    let dist_regex = Regex::new(r"(\d+)").unwrap();
    let dists: Vec<i64> = dist_regex
        .captures_iter(lines[1])
        .map(|c| {
            let dist_str = c.get(0).unwrap().as_str();
            let dist: i64 = dist_str.parse().unwrap();
            dist
        })
        .collect();

    // Zip vectors
    let mut races: Vec<Race> = Vec::new();
    for (t, d) in times.iter().zip(dists.iter()) {
        races.push(Race {
            time: *t,
            distance: *d,
        });
    }
    races
}

pub fn solution1() {
    let races = read_data("src/day6/input.txt");
    let score: f64 = races
        .iter()
        .map(|race| race.calculate_ways_to_win())
        .product();
    println!("{:?}", score);
}

pub fn solution2() {
    let races = read_data("src/day6/input2.txt");
    let score: f64 = races
        .iter()
        .map(|race| race.calculate_ways_to_win())
        .product();
    println!("{:?}", score);
}
