use std::collections::HashMap;

const LINE_FEED: u8 = 10;
const START: Element = *b"AAA";
const TARGET: Element = *b"ZZZ";
const INPUT: &[u8] = include_bytes!("./input.txt");

type Element = [u8; 3];

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_byte(byte: &u8) -> Direction {
        match *byte {
            b'L' => Direction::Left,
            b'R' => Direction::Right,
            _ => panic!(),
        }
    }
}

pub fn part_one() {
    let (directions, node_map) = parse_input(INPUT);
    println!(
        "8.1 answer: {}",
        get_steps_to_zzz(directions.as_slice(), &node_map)
    );
}

// pub fn part_two() {
//     todo!()
// }

fn parse_input(input: &[u8]) -> (Vec<Direction>, HashMap<Element, (Element, Element)>) {
    let mut lines = input.split(|int| *int == LINE_FEED);
    let directions = parse_directions(lines.next().unwrap());
    let mut hash_map: HashMap<Element, (Element, Element)> = HashMap::new();

    lines.next();
    lines
        .filter(|line| !line.is_empty())
        .map(parse_node)
        .for_each(|(key, left, right)| {
            hash_map.insert(key, (left, right));
        });
    (directions, hash_map)
}

fn get_steps_to_zzz(
    directions: &[Direction],
    hash_map: &HashMap<Element, (Element, Element)>,
) -> u64 {
    let mut i = 0;
    let mut current_element = &START;
    let mut directions_iter = directions.iter();

    while current_element != &TARGET {
        if let Some(direction) = directions_iter.next() {
            let (left, right) = hash_map.get(current_element).unwrap();

            current_element = match direction {
                Direction::Left => left,
                Direction::Right => right,
            };
            i += 1;
        } else {
            directions_iter = directions.iter();
        }
    }
    i
}

fn parse_directions(line: &[u8]) -> Vec<Direction> {
    line.iter().map(Direction::from_byte).collect()
}

fn parse_node(line: &[u8]) -> (Element, Element, Element) {
    (
        [line[0], line[1], line[2]],
        [line[7], line[8], line[9]],
        [line[12], line[13], line[14]],
    )
}

#[cfg(test)]
mod tests {
    use crate::day08::{get_steps_to_zzz, parse_input};

    const EXAMPLE_ONE: &[u8] = b"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    const EXAMPLE_TWO: &[u8] = b"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn part_one_works() {
        let (directions1, nodemap1) = parse_input(EXAMPLE_ONE);
        let (directions2, nodemap2) = parse_input(EXAMPLE_TWO);
        assert_eq!(get_steps_to_zzz(directions1.as_slice(), &nodemap1), 2);
        assert_eq!(get_steps_to_zzz(directions2.as_slice(), &nodemap2), 6);
    }

    // #[test]
    // fn part_two_works() {}
}
