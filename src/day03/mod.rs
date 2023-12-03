const POINT: u8 = b'.';
const LINE_FEED: u8 = 10;
const CARRIAGE_RETURN: u8 = 13;

const INPUT: &[u8] = include_bytes!("./input.txt");

pub fn part_one() {
    let sum = get_sum(INPUT);
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
                .filter(|(_, c)| !c.is_ascii_digit() && c != &&POINT && c != &&CARRIAGE_RETURN)
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

            'sum: for y2 in y.saturating_sub(1)..=y.saturating_add(1) {
                for x2 in
                    start_index.saturating_sub(1)..=start_index.saturating_add(number_vec.len())
                {
                    if let Some(symbol_y) = symbols_indexes.get(y2) {
                        if symbol_y.contains(&x2) {
                            let number_string: String =
                                number_vec.iter().map(|val| *val as char).collect();
                            if !number_string.is_empty() {
                                dbg!(number_string.as_str());
                            }
                            sum += number_string.parse().unwrap_or(0);
                            break 'sum;
                        }
                    }
                }
            }
            number_vec.clear();
        }
    }
    sum
}

// pub fn part_two() {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::get_sum;

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
}
