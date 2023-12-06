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
        let (time, record) = (self.time, self.record);

        let mut left = 0;
        let mut right = time;
        let mut mid = time / 2;
        let mut last_record_beating_index = mid;
        while left <= right {
            let distance = get_distance(time, mid);

            if distance > record {
                right = mid - 1;
                last_record_beating_index = mid;
            } else {
                left = mid + 1;
            }
            mid = (left + right) / 2;
        }
        let left_bound = last_record_beating_index;

        let mut left = 0;
        let mut right = time;
        let mut mid = time / 2;
        let mut last_record_beating_index = mid;
        while left <= right {
            let distance = get_distance(time, mid);

            if distance > record {
                left = mid + 1;
                last_record_beating_index = mid;
            } else {
                right = mid - 1;
            }
            mid = (left + right) / 2;
        }
        let right_bound = last_record_beating_index;

        (left_bound, right_bound)

        // let mut left_bound = self.time;
        // let mut right_bound = 0;

        // [1..self.time].binary_search(||)

        // for i in 1..self.time {
        //     let distance = get_distance(self.time, i);
        //     if distance > self.record {
        //         left_bound = i;
        //         break;
        //     }
        // }
        // for i in (1..self.time).rev() {
        //     let distance = get_distance(self.time, i);
        //     if distance > self.record {
        //         right_bound = i;
        //         break;
        //     }
        // }
        // (left_bound, right_bound)
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

pub fn part_two() {
    let race = parse_long_race(INPUT);
    let margin_of_error = race.get_margin_of_error();

    println!("6.2 answer: {}", 1 + margin_of_error.1 - margin_of_error.0);
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

fn parse_long_number(line: &str) -> u64 {
    let mut number_string = String::new();

    line.split_ascii_whitespace()
        .enumerate()
        .filter(|(key, _)| key != &0)
        .for_each(|(_, num_str)| number_string.push_str(num_str));
    number_string.parse().unwrap()
}

fn parse_long_race(input: &str) -> Race {
    let mut lines = input.lines();
    let time = parse_long_number(lines.next().unwrap());
    let record = parse_long_number(lines.next().unwrap());

    Race { time, record }
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
    use super::{get_error_product, parse_long_race, parse_races, MarginOfError};

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

    #[test]
    fn part_two_works() {
        let race = parse_long_race(EXAMPLE);
        let margin_of_error = race.get_margin_of_error();

        assert_eq!(margin_of_error, (14, 71516));
        assert_eq!(1 + margin_of_error.1 - margin_of_error.0, 71503);
    }
}
