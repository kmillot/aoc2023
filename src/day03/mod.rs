const POINT: u8 = b'.';
const LINE_FEED: u8 = 10;
const CARRIAGE_RETURN: u8 = 13;
const ASTERISK: u8 = 42;

const INPUT: &[u8] = include_bytes!("./input.txt");

pub fn part_one() {
    let sum = get_sum(INPUT);
    println!("{}", sum);
}

pub fn part_two() {
    let sum: u64 = get_gear_values(INPUT).iter().sum();
    println!("{}", sum);
}

pub fn get_sum(input: &[u8]) -> u64 {
    let lines: Vec<&[u8]> = input.split(|int| *int == LINE_FEED).collect();
    let mut sum: u64 = 0;
    let symbols_indexes = lines
        .iter()
        .map(|line| {
            line.iter()
                .enumerate()
                .filter(|(_, c)| !c.is_ascii_digit() && **c != POINT && **c != CARRIAGE_RETURN)
                .map(|(key, _)| key)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    for (y, _) in lines.iter().enumerate() {
        let (mut start_index, mut number_vec): (usize, Vec<u8>) = (0, vec![]);

        'x: for x in 0..lines[y].len() {
            let character = lines[y][x];
            if character.is_ascii_digit() {
                if number_vec.is_empty() {
                    start_index = x;
                }
                number_vec.push(character);
            }

            if lines[y].get(x + 1).is_some_and(u8::is_ascii_digit) {
                continue 'x;
            }

            if !number_vec.is_empty() {
                'sum: for y2 in y.saturating_sub(1)..=y.saturating_add(1) {
                    for x2 in
                        start_index.saturating_sub(1)..=start_index.saturating_add(number_vec.len())
                    {
                        if let Some(symbol_y) = symbols_indexes.get(y2) {
                            if symbol_y.contains(&x2) {
                                let number_string: String =
                                    number_vec.iter().map(|val| *val as char).collect();
                                sum += number_string.parse().unwrap_or(0);
                                break 'sum;
                            }
                        }
                    }
                }
                number_vec.clear();
            }
        }
    }
    sum
}

pub fn get_gear_values(input: &[u8]) -> Vec<u64> {
    let lines: Vec<&[u8]> = input.split(|int| *int == LINE_FEED).collect();
    let gear_indexes = lines
        .iter()
        .map(|line| {
            line.iter()
                .enumerate()
                .filter(|(_, c)| **c == ASTERISK)
                .map(|(key, _)| key)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let mut gear_vec: Vec<(usize, usize, u64, bool)> = vec![];

    for (y, _) in lines.iter().enumerate() {
        let (mut start_index, mut number_vec): (usize, Vec<u8>) = (0, vec![]);

        'x: for x in 0..lines[y].len() {
            let character = lines[y][x];
            if character.is_ascii_digit() {
                if number_vec.is_empty() {
                    start_index = x;
                }
                number_vec.push(character);
            }
            if lines[y].get(x + 1).is_some_and(u8::is_ascii_digit) {
                continue 'x;
            }
            if !number_vec.is_empty() {
                'gear: for y2 in y.saturating_sub(1)..=y.saturating_add(1) {
                    for x2 in
                        start_index.saturating_sub(1)..=start_index.saturating_add(number_vec.len())
                    {
                        if let Some(gear_y) = gear_indexes.get(y2) {
                            if gear_y.contains(&x2) {
                                let number_string: String =
                                    number_vec.iter().map(|val| *val as char).collect();
                                let number = number_string.parse::<u64>().unwrap();

                                if let Some(gear) = gear_vec
                                    .iter_mut()
                                    .find(|(y, x, _, _)| *y == y2 && *x == x2)
                                {
                                    gear.2 *= number;
                                    gear.3 = true;
                                } else {
                                    gear_vec.push((y2, x2, number, false));
                                }
                                break 'gear;
                            }
                        }
                    }
                }
            }
            number_vec.clear();
        }
    }
    gear_vec
        .iter()
        .filter(|(_, _, _, is_gear)| *is_gear)
        .map(|(_, _, value, _)| *value)
        .collect()
}

// pub fn part_two() {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::{get_gear_values, get_sum};

    const EXAMPLE: &[u8] = b"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn it_works() {
        let sum = get_sum(EXAMPLE);
        assert_eq!(sum, 4361);
    }

    #[test]
    fn part_two_works() {
        let gear_values = get_gear_values(EXAMPLE);
        assert_eq!(gear_values[0], 16345);
        assert_eq!(gear_values[1], 451490);
        assert_eq!(gear_values.iter().sum::<u64>(), 467835);
    }
}
