mod data;

use data::Game;

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
    for reveal in &game.reveals {
        if reveal.red > 12 {
            return None;
        }
        if reveal.blue > 14 {
            return None;
        }
        if reveal.green > 13 {
            return None;
        }
    }

    Some(game.id)
}

fn part_2(game: &Game) -> Option<u64> {
    let mut max_red = 0;
    let mut max_blue = 0;
    let mut max_green = 0;

    for reveal in game.reveals.iter() {
        max_red = max_red.max(reveal.red);
        max_blue = max_blue.max(reveal.blue);
        max_green = max_green.max(reveal.green);
    }

    Some(max_red * max_blue * max_green)
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
