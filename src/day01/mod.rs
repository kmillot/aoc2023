const INPUT: &str = include_str!("./input.txt");

type FoundNumber<'a> = (usize, &'a str);

pub fn part_one() {
    let sum: u64 = INPUT.lines().map(get_calibration_number_legacy).sum();
    println!("{}", sum);
}

pub fn part_two() {
    let sum: u64 = INPUT.lines().map(get_calibration_number).sum();
    println!("{}", sum);
}

fn get_calibration_number_legacy(s: &str) -> u64 {
    let num_vec: Vec<u32> = s
        .chars()
        .filter(|char| char.is_numeric())
        .map(|char| char.to_digit(10).unwrap())
        .collect();
    let first_number = num_vec.first().unwrap();
    let last_number = num_vec.last().unwrap();

    format!("{first_number}{last_number}").parse().unwrap()
}

fn get_calibration_number(s: &str) -> u64 {
    let word_nums = [
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
    let mut first_and_last_number: Option<(FoundNumber, FoundNumber)> = None;

    for word_num in word_nums {
        let (word, num) = word_num;
        let found_word = s.find(word);
        let found_num = s.find(num);
        let found_word_r = s.rfind(word);
        let found_num_r = s.rfind(num);

        if let Some(found_word) = found_word {
            if let Some((mut first, last)) = first_and_last_number {
                if found_word < first.0 {
                    first.0 = found_word;
                    first.1 = num;
                    first_and_last_number = Some((first, last))
                }
            } else {
                first_and_last_number = Some(((found_word, num), (found_word, num)));
            }
        }

        if let Some(found_word_r) = found_word_r {
            if let Some((first, mut last)) = first_and_last_number {
                if found_word_r > last.0 {
                    last.0 = found_word_r;
                    last.1 = num;
                    first_and_last_number = Some((first, last))
                }
            }
        }

        if let Some(found_num) = found_num {
            if let Some((mut first, last)) = first_and_last_number {
                if found_num < first.0 {
                    first.0 = found_num;
                    first.1 = num;
                    first_and_last_number = Some((first, last))
                }
            } else {
                first_and_last_number = Some(((found_num, num), (found_num, num)));
            }
        }

        if let Some(found_num_r) = found_num_r {
            if let Some((first, mut last)) = first_and_last_number {
                if found_num_r > last.0 {
                    last.0 = found_num_r;
                    last.1 = num;
                    first_and_last_number = Some((first, last))
                }
            }
        }
    }

    let (first, last) = first_and_last_number.unwrap();
    let (first, last) = (first.1.chars(), last.1.chars());

    first
        .chain(last)
        .collect::<String>()
        .parse::<u64>()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use crate::day01::{get_calibration_number, get_calibration_number_legacy};

    #[test]
    fn produces_correct_calibration_numbers_legacy() {
        let example: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let (num_array, sum): (Vec<u64>, u64) =
            get_test_values(example, &get_calibration_number_legacy);

        assert_eq!(num_array.len(), 4);
        assert_eq!(num_array.first(), Some(&12));
        assert_eq!(num_array.get(1), Some(&38));
        assert_eq!(num_array.get(2), Some(&15));
        assert_eq!(num_array.get(3), Some(&77));
        assert_eq!(sum, 142);
    }

    #[test]
    fn produces_correct_calibration_numbers() {
        let example: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let (num_array, sum): (Vec<u64>, u64) = get_test_values(example, &get_calibration_number);

        assert_eq!(num_array.len(), 7);
        assert_eq!(num_array.first(), Some(&29));
        assert_eq!(num_array.get(1), Some(&83));
        assert_eq!(num_array.get(2), Some(&13));
        assert_eq!(num_array.get(3), Some(&24));
        assert_eq!(num_array.get(4), Some(&42));
        assert_eq!(num_array.get(5), Some(&14));
        assert_eq!(num_array.get(6), Some(&76));
        assert_eq!(sum, 281);
    }

    fn get_test_values(s: &str, func: &dyn Fn(&str) -> u64) -> (Vec<u64>, u64) {
        let num_array: Vec<u64> = s.lines().map(func).collect();
        let sum: u64 = num_array.iter().sum();
        (num_array, sum)
    }
}
