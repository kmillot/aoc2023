const INPUT: &str = include_str!("./input.txt");

use std::{collections::HashMap, str::FromStr};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum Card {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl Card {
    fn from_char(c: char) -> Result<Self, ()> {
        match c {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Jack),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 4,
    ThreeOfAKind = 8,
    FullHouse = 16,
    FourOfAKind = 32,
    FiveOfAKind = 64,
}

impl HandType {
    fn from_cards(cards: &[Card; 5]) -> Self {
        let mut card_set: HashMap<Card, u8> = HashMap::with_capacity(5);

        for card in cards {
            if let Some(i) = card_set.get_mut(card) {
                *i += 1;
            } else {
                card_set.insert(*card, 1);
            }
        }
        let numbers: Vec<u8> = card_set.into_values().collect();
        match numbers.as_slice() {
            [5] => HandType::FiveOfAKind,
            [4, 1] | [1, 4] => HandType::FourOfAKind,
            [3, 2] | [2, 3] => HandType::FullHouse,
            [2, 2, _] | [2, _, 2] | [_, 2, 2] => HandType::TwoPair,
            slice => {
                if slice.contains(&3) {
                    HandType::ThreeOfAKind
                } else if slice.contains(&2) {
                    HandType::OnePair
                } else {
                    HandType::HighCard
                }
            }
        }
    }
}

#[derive(Eq)]
struct Hand {
    bid: u64,
    cards: [Card; 5],
    hand_type: HandType,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Less => return Some(std::cmp::Ordering::Less),
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => return Some(std::cmp::Ordering::Greater),
        };
        for i in 0..5 {
            // SAFETY: length of Hand::cards is always 5
            match self.cards[i].cmp(&other.cards[i]) {
                std::cmp::Ordering::Less => return Some(std::cmp::Ordering::Less),
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Greater => return Some(std::cmp::Ordering::Greater),
            }
        }
        Some(std::cmp::Ordering::Equal)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = [Card::Two; 5];

        let (cards_str, bid_str) = s.split_once(' ').ok_or(())?;
        let mut cards_chars = cards_str.chars();
        for card in cards.iter_mut() {
            let char = cards_chars.next().ok_or(())?;
            *card = Card::from_char(char)?;
        }
        let hand_type = HandType::from_cards(&cards);
        let bid = bid_str.parse().map_err(|_| ())?;

        Ok(Self {
            bid,
            cards,
            hand_type,
        })
    }
}

pub fn part_one() {
    let mut hands: Vec<Hand> = INPUT
        .lines()
        .map(|line| Hand::from_str(line).unwrap())
        .collect();
    hands.sort();
    let sum: u64 = hands
        .iter()
        .enumerate()
        .map(|(k, v)| (k + 1) as u64 * v.bid)
        .sum();

    println!("7.1 answer: {}", sum);
}

// pub fn part_two() {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{Card, Hand, HandType};

    const EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part_one_works() {
        let mut hands: Vec<Hand> = EXAMPLE
            .lines()
            .map(|line| Hand::from_str(line).unwrap())
            .collect();
        hands.sort();
        let sum: u64 = hands
            .iter()
            .enumerate()
            .map(|(k, v)| (k + 1) as u64 * v.bid)
            .sum();

        assert_eq!(hands[0].hand_type, HandType::OnePair);
        assert_eq!(
            hands[0].cards,
            [Card::Three, Card::Two, Card::Ten, Card::Three, Card::King]
        );
        assert_eq!(hands[1].hand_type, HandType::TwoPair);
        assert_eq!(
            hands[1].cards,
            [Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten]
        );
        assert_eq!(hands[2].hand_type, HandType::TwoPair);
        assert_eq!(
            hands[2].cards,
            [Card::King, Card::King, Card::Six, Card::Seven, Card::Seven]
        );
        assert_eq!(hands[3].hand_type, HandType::ThreeOfAKind);
        assert_eq!(
            hands[3].cards,
            [Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five]
        );
        assert_eq!(hands[4].hand_type, HandType::ThreeOfAKind);
        assert_eq!(
            hands[4].cards,
            [Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace]
        );
        assert_eq!(sum, 6440);
    }
}
