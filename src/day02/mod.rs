const INPUT: &str = include_str!("./input.txt");

#[derive(Default, Debug)]
struct CubeSet {
    red: u64,
    green: u64,
    blue: u64,
}

impl CubeSet {
    fn from_input(input: &str) -> impl Iterator<Item = impl Iterator<Item = CubeSet> + '_> + '_ {
        input.lines().map(Self::from_game)
    }

    fn from_game(s: &str) -> impl Iterator<Item = CubeSet> + '_ {
        s.split_once(": ")
            .unwrap()
            .1
            .split("; ")
            .map(CubeSet::from_set)
    }

    fn from_set(set_str: &str) -> Self {
        let mut cube_set = Self::default();
        let split = set_str.split(", ");

        for s in split {
            let (number, color): (&str, &str) = s.split_once(' ').unwrap();
            let number: u64 = number.parse().unwrap();
            match color {
                "red" => cube_set.red += number,
                "green" => cube_set.green += number,
                "blue" => cube_set.blue += number,
                _ => panic!(),
            };
        }

        cube_set
    }

    fn is_playable(&self, limit: &Self) -> bool {
        self.red <= limit.red && self.green <= limit.green && self.blue <= limit.blue
    }
}

pub fn part_one() {
    let limit = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games = CubeSet::from_input(INPUT);
    let id_sum = games
        .enumerate()
        .map(|(key, game)| (key + 1, game))
        .map(|(id, mut game)| (id, game.all(|cube_set| cube_set.is_playable(&limit))))
        .filter(|(_, predicate)| *predicate)
        .map(|(id, _)| id)
        .sum::<usize>() as u64;
    println!("{}", id_sum);
}

// pub fn part_two() {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::CubeSet;

    #[test]
    fn playable_games_are_correct() {
        let example = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let limit = CubeSet {
            red: 12,
            green: 13,
            blue: 14,
        };
        let games_iter = CubeSet::from_input(example);
        let mut are_playable = games_iter
            .enumerate()
            .map(|(key, game)| (key + 1, game))
            .map(|(id, mut game)| (id, game.all(|cube_set| cube_set.is_playable(&limit))));

        assert!(are_playable.next().unwrap().1);
        assert!(are_playable.next().unwrap().1);
        assert!(!are_playable.next().unwrap().1);
        assert!(!are_playable.next().unwrap().1);
        assert!(are_playable.next().unwrap().1);

        let games_iter = CubeSet::from_input(example);
        let sum = games_iter
            .enumerate()
            .map(|(key, game)| (key + 1, game))
            .map(|(id, mut game)| (id, game.all(|cube_set| cube_set.is_playable(&limit))))
            .filter(|(_, predicate)| *predicate)
            .map(|(id, _)| id)
            .sum::<usize>() as u64;
        assert_eq!(sum, 8);
    }
}
