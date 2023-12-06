use regex::Regex;
use std::fs;

pub fn solution1() {
    let contents = fs::read_to_string("src/day2/input.txt").expect("Unable to read file.");
    let possible_games_sum: i32 = contents
        .split('\n')
        .map(|s| {
            // Extract game id
            let re = Regex::new(r"(\d*):").unwrap();
            let game_id = &re.captures(s).unwrap()[1];
            let game_id: i32 = game_id.parse().unwrap();

            // Extract subsets
            let re = Regex::new(r":(.*)").unwrap();
            let game_data = &re.captures(s).unwrap()[1];
            let impossible_subset_detected = game_data
                .split(';')
                // Determine if a given subset is possible or not
                .any(|ss| {
                    // Extract number of each cube in subset
                    let red_regex = Regex::new(r"(\d*) red").unwrap();
                    let green_regex = Regex::new(r"(\d*) green").unwrap();
                    let blue_regex = Regex::new(r"(\d*) blue").unwrap();

                    // Number of each cube (defaults)
                    let mut red_num: i32 = 0;
                    let mut green_num: i32 = 0;
                    let mut blue_num: i32 = 0;

                    // Add observed cubes
                    if let Some(red_capture) = red_regex.captures(ss) {
                        red_num += red_capture[1].parse::<i32>().unwrap();
                    }

                    if let Some(green_capture) = green_regex.captures(ss) {
                        green_num += green_capture[1].parse::<i32>().unwrap();
                    }

                    if let Some(blue_capture) = blue_regex.captures(ss) {
                        blue_num += blue_capture[1].parse::<i32>().unwrap();
                    }

                    // Determine if any cube counts are impossible
                    (red_num > 12) || (green_num > 13) || (blue_num > 14)
                });

            // Return game id if impossible subsets are detected, 0 otherwise
            if impossible_subset_detected {
                0
            } else {
                game_id
            }
        })
        .sum();

    println!("Sum of possible game IDs: {}", possible_games_sum);
}

fn get_cube_count(s: &str, regex: &str) -> i32 {
    let re = Regex::new(regex).unwrap();
    let mut count: i32 = 0;
    if let Some(capture) = re.captures(s) {
        count += capture[1].parse::<i32>().unwrap();
    }
    count
}

pub fn solution2() {
    let contents = fs::read_to_string("src/day2/input.txt").expect("Unable to read file.");
    let power_games_sum: i32 = contents
        .split('\n')
        .map(|s| {
            // Extract subsets
            let re = Regex::new(r":(.*)").unwrap();
            let game_data = &re.captures(s).unwrap()[1];

            // Extract maximum number of blue cubes
            let maximum_blue = game_data
                .split(';')
                .map(|ss| get_cube_count(ss, r"(\d*) blue"))
                .max()
                .unwrap();

            // Extract maximum number of blue cubes
            let maximum_red = game_data
                .split(';')
                .map(|ss| get_cube_count(ss, r"(\d*) red"))
                .max()
                .unwrap();

            // Extract maximum number of green cubes
            let maximum_green = game_data
                .split(';')
                .map(|ss| get_cube_count(ss, r"(\d*) green"))
                .max()
                .unwrap();

            maximum_blue * maximum_red * maximum_green
        })
        .sum();

    println!("Power of game IDs: {}", power_games_sum);
}
