const INPUT: &str = include_str!("./input.txt");

type MarginOfError = (u64, u64);
type Millisecond = u64;
type Distance = u64;

enum ButtonState {
    Held,
    Released,
}

#[derive(Debug)]
struct Race {
    time: Millisecond,
    record: Distance,
}

impl Race {
    fn get_margin_of_error(&self) -> MarginOfError {
        // BINARY SEARCH METHOD (coding this later)
        // let mut left_bound: Millisecond = 0;
        // let mut right_bound: Millisecond = u64::MAX;
        // let mut left_curr_try: Millisecond = self.time >> 1;
        // let mut left_curr_try: Millisecond = left_curr_try;
        // let mut left_j = Some(left_curr_try);
        // let mut right_j = Some(left_curr_try);

        // while let Some(mut j) = left_j {
        //     let distance = get_distance(self.time, left_curr_try);

        //     if distance > self.record {
        //         if left_bound < left_curr_try {
        //             left_bound = left_curr_try;
        //         }
        //     } else {
        //         j = j + (j>>1)
        //     }
        //     left_i = j.checked_shr(1);
        // }
        let mut left_bound = self.time;
        let mut right_bound = 0;

        for i in 1..self.time {
            let distance = get_distance(self.time, i);
            if distance > self.record {
                left_bound = i;
                break;
            }
        }
        for i in (1..self.time).rev() {
            let distance = get_distance(self.time, i);
            if distance > self.record {
                right_bound = i;
                break;
            }
        }
        (left_bound, right_bound)
    }
}

pub fn part_one() {
    let races = parse_races(INPUT);
    let margins_of_error: Vec<MarginOfError> = races
        .iter()
        .map(|race| race.get_margin_of_error())
        .collect();
    let product = get_error_product(margins_of_error.as_slice());

    println!("6.1 answer: {}", product);
}

fn parse_races(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times = parse_number(lines.next().unwrap());
    let records = parse_number(lines.next().unwrap());
    let len = times.len();
    let mut race_vec = Vec::with_capacity(len);

    for i in 0..len {
        race_vec.push(Race {
            time: times[i],
            record: records[i],
        })
    }
    race_vec
}

fn parse_number(line: &str) -> Vec<u64> {
    line.split_ascii_whitespace()
        .enumerate()
        .filter(|(key, _)| key != &0)
        .map(|(_, num_str)| num_str.parse().unwrap())
        .collect()
}

fn get_distance(race_time: Millisecond, hold_time: Millisecond) -> Distance {
    let mut accel = 0u64;
    let mut distance = 0u64;
    let mut button_state;

    for i in 1..=race_time {
        if i > hold_time {
            button_state = ButtonState::Released;
        } else {
            button_state = ButtonState::Held;
        }

        match button_state {
            ButtonState::Held => {
                accel += 1;
            }
            ButtonState::Released => {
                distance += accel;
            }
        }
    }
    distance
}

fn get_error_product(margins: &[MarginOfError]) -> u64 {
    margins
        .iter()
        .map(|(left, right)| 1 + right - left)
        .product()
}

#[cfg(test)]
mod tests {
    use super::{get_error_product, parse_races, MarginOfError};

    const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part_one_works() {
        let races = parse_races(EXAMPLE);
        let margins_of_error: Vec<MarginOfError> = races
            .iter()
            .map(|race| race.get_margin_of_error())
            .collect();
        let product = get_error_product(margins_of_error.as_slice());

        assert_eq!(margins_of_error[0], (2, 5));
        assert_eq!(margins_of_error[1], (4, 11));
        assert_eq!(margins_of_error[2], (11, 19));
        assert_eq!(product, 288);
    }
}
