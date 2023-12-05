const INPUT: &str = include_str!("./input.txt");

#[derive(Debug)]
struct ConversionNumbers {
    dest_num: u64,
    src_num: u64,
    add: u64,
}

fn convert(number: u64, conversion_map: &[ConversionNumbers]) -> u64 {
    for numbers in conversion_map {
        if (numbers.src_num..=numbers.src_num + numbers.add).contains(&number) {
            return number + numbers.dest_num - numbers.src_num;
        }
    }
    number
}

pub fn part_one() {
    let conversions = from_input(INPUT);
    let minimum = find_lowest(conversions.as_slice());
    println!("5.1 answer: {}", minimum);
}

fn parse_seeds(input: &str) -> Vec<u64> {
    input
        .lines()
        .next()
        .unwrap()
        .split_once("seeds: ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|num_str| num_str.parse().unwrap())
        .collect()
}

fn from_input(input: &str) -> Vec<[u64; 8]> {
    let seeds = parse_seeds(input);
    let conversion_maps = get_conversion_maps(input);
    get_conversions(conversion_maps.as_slice(), seeds.as_slice())
}

fn get_conversion_maps(input: &str) -> Vec<Vec<ConversionNumbers>> {
    let mut lines = input.lines();
    let mut conversion_maps: Vec<Vec<ConversionNumbers>> = Vec::with_capacity(7);
    let mut current_conversion_numbers: Option<Vec<ConversionNumbers>> = None;

    lines.nth(1);
    for line in lines {
        if line.contains("map:") {
            current_conversion_numbers = Some(vec![]);
            continue;
        }

        if line.is_empty() {
            conversion_maps.push(current_conversion_numbers.unwrap());
            current_conversion_numbers = None;
            continue;
        }

        if let Some(ref mut cm) = current_conversion_numbers {
            let literals: Vec<u64> = line
                .trim_end()
                .splitn(3, ' ')
                .map(|num_str| num_str.parse::<u64>().unwrap())
                .collect();
            cm.push(ConversionNumbers {
                dest_num: literals[0],
                src_num: literals[1],
                add: literals[2],
            })
        }
    }
    conversion_maps.push(current_conversion_numbers.take().unwrap());

    conversion_maps
}

fn get_conversions(conversion_maps: &[Vec<ConversionNumbers>], seeds: &[u64]) -> Vec<[u64; 8]> {
    let mut conversions: Vec<[u64; 8]> = Vec::with_capacity(seeds.len());

    for seed in seeds {
        let mut conversion_set = [*seed, 0, 0, 0, 0, 0, 0, 0];
        for i in 0..conversion_maps.len() {
            conversion_set[i + 1] = convert(conversion_set[i], &conversion_maps[i]);
        }
        conversions.push(conversion_set);
    }
    conversions
}

fn find_lowest(conversions: &[[u64; 8]]) -> u64 {
    conversions.iter().map(|set| set[7]).min().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    // The almanac starts by listing which seeds need to be planted: seeds 79, 14, 55, and 13.
    // The rest of the almanac contains a list of maps which describe how to convert numbers
    // from a source category into numbers in a destination category. That is, the section that
    // starts with seed-to-soil map: describes how to convert a seed number (the source) to a soil
    // number (the destination). This lets the gardener and his team know which soil to use with
    // which seeds, which water to use with which fertilizer, and so on.

    use crate::day05::{find_lowest, from_input};

    const EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part_one_works() {
        let conversions = from_input(EXAMPLE);
        let minimum = find_lowest(conversions.as_slice());

        assert_eq!(conversions[0], [79, 81, 81, 81, 74, 78, 78, 82]);
        assert_eq!(conversions[1], [14, 14, 53, 49, 42, 42, 43, 43]);
        assert_eq!(conversions[2], [55, 57, 57, 53, 46, 82, 82, 86]);
        assert_eq!(conversions[3], [13, 13, 52, 41, 34, 34, 35, 35]);
        assert_eq!(minimum, 35);
    }
}
