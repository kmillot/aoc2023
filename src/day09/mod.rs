use std::collections::VecDeque;

const INPUT: &str = include_str!("./input.txt");

pub fn part_one() {
    let number_vecs = parse_input(INPUT);
    let mut differences_deques: Vec<Vec<VecDeque<i64>>> = number_vecs
        .iter()
        .map(|nums| get_with_differences(nums))
        .collect();

    differences_deques
        .iter_mut()
        .for_each(|diff_deque| push_values_back(diff_deque));
    let history_values: Vec<i64> = differences_deques
        .iter()
        .map(|diff_deque| get_right_history_value(diff_deque))
        .collect();
    println!("9.1 answer: {}", history_values.iter().sum::<i64>());
}

pub fn part_two() {
    let number_vecs = parse_input(INPUT);
    let mut differences_deques: Vec<Vec<VecDeque<i64>>> = number_vecs
        .iter()
        .map(|nums| get_with_differences(nums))
        .collect();

    differences_deques
        .iter_mut()
        .for_each(|diff_deque| push_values_front(diff_deque));
    let history_values: Vec<i64> = differences_deques
        .iter()
        .map(|diff_deque| get_left_history_value(diff_deque))
        .collect();
    println!("9.2 answer: {}", history_values.iter().sum::<i64>());
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.trim_end()
                .split_ascii_whitespace()
                .map(|num_str| num_str.parse().unwrap())
                .collect()
        })
        .collect()
}

fn get_with_differences(nums: &[i64]) -> Vec<VecDeque<i64>> {
    let mut curr = nums;
    let mut first_deque: VecDeque<i64> = VecDeque::with_capacity(curr.len() * 2 - 1);
    let mut diff_deques = Vec::with_capacity(curr.len());
    let mut first_vec = Vec::with_capacity(curr.len() * 2 - 1);

    first_deque.extend(nums.iter());
    first_vec.push(..nums);
    diff_deques.push(first_deque);
    while curr.len() > 1 && !curr.iter().all(|n| *n == 0) {
        let mut differences = get_differences(curr);
        differences.make_contiguous();
        diff_deques.push(differences);
        (curr, _) = diff_deques.last().unwrap().as_slices();
    }
    diff_deques
}

fn get_differences(nums: &[i64]) -> VecDeque<i64> {
    let len = nums.len();
    let mut diff_deque = VecDeque::with_capacity(len * 2 - 1);

    for i in 1..len {
        diff_deque.push_back(unsafe { nums.get_unchecked(i) - nums.get_unchecked(i - 1) });
    }
    diff_deque
}

fn push_values_back(num_deques: &mut [VecDeque<i64>]) {
    let mut below = 0;

    for num_deque in num_deques.iter_mut().rev() {
        let len = num_deque.len();
        let last = num_deque[len - 1];
        let next = last + below;

        num_deque.push_back(next);
        below = next;
    }
}

fn push_values_front(num_deques: &mut [VecDeque<i64>]) {
    let mut below = 0;

    for num_deque in num_deques.iter_mut().rev() {
        let first = num_deque[0];
        let prev = first - below;

        num_deque.push_front(prev);
        below = prev;
    }
}

fn get_right_history_value(num_deques: &[VecDeque<i64>]) -> i64 {
    num_deques
        .iter()
        .map(|num_deque| *num_deque.back().unwrap())
        .next()
        .unwrap_or(0)
}

fn get_left_history_value(num_deques: &[VecDeque<i64>]) -> i64 {
    num_deques
        .iter()
        .map(|num_deque| *num_deque.front().unwrap())
        .next()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::day09::get_left_history_value;

    use super::{
        get_right_history_value, get_with_differences, parse_input, push_values_back,
        push_values_front,
    };
    const EXAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part_one_works() {
        let number_vecs = parse_input(EXAMPLE);
        let mut differences_deques: Vec<Vec<VecDeque<i64>>> = number_vecs
            .iter()
            .map(|nums| get_with_differences(nums))
            .collect();
        let final_match1 = parse_input(
            "0   3   6   9  12  15  18
  3   3   3   3   3   3
    0   0   0   0   0",
        );
        let final_match2 = parse_input(
            "1   3   6  10  15  21  28
  2   3   4   5   6   7
    1   1   1   1   1
      0   0   0   0",
        );
        let final_match3 = parse_input(
            "10  13  16  21  30  45  68
   3   3   5   9  15  23
     0   2   4   6   8
       2   2   2   2
         0   0   0",
        );

        differences_deques
            .iter_mut()
            .for_each(|diff_deque| push_values_back(diff_deque));
        let history_values: Vec<i64> = differences_deques
            .iter()
            .map(|diff_deque| get_right_history_value(diff_deque))
            .collect();
        assert_eq!(differences_deques[0], final_match1);
        assert_eq!(differences_deques[1], final_match2);
        assert_eq!(differences_deques[2], final_match3);
        assert_eq!(history_values.as_slice(), &[18, 28, 68]);
        assert_eq!(history_values.iter().sum::<i64>(), 114);
    }

    #[test]
    fn differences_are_correct() {
        let number_vecs = parse_input(EXAMPLE);
        let differences_deques: Vec<Vec<VecDeque<i64>>> = number_vecs
            .iter()
            .map(|nums| get_with_differences(nums))
            .collect();
        let differences_match1 = parse_input(
            "0   3   6   9  12  15
  3   3   3   3   3
    0   0   0   0",
        );
        let differences_match2 = parse_input(
            "1   3   6  10  15  21
  2   3   4   5   6
    1   1   1   1
      0   0   0",
        );
        let differences_match3 = parse_input(
            "10  13  16  21  30  45
   3   3   5   9  15
     0   2   4   6
       2   2   2
         0   0",
        );

        assert_eq!(differences_deques[0], differences_match1);
        assert_eq!(differences_deques[1], differences_match2);
        assert_eq!(differences_deques[2], differences_match3);
    }

    #[test]
    fn part_two_works() {
        let number_vecs = parse_input(EXAMPLE);
        let mut differences_deques: Vec<Vec<VecDeque<i64>>> = number_vecs
            .iter()
            .map(|nums| get_with_differences(nums))
            .collect();
        let differences_match3 = parse_input(
            "5  10  13  16  21  30  45
  5   3   3   5   9  15
   -2   0   2   4   6
      2   2   2   2
        0   0   0",
        );

        differences_deques
            .iter_mut()
            .for_each(|diff_deque| push_values_front(diff_deque));
        let history_values: Vec<i64> = differences_deques
            .iter()
            .map(|diff_vec| get_left_history_value(diff_vec))
            .collect();
        assert_eq!(differences_deques[2], differences_match3);
        assert_eq!(history_values.as_slice(), &[-3, 0, 5]);
        assert_eq!(history_values.iter().sum::<i64>(), 2);
    }

    #[test]
    fn differences_are_correct_when_negatives() {
        let number_vecs = parse_input("-12 -9 -6 -3 0 3");
        let differences_match = parse_input(
            "-12 -9 -6 -3 0 3
                3 3 3 3 3
                0 0 0 0",
        );
        let differences_deque: Vec<VecDeque<i64>> = get_with_differences(number_vecs[0].as_slice());

        assert_eq!(differences_deque, differences_match);
    }
}
