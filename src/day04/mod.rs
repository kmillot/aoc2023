const INPUT: &str = include_str!("./input.txt");

pub fn part_one() {
    let cards = parse_cards(INPUT);
    let won_numbers: Vec<Vec<u8>> = cards
        .iter()
        .map(|card| get_won_numbers(card.0.as_slice(), card.1.as_slice()))
        .collect();
    let total_points: u32 = won_numbers
        .iter()
        .map(|card| get_points(card.as_slice()))
        .sum();

    println!("3.1 answer: {}", total_points);
}

pub fn part_two() {
    let mut cards = parse_cards_with_id(INPUT);
    get_all_card_copies(&mut cards);
    let sum = cards.iter().map(|(_,count,_,_)| count).sum::<u32>();

    println!("3.2 answer: {}", sum);
}

fn parse_cards_iter(s: &str) -> impl Iterator<Item = (Vec<u8>, Vec<u8>)> + '_ {
    s.lines()
        .map(|line| line.split_once(": ").unwrap().1.split_once("| ").unwrap())
        .map(|card| {
            (
                card.0
                    .split_whitespace()
                    .map(|number| number.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>(),
                card.1
                    .split_whitespace()
                    .map(|number| number.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>(),
            )
        })
}

fn parse_cards(s: &str) -> Vec<(Vec<u8>, Vec<u8>)> {
    parse_cards_iter(s).collect()
}

fn parse_cards_with_id(s: &str) -> Vec<(usize, u32, Vec<u8>, Vec<u8>)> {
    parse_cards_iter(s)
        .enumerate()
        .map(|(key, cards)| (key + 1, 1, cards.0, cards.1))
        .collect()
}

fn get_won_numbers(winning_numbers: &[u8], numbers: &[u8]) -> Vec<u8> {
    numbers
        .iter()
        .filter(|number| winning_numbers.contains(number))
        .copied()
        .collect::<Vec<u8>>()
}

fn get_all_card_copies(cards: &mut Vec<(usize, u32, Vec<u8>, Vec<u8>)>) {
    let mut i = 0;
    let mut len = cards.len();

    while i < len {
        let start_id = cards[i].0 + 1;
        let win_count = get_won_numbers(cards[i].2.as_slice(), cards[i].3.as_slice()).len();
        let end_id = start_id + win_count;

        if win_count != 0 {
            for j in start_id..end_id {
                for _ in 0..cards[i].1 {
                    cards[j - 1].1 += 1;
                }
            }
            len = cards.len();
        }
        i += 1;
    }
}

fn get_points(won_numbers: &[u8]) -> u32 {
    if won_numbers.is_empty() {
        0
    } else {
        u32::pow(2, (won_numbers.len() as u32) - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        get_all_card_copies, get_points, get_won_numbers, parse_cards, parse_cards_with_id,
    };

    const EXAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part_one_works() {
        let cards = parse_cards(EXAMPLE);
        let won_numbers: Vec<Vec<u8>> = cards
            .iter()
            .map(|card| get_won_numbers(card.0.as_slice(), card.1.as_slice()))
            .collect();
        let game_points: Vec<u32> = won_numbers
            .iter()
            .map(|card| get_points(card.as_slice()))
            .collect();

        assert!(won_numbers[0]
            .iter()
            .all(|num| [48, 83, 17, 86].contains(num)));
        assert!(won_numbers[1].iter().all(|num| [32, 61].contains(num)));
        assert!(won_numbers[2].iter().all(|num| [1, 21].contains(num)));
        assert!(won_numbers[3].iter().all(|num| [84].contains(num)));
        assert!(won_numbers[4].is_empty());
        assert!(won_numbers[5].is_empty());
        assert_eq!(game_points.as_slice(), &[8, 2, 2, 1, 0, 0]);
        assert_eq!(game_points.iter().sum::<u32>(), 13);
    }

    #[test]
    fn part_two_works() {
        let mut cards = parse_cards_with_id(EXAMPLE);
        get_all_card_copies(&mut cards);
        let sum = cards.iter().map(|(_,count,_,_)| count).sum::<u32>();
        
        assert_eq!(cards[0].1, 1);
        assert_eq!(cards[1].1, 2);
        assert_eq!(cards[2].1, 4);
        assert_eq!(cards[3].1, 8);
        assert_eq!(cards[4].1, 14);
        assert_eq!(cards[5].1, 1);
        assert_eq!(sum, 30);
    }
}
