use aocd::*;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct History {
    data: Vec<i64>
}

impl History {
    pub fn extrapolate_score(&self) -> i64 {
        let mut vecs: Vec<Vec<i64>> = Vec::new();
        vecs.push(self.data.clone());

        'report: loop {
            // Copy all elements except the last
            let mut subtract_vec = vecs.last().unwrap().clone();
            subtract_vec.pop();

            // Insert 0 to shift the elements forward
            subtract_vec.insert(0, 0);

            // Subtract those elements from the original vector (x_n - x_n-1)
            let mut diff_vec: Vec<i64> = vecs.last().unwrap().iter().zip(subtract_vec.iter())
                .map(|(x, y)| {
                    x - y
                })
                .collect();

            // Remove the first element
            diff_vec.remove(0);

            // Store the new vector
            vecs.push(diff_vec);

            // If the vector is all zeroes
            if vecs.last().unwrap().iter().all(|n| *n == 0) {
                break 'report;
            }
        }

        let mut extrapolation_vec = Vec::<i64>::new();

        for (i, vec) in vecs.iter_mut().rev().enumerate() {
            if i == 0 {
                // Extrapolate last vector (zeroes)
                extrapolation_vec.push(0);
            } else if i == 1 {
                // Extrapolate second-to-last vector (constants)
                let num = vec.first().unwrap().clone();
                extrapolation_vec.push(num);
            } else {
                // Extrapolate for any higher vecs
                let num = vec.last().unwrap() + extrapolation_vec.last().unwrap();
                extrapolation_vec.push(num);
            }
        }

        *extrapolation_vec.last().unwrap()
    }

    pub fn reverse_extrapolate_score(&self) -> i64 {
        let mut vecs: Vec<Vec<i64>> = Vec::new();
        vecs.push(self.data.clone());

        'report: loop {
            // Copy all elements except the last
            let mut subtract_vec = vecs.last().unwrap().clone();
            subtract_vec.pop();

            // Insert 0 to shift the elements forward
            subtract_vec.insert(0, 0);

            // Subtract those elements from the original vector (x_n - x_n-1)
            let mut diff_vec: Vec<i64> = vecs.last().unwrap().iter().zip(subtract_vec.iter())
                .map(|(x, y)| {
                    x - y
                })
                .collect();

            // Remove the first element
            diff_vec.remove(0);

            // Store the new vector
            vecs.push(diff_vec);

            // If the vector is all zeroes
            if vecs.last().unwrap().iter().all(|n| *n == 0) {
                break 'report;
            }
        }

        let mut extrapolation_vec = Vec::<i64>::new();

        for (i, vec) in vecs.iter_mut().rev().enumerate() {
            if i == 0 {
                // Extrapolate last vector (zeroes)
                extrapolation_vec.push(0);
            } else if i == 1 {
                // Extrapolate second-to-last vector (constants)
                let num = vec.first().unwrap().clone();
                extrapolation_vec.push(num);
            } else {
                // Extrapolate for any higher vecs
                let num = vec.first().unwrap() - extrapolation_vec.last().unwrap();
                extrapolation_vec.push(num);
            }
        }

        *extrapolation_vec.last().unwrap()
    }

    pub fn alternative_extrapolate_score(&self) -> i64 {
        let mut data = self.data.clone();
        let mut degree: u32 = 0;

        // Calculate degree of generating polynomial
        'report: loop {
            // Increment degree
            degree += 1;

            // Copy all elements except the last
            let mut subtract_vec = data.clone();
            subtract_vec.pop();

            // Insert 0 to shift the elements forward
            subtract_vec.insert(0, 0);

            // Subtract those elements from the original vector (x_n - x_n-1)
            data = data.iter().zip(subtract_vec.iter())
                .map(|(x, y)| {
                    x - y
                })
                .collect();

            // Remove the first element
            data.remove(0);

            // If the vector is all zeroes
            if data.iter().all(|n| *n == 0) {
                degree -= 1;
                break 'report;
            }
        }

        // Create system of equations
        let mut matrix: Vec<Vec<i64>> = Vec::new();
        for x in 0..self.data.len() {
            let mut row = Vec::<i64>::new();
            for p in 0..=degree {
                row.push((x as i64).pow(p));
            }
            matrix.push(row);
        }

        // Solve system of equations
        // (not implemented)

        0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseHistoryErr;

impl FromStr for History {
    type Err = ParseHistoryErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<i64> = s
            .split(' ')
            .map(|s| {
                let num: i64 = s.parse().unwrap();
                num
            })
            .collect();

        Ok(History {
            data
        })
    }
}

#[aocd(2023, 9)]
pub fn solution1() {
    let histories: Vec<History> = input!()
        .split('\n')
        .map(|s| History::from_str(s).unwrap())
        .collect();

    // for history in histories {
    //     println!("{:?}", &history);
    // }

    // let test_history_1 = History::from_str("0 3 6 9 12 15").unwrap();
    // let test_history_2 = History::from_str("1 3 6 10 15 21").unwrap();
    // let test_history_3 = History::from_str("10 13 16 21 30 45").unwrap();
    // let test_history_4 = History::from_str("3 3 3 3 3 3").unwrap();
    // println!("{}", test_history_4.extrapolate_score());

    let score: i64 = histories.iter().map(|h| h.extrapolate_score()).sum();

    submit!(1, score);
}

#[aocd(2023, 9)]
pub fn solution2() {
    let histories: Vec<History> = input!()
        .split('\n')
        .map(|s| History::from_str(s).unwrap())
        .collect();

    // for history in histories {
    //     println!("{:?}", &history);
    // }

    // let test_history_1 = History::from_str("0 3 6 9 12 15").unwrap();
    // let test_history_2 = History::from_str("1 3 6 10 15 21").unwrap();
    // let test_history_3 = History::from_str("10 13 16 21 30 45").unwrap();
    // let test_history_4 = History::from_str("3 3 3 3 3 3").unwrap();
    // println!("{}", test_history_4.extrapolate_score());

    let score: i64 = histories.iter().map(|h| h.reverse_extrapolate_score()).sum();

    submit!(2, score);
}