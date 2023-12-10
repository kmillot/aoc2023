use crate::utils::parse_from_bytes;

const INPUT: &[u8] = include_bytes!("./input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Pipe {
    fn from_byte(byte: u8) -> Self {
        match byte {
            b'|' => Self::NorthSouth,
            b'-' => Self::EastWest,
            b'L' => Self::NorthEast,
            b'J' => Self::NorthWest,
            b'7' => Self::SouthWest,
            b'F' => Self::SouthEast,
            b'.' => Self::Ground,
            b'S' => Self::Start,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    West,
    East,
    South,
}

impl Direction {
    fn next_from_pipe(&self, next_pipe: &Pipe) -> Option<Self> {
        match next_pipe {
            Pipe::NorthSouth => match self {
                Self::East | Self::West => None,
                dir => Some(*dir),
            },
            Pipe::EastWest => match self {
                Self::North | Self::South => None,
                dir => Some(*dir),
            },
            Pipe::NorthEast => match self {
                Self::West => Some(Self::North),
                Self::South => Some(Self::East),
                _ => None,
            },
            Pipe::NorthWest => match self {
                Self::East => Some(Self::North),
                Self::South => Some(Self::West),
                _ => None,
            },
            Pipe::SouthWest => match self {
                Self::East => Some(Self::South),
                Self::North => Some(Self::West),
                _ => None,
            },
            Pipe::SouthEast => match self {
                Self::West => Some(Self::South),
                Self::North => Some(Self::East),
                _ => None,
            },
            _ => None,
        }
    }
}

pub fn part_one() {
    let pipes = parse_input(INPUT);
    let furthest = get_furthest_path_steps(pipes.as_slice());

    println!("10.1 answer: {}", furthest);
}

// pub fn part_two() {
//     todo!()
// }

fn parse_input(input: &[u8]) -> Vec<Vec<Pipe>> {
    parse_from_bytes(input)
        .map(|line| line.map(Pipe::from_byte).collect())
        .collect()
}

fn get_furthest_path_steps(pipes: &[Vec<Pipe>]) -> u64 {
    let (start_y, start_x) = find_start(pipes);
    let start_directions = [
        Direction::North,
        Direction::West,
        Direction::East,
        Direction::South,
    ];

    start_directions
        .iter()
        .filter_map(|dir| get_steps_to_loop(pipes, start_y, start_x, *dir))
        .max()
        .unwrap()
        / 2
}

fn get_steps_to_loop(
    pipes: &[Vec<Pipe>],
    start_y: usize,
    start_x: usize,
    start_dir: Direction,
) -> Option<u64> {
    let (mut y, mut x) = (start_y, start_x);
    let mut direction = Some(start_dir);
    let mut step_count = 0;

    while let Some(dir) = direction {
        if let Some((pipe_y, pipe_x)) = get_next_step(pipes, dir, y, x) {
            y = pipe_y;
            x = pipe_x;
            direction = dir.next_from_pipe(&pipes[y][x]);
        } else {
            direction = None;
        }
        step_count += 1;
        if pipes[y][x] == Pipe::Start {
            return Some(step_count);
        }
    }
    None
}

fn get_next_step(
    pipes: &[Vec<Pipe>],
    direction: Direction,
    y: usize,
    x: usize,
) -> Option<(usize, usize)> {
    let (y, x) = match direction {
        Direction::North => (y.checked_sub(1), Some(x)),
        Direction::West => (Some(y), x.checked_sub(1)),
        Direction::East => (Some(y), x.checked_add(1)),
        Direction::South => (y.checked_add(1), Some(x)),
    };

    if let (Some(y), Some(x)) = (y, x) {
        if let Some(line) = pipes.get(y) {
            if line.get(x).is_some() {
                return Some((y, x));
            }
        }
    }
    None
}

fn find_start(pipes: &[Vec<Pipe>]) -> (usize, usize) {
    pipes
        .iter()
        .enumerate()
        .find_map(|line| {
            line.1
                .iter()
                .enumerate()
                .find(|pipe| pipe.1 == &Pipe::Start)
                .map(|start| (line.0, start.0))
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::{find_start, get_furthest_path_steps, parse_input};

    const EXAMPLE1: &[u8] = b"-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    const EXAMPLE2: &[u8] = b"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    #[test]
    fn part_one_works() {
        let pipes1 = parse_input(EXAMPLE1);
        let pipes2 = parse_input(EXAMPLE2);
        let furthest1 = get_furthest_path_steps(pipes1.as_slice());
        let furthest2 = get_furthest_path_steps(pipes2.as_slice());

        assert_eq!(furthest1, 4);
        assert_eq!(furthest2, 8);
    }

    #[test]
    fn find_start_works() {
        let pipes1 = parse_input(EXAMPLE1);
        let pipes2 = parse_input(EXAMPLE2);
        let start1 = find_start(pipes1.as_slice());
        let start2 = find_start(pipes2.as_slice());

        assert_eq!(start1, (1, 1));
        assert_eq!(start2, (2, 0));
    }
}
