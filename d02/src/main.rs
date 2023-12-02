mod data;

use data::{Game, Reveal};

fn main() {
    let input = include_str!("../input.txt");

    println!("part 1: {}", run_on_input(input, part_1));
    println!("part 2: {}", run_on_input(input, part_2));
}

fn run_on_input<F>(input: &str, f: F) -> u64
where
    F: Fn(&Game) -> Option<u64>,
{
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| {
            let game: Game = l.parse().unwrap();
            f(&game)
        })
        .sum()
}

fn part_1(game: &Game) -> Option<u64> {
    game.reveals
        .iter()
        .all(|reveal| reveal.red <= 12 && reveal.blue <= 14 && reveal.green <= 13)
        .then_some(game.id)
}

fn part_2(game: &Game) -> Option<u64> {
    let max_reveal = game
        .reveals
        .iter()
        .fold(Reveal::default(), |acc, reveal| acc.max(reveal));

    Some(max_reveal.red * max_reveal.blue * max_reveal.green)
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
        assert_eq!(run_on_input(TEST_INPUT, part_1), 8);
    }

    #[test]
    fn test_2() {
        assert_eq!(run_on_input(TEST_INPUT, part_2), 2286);
    }
}
