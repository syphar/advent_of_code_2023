use regex::Regex;

fn main() {
    let lines: Vec<String> = std::fs::read_to_string(std::env::args().nth(1).unwrap())
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

    println!("part 1: {}", part_1(lines.iter()));
    // println!("part 2: {}", run(lines.iter()));
}

fn part_1<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> u64 {
    let line_pattern = Regex::new(r#"Game (\d+): (.*)"#).unwrap();

    lines
        .filter_map(|line| {
            let line = line_pattern.captures(line.as_ref().trim())?;
            let game_id: u64 = line.get(1).unwrap().as_str().parse().unwrap();
            println!("game_id: {}", game_id);

            for reveal in line.get(2).unwrap().as_str().split(';') {
                println!("reveal: {}", reveal);
                let mut amount_red = 0;
                let mut amount_blue = 0;
                let mut amount_green = 0;
                for color_show in reveal.split(',') {
                    let (amount, color) = color_show.trim().split_once(' ').unwrap();
                    println!("color: {}, amount: {}", color, amount);

                    let amount: u64 = amount.parse().unwrap();

                    match color {
                        "red" => amount_red += amount,
                        "blue" => amount_blue += amount,
                        "green" => amount_green += amount,
                        _ => panic!("unknown color: {}", color),
                    }
                }
                if amount_red > 12 || amount_green > 13 || amount_blue > 14 {
                    return None;
                }
            }

            Some(game_id)
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
        assert_eq!(part_1(TEST_INPUT.lines()), 8);
    }
}
