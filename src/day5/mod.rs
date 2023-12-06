use regex::Regex;
use std::fs;

#[derive(Debug)]
struct MapEntry {
    length: u32,
    destination: u32,
    source: u32,
}

impl MapEntry {
    pub fn from_str(s: &str) -> MapEntry {
        let values_regex = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
        let captures = values_regex.captures(s).unwrap();
        let length: u32 = captures[3].parse().unwrap();
        let destination: u32 = captures[1].parse().unwrap();
        let source: u32 = captures[2].parse().unwrap();

        MapEntry {
            length,
            destination,
            source,
        }
    }

    pub fn map(&self, value: u32) -> Option<u32> {
        let source_range = self.source..(self.source + self.length);
        if source_range.contains(&value) {
            let offset = value - self.source;
            Some(self.destination + offset)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Map {
    entries: Vec<MapEntry>,
}

impl Map {
    pub fn from_file(file: &str) -> Map {
        let contents = fs::read_to_string(file).expect("Unable to read file.");

        let entries: Vec<MapEntry> = contents.split('\n').map(MapEntry::from_str).collect();

        Map { entries }
    }

    pub fn lookup(&self, value: u32) -> u32 {
        for entry in &self.entries {
            if let Some(destination) = entry.map(value) {
                return destination;
            }
        }
        value
    }
}

fn read_seeds() -> Vec<u32> {
    let contents = fs::read_to_string("src/day5/seeds.txt").expect("Unable to read seeds.");
    let seeds: Vec<u32> = contents
        .split(' ')
        .map(|s| {
            let value: u32 = s.parse().unwrap();
            value
        })
        .collect();
    seeds
}

pub fn solution1() {
    // Read in maps
    let fertilizer_to_water_map = Map::from_file("src/day5/fertilizer_to_water_map.txt");
    let humidity_to_location_map = Map::from_file("src/day5/humidity_to_location_map.txt");
    let light_to_temperature_map = Map::from_file("src/day5/light_to_temperature_map.txt");
    let seed_to_soil_map = Map::from_file("src/day5/seed_to_soil_map.txt");
    let soil_to_fertilizer_map = Map::from_file("src/day5/soil_to_fertilizer_map.txt");
    let temperature_to_humidity_map = Map::from_file("src/day5/temperature_to_humidity_map.txt");
    let water_to_light_map = Map::from_file("src/day5/water_to_light_map.txt");

    // Read in seeds
    let seeds = read_seeds();

    // Map seeds
    let mut locations: Vec<u32> = Vec::new();
    for seed in seeds {
        let soil = seed_to_soil_map.lookup(seed);
        let fertilizer = soil_to_fertilizer_map.lookup(soil);
        let water = fertilizer_to_water_map.lookup(fertilizer);
        let light = water_to_light_map.lookup(water);
        let temperature = light_to_temperature_map.lookup(light);
        let humidity = temperature_to_humidity_map.lookup(temperature);
        let location = humidity_to_location_map.lookup(humidity);
        locations.push(location);
    }

    // Lowest location
    let lowest_location = locations.into_iter().min().unwrap();
    println!("Lowest location: {}", lowest_location);
}

pub fn solution2() {
    // Read in maps
    let fertilizer_to_water_map = Map::from_file("src/day5/fertilizer_to_water_map.txt");
    let humidity_to_location_map = Map::from_file("src/day5/humidity_to_location_map.txt");
    let light_to_temperature_map = Map::from_file("src/day5/light_to_temperature_map.txt");
    let seed_to_soil_map = Map::from_file("src/day5/seed_to_soil_map.txt");
    let soil_to_fertilizer_map = Map::from_file("src/day5/soil_to_fertilizer_map.txt");
    let temperature_to_humidity_map = Map::from_file("src/day5/temperature_to_humidity_map.txt");
    let water_to_light_map = Map::from_file("src/day5/water_to_light_map.txt");

    // Read in seeds
    let seed_ranges = read_seeds();
    let mut seed_starts: Vec<u32> = Vec::new();
    let mut range_lengths: Vec<u32> = Vec::new();
    for (index, value) in seed_ranges.iter().enumerate() {
        if index % 2 == 0 {
            seed_starts.push(*value);
        } else {
            range_lengths.push(*value);
        }
    }
    let seed_ranges = seed_starts.iter().zip(range_lengths.iter());

    // Map seeds
    println!("Ah, here we go...");
    let mut lowest_location = u32::MAX;
    for (seed_start, seed_range) in seed_ranges {
        println!("Seed start: {}, Seed range: {}", seed_start, seed_range);
        for i in 0..*seed_range {
            let soil = seed_to_soil_map.lookup(seed_start + i);
            let fertilizer = soil_to_fertilizer_map.lookup(soil);
            let water = fertilizer_to_water_map.lookup(fertilizer);
            let light = water_to_light_map.lookup(water);
            let temperature = light_to_temperature_map.lookup(light);
            let humidity = temperature_to_humidity_map.lookup(temperature);
            let location = humidity_to_location_map.lookup(humidity);
            if location < lowest_location {
                lowest_location = location;
            }
        }
    }
    println!("Lowest location: {}", lowest_location);
}
