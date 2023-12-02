use std::{collections::HashMap, str::FromStr};

use regex::Regex;
use strum::EnumString;

fn main() {
    let games: Vec<Game> = std::fs::read_to_string(std::env::args().nth(1).unwrap())
        .unwrap()
        .lines()
        .filter_map(|l| {
            if l.trim().is_empty() {
                None
            } else {
                Some(l.parse().unwrap())
            }
        })
        .collect();

    println!("part 1: {}", part_1(games.iter()));
    println!("part 2: {}", part_2(games.iter()));
}

#[derive(Debug, Hash, Eq, PartialEq, EnumString)]
#[strum(serialize_all = "lowercase")]
enum Color {
    Red,
    Blue,
    Green,
}

#[derive(Debug, Eq, PartialEq)]
struct Reveal(HashMap<Color, u64>);

impl FromStr for Reveal {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut colors = HashMap::new();
        for color_show in s.split(',') {
            let (amount, color) = color_show.trim().split_once(' ').unwrap();
            let amount: u64 = amount.parse().unwrap();
            let color = color.parse().unwrap();

            debug_assert!(!colors.contains_key(&color));
            colors.insert(color, amount);
        }
        Ok(Self(colors))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Game {
    id: u64,
    reveals: Vec<Reveal>,
}

impl FromStr for Game {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line_pattern = Regex::new(r#"Game (\d+): (.*)"#).unwrap();
        println!("line: {:?}", s.trim());
        let line = line_pattern.captures(s.trim()).unwrap();
        let id: u64 = line.get(1).unwrap().as_str().parse().unwrap();
        let reveals = line
            .get(2)
            .unwrap()
            .as_str()
            .split(';')
            .map(|s| s.trim().parse().unwrap())
            .collect();
        Ok(Self { id, reveals })
    }
}

fn part_1<'a>(games: impl Iterator<Item = &'a Game>) -> u64 {
    games
        .map(|game| {
            for reveal in &game.reveals {
                let mut amount_red = 0;
                let mut amount_blue = 0;
                let mut amount_green = 0;

                for (color, amount) in &reveal.0 {
                    match color {
                        Color::Red => amount_red += amount,
                        Color::Blue => amount_blue += amount,
                        Color::Green => amount_green += amount,
                    }
                }
                if amount_red > 12 || amount_green > 13 || amount_blue > 14 {
                    return 0;
                }
            }

            game.id
        })
        .sum()
}

fn part_2<'a>(games: impl Iterator<Item = &'a Game>) -> u64 {
    games
        .map(|game| {
            let mut max_red = 0;
            let mut max_blue = 0;
            let mut max_green = 0;

            for reveal in game.reveals.iter() {
                for (color, amount) in &reveal.0 {
                    match color {
                        Color::Red => max_red = max_red.max(*amount),
                        Color::Blue => max_blue = max_blue.max(*amount),
                        Color::Green => max_green = max_green.max(*amount),
                    }
                }
            }

            max_red * max_blue * max_green
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    static TEST_INPUT: &str = "
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";

    #[test]
    fn parse_game() {
        assert_eq!(
            Game::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap(),
            Game {
                id: 1,
                reveals: vec![
                    Reveal(HashMap::from_iter([(Color::Blue, 3), (Color::Red, 4),])),
                    Reveal(HashMap::from_iter([
                        (Color::Green, 2),
                        (Color::Blue, 6),
                        (Color::Red, 1),
                    ])),
                    Reveal(HashMap::from_iter([(Color::Green, 2),])),
                ]
            }
        );
    }

    #[test]
    fn parse_color() {
        assert_eq!("red".parse::<Color>().unwrap(), Color::Red);
        assert_eq!("blue".parse::<Color>().unwrap(), Color::Blue);
        assert_eq!("green".parse::<Color>().unwrap(), Color::Green);
    }

    #[test_case("blue")]
    #[test_case("red")]
    #[test_case("green")]
    fn parse_reveal(color: &str) {
        assert_eq!(
            Reveal::from_str(&format!("3 {}", color)).unwrap().0,
            HashMap::from_iter([(color.parse().unwrap(), 3)]),
        );
    }

    #[test]
    fn parse_reveal_multi() {
        assert_eq!(
            Reveal::from_str("6 red, 1 blue, 3 green").unwrap().0,
            HashMap::from_iter([(Color::Red, 6), (Color::Blue, 1), (Color::Green, 3)])
        );
    }

    #[test]
    fn test_1() {
        let games: Vec<Game> = TEST_INPUT
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|l| l.parse().unwrap())
            .collect();
        assert_eq!(part_1(games.iter()), 8);
    }

    #[test]
    fn test_2() {
        let games: Vec<Game> = TEST_INPUT
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|l| l.parse().unwrap())
            .collect();
        assert_eq!(part_2(games.iter()), 2286);
    }
}
