const INPUT: &[u8] = include_bytes!("./input.txt");

type GalaxyNumberAndXY = (usize, (u128, u128));
type GalaxyPair = (GalaxyNumberAndXY, GalaxyNumberAndXY);

use crate::utils::parse_from_bytes;

pub fn part_one() {
    let galaxy_matrix = parse_with_expansion(INPUT, 2);
    let galaxy_pairs = get_galaxy_pairs(galaxy_matrix.as_slice());
    let steps_betwixt: Vec<u128> = galaxy_pairs.iter().map(get_steps).collect();

    println!("11.1 answer: {}", steps_betwixt.iter().sum::<u128>());
}

pub fn part_two() {
    let galaxy_matrix = parse_with_expansion(INPUT, 1000000);
    let galaxy_pairs = get_galaxy_pairs(galaxy_matrix.as_slice());
    let steps_betwixt: Vec<u128> = galaxy_pairs.iter().map(get_steps).collect();

    println!("11.1 answer: {}", steps_betwixt.iter().sum::<u128>());
}

fn parse_input(input: &[u8]) -> Vec<Vec<bool>> {
    parse_from_bytes(input)
        .map(|line| line.map(|c| c == b'#').collect())
        .collect()
}
fn parse_with_expansion(input: &[u8], expansion_size: u128) -> Vec<(u128, Vec<(u128, bool)>)> {
    let parsed = parse_input(input);
    let count_y = parsed.len();
    let count_x = parsed[0].len();

    let are_empty_rows: Vec<bool> = parsed
        .iter()
        .map(|are_galaxies| !are_galaxies.contains(&true))
        .collect();
    let mut are_empty_columns: Vec<bool> = Vec::with_capacity(count_x);
    for x in 0..count_x {
        let mut is_empty_column = true;
        'y: for y in 0..count_y {
            if unsafe { *parsed.get_unchecked(y).get_unchecked(x) } {
                is_empty_column = false;
                break 'y;
            }
        }
        are_empty_columns.push(is_empty_column);
    }
    let mut are_galaxies_vecs = Vec::with_capacity(count_y);
    let mut true_x: u128 = 0;
    let mut true_y: u128 = 0;

    for (y, are_galaxies_row) in parsed.iter().enumerate() {
        let mut are_galaxies = Vec::with_capacity(count_x);

        for (x, is_galaxy) in are_galaxies_row.iter().enumerate() {
            if are_empty_columns[x] {
                true_x += expansion_size - 1;
            }
            are_galaxies.push((true_x, *is_galaxy));

            true_x += 1;
        }
        if are_empty_rows[y] {
            true_y += expansion_size - 1;
        }
        are_galaxies_vecs.push((true_y, are_galaxies));

        true_x = 0;
        true_y += 1;
    }

    are_galaxies_vecs
}

fn _legacy_parse(input: &[u8]) -> Vec<Vec<bool>> {
    let parsed = parse_input(input);
    let count_y = parsed.len();
    let count_x = parsed[0].len();

    let are_empty_rows: Vec<bool> = parsed
        .iter()
        .map(|are_galaxies| !are_galaxies.contains(&true))
        .collect();
    let mut are_empty_columns: Vec<bool> = Vec::with_capacity(count_x);
    for x in 0..count_x {
        let mut is_empty_column = true;
        'y: for y in 0..count_y {
            if unsafe { *parsed.get_unchecked(y).get_unchecked(x) } {
                is_empty_column = false;
                break 'y;
            }
        }
        are_empty_columns.push(is_empty_column);
    }
    let y_alloc: usize = are_empty_rows
        .iter()
        .map(|is_empty| if *is_empty { 2 } else { 1 })
        .sum();
    let x_alloc: usize = are_empty_columns
        .iter()
        .map(|is_empty| if *is_empty { 2 } else { 1 })
        .sum();
    let mut are_galaxies_vecs = Vec::with_capacity(y_alloc);

    for (y, are_galaxies_row) in parsed.iter().enumerate() {
        let mut are_galaxies = Vec::with_capacity(x_alloc);

        for (x, is_galaxy) in are_galaxies_row.iter().enumerate() {
            if are_empty_columns[x] {
                are_galaxies.push(*is_galaxy);
            }
            are_galaxies.push(*is_galaxy);
        }
        if are_empty_rows[y] {
            are_galaxies_vecs.push(are_galaxies.clone());
        }
        are_galaxies_vecs.push(are_galaxies);
    }

    are_galaxies_vecs
}

fn get_galaxy_pairs(are_galaxies: &[(u128, Vec<(u128, bool)>)]) -> Vec<GalaxyPair> {
    let galaxies_yx: Vec<GalaxyNumberAndXY> = are_galaxies
        .iter()
        .flat_map(|(y, row)| {
            row.iter()
                .filter(|(_, is)| *is)
                .map(|(x, _)| (*y, *x))
                .collect::<Vec<_>>()
        })
        .enumerate()
        .map(|(k, v)| (k + 1, v))
        .collect();
    let len = galaxies_yx.len();
    let mut pairs = Vec::with_capacity(len / 2);

    let mut i = 0;
    while i < len - 1 {
        let mut j = i + 1;
        while j < len {
            pairs.push((galaxies_yx[i], galaxies_yx[j]));
            j += 1;
        }
        i += 1;
    }
    pairs
}

fn get_steps(galaxy_pair: &GalaxyPair) -> u128 {
    let ((_, (y1, x1)), (_, (y2, x2))) = galaxy_pair;
    y2.abs_diff(*y1) + x2.abs_diff(*x1)
}

#[cfg(test)]
mod tests {
    use crate::day11::{
        _legacy_parse, get_galaxy_pairs, get_steps, parse_input, parse_with_expansion,
    };

    const EXAMPLE: &[u8] = b"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    const EXAMPLE_EXPANDED: &[u8] = b"....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";

    #[test]
    fn part_one_works() {
        let galaxy_matrix = parse_with_expansion(EXAMPLE, 2);
        let already_expanded = parse_input(EXAMPLE_EXPANDED);
        let runtime_expanded: Vec<Vec<bool>> = _legacy_parse(EXAMPLE);
        let galaxy_pairs = get_galaxy_pairs(galaxy_matrix.as_slice());
        let steps_betwixt: Vec<u128> = galaxy_pairs.iter().map(get_steps).collect();

        assert_eq!(already_expanded, runtime_expanded);
        assert_eq!(steps_betwixt.iter().sum::<u128>(), 374);
    }

    #[test]
    fn part_two_works() {
        let galaxy_matrix1 = parse_with_expansion(EXAMPLE, 10);
        let galaxy_matrix2 = parse_with_expansion(EXAMPLE, 100);
        let galaxy_pairs1 = get_galaxy_pairs(galaxy_matrix1.as_slice());
        let galaxy_pairs2 = get_galaxy_pairs(galaxy_matrix2.as_slice());
        let steps_betwixt1: Vec<u128> = galaxy_pairs1.iter().map(get_steps).collect();
        let steps_betwixt2: Vec<u128> = galaxy_pairs2.iter().map(get_steps).collect();

        assert_eq!(steps_betwixt1.iter().sum::<u128>(), 1030);
        assert_eq!(steps_betwixt2.iter().sum::<u128>(), 8410);
    }
}
