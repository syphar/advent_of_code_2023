mod data;

use data::Game;

fn main() {
    let games: Vec<Game> =
        parse_input(&std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap());

    println!("part 1: {}", part_1(games.iter().cloned()));
    println!("part 2: {}", part_2(games.iter().cloned()));
}

fn parse_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .filter_map(|l| {
            if l.trim().is_empty() {
                None
            } else {
                Some(l.parse().unwrap())
            }
        })
        .collect()
}

fn part_1(games: impl IntoIterator<Item = Game>) -> u64 {
    games
        .into_iter()
        .map(|game| {
            for reveal in &game.reveals {
                if reveal.red > 12 {
                    return 0;
                }
                if reveal.blue > 14 {
                    return 0;
                }
                if reveal.green > 13 {
                    return 0;
                }
            }

            game.id
        })
        .sum()
}

fn part_2(games: impl IntoIterator<Item = Game>) -> u64 {
    games
        .into_iter()
        .map(|game| {
            let mut max_red = 0;
            let mut max_blue = 0;
            let mut max_green = 0;

            for reveal in game.reveals.iter() {
                max_red = max_red.max(reveal.red);
                max_blue = max_blue.max(reveal.blue);
                max_green = max_green.max(reveal.green);
            }

            max_red * max_blue * max_green
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";

    #[test]
    fn test_1() {
        assert_eq!(part_1(parse_input(TEST_INPUT)), 8);
    }

    #[test]
    fn test_2() {
        assert_eq!(part_2(parse_input(TEST_INPUT)), 2286);
    }
}
