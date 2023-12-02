use std::str::FromStr;

use regex::Regex;

fn main() {
    let games: Vec<Game> =
        parse_input(&std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap());

    println!("part 1: {}", part_1(games.iter()));
    println!("part 2: {}", part_2(games.iter()));
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

#[derive(Debug, Eq, PartialEq, Default)]
struct Reveal {
    red: u64,
    blue: u64,
    green: u64,
}

impl FromStr for Reveal {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut reveal = Self::default();

        for color_show in s.split(',') {
            let (amount, color) = color_show.trim().split_once(' ').unwrap();
            let amount: u64 = amount.parse().unwrap();

            match color {
                "red" => reveal.red += amount,
                "blue" => reveal.blue += amount,
                "green" => reveal.green += amount,
                _ => panic!("unknown color {}", color),
            }
        }
        Ok(reveal)
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

fn part_2<'a>(games: impl Iterator<Item = &'a Game>) -> u64 {
    games
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
    fn parse_game() {
        assert_eq!(
            Game::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap(),
            Game {
                id: 1,
                reveals: vec![
                    Reveal {
                        blue: 3,
                        red: 4,
                        ..Default::default()
                    },
                    Reveal {
                        green: 2,
                        blue: 6,
                        red: 1
                    },
                    Reveal {
                        green: 2,
                        ..Default::default()
                    },
                ]
            }
        );
    }

    #[test]
    fn parse_reveal_blue() {
        assert_eq!(
            Reveal::from_str("3 blue").unwrap(),
            Reveal {
                blue: 3,
                ..Default::default()
            },
        );
    }

    #[test]
    fn parse_reveal_green() {
        assert_eq!(
            Reveal::from_str("3 green").unwrap(),
            Reveal {
                green: 3,
                ..Default::default()
            },
        );
    }

    #[test]
    fn parse_reveal_red() {
        assert_eq!(
            Reveal::from_str("3 red").unwrap(),
            Reveal {
                red: 3,
                ..Default::default()
            },
        );
    }

    #[test]
    fn parse_reveal_multi() {
        assert_eq!(
            Reveal::from_str("6 red, 1 blue, 3 green").unwrap(),
            Reveal {
                red: 6,
                blue: 1,
                green: 3
            }
        );
    }

    #[test]
    fn test_1() {
        assert_eq!(part_1(parse_input(TEST_INPUT).iter()), 8);
    }

    #[test]
    fn test_2() {
        assert_eq!(part_2(parse_input(TEST_INPUT).iter()), 2286);
    }
}
