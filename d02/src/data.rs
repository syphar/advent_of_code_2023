use std::str::FromStr;

use regex::Regex;

#[derive(Debug, Eq, PartialEq, Default)]
pub(crate) struct Reveal {
    pub(crate) red: u64,
    pub(crate) blue: u64,
    pub(crate) green: u64,
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
pub(crate) struct Game {
    pub(crate) id: u64,
    pub(crate) reveals: Vec<Reveal>,
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
