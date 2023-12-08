mod data;
use std::collections::HashMap;

use data::Direction;
use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");

    println!("part 1: {}", part_1(input));
    // println!("part 2: {}", run(parse_races_2(input).iter().copied()));
}

fn part_1(input: &str) -> u64 {
    let mut lines = input.lines().map(str::trim).filter(|line| !line.is_empty());

    let directions: Vec<_> = lines
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(Direction::from)
        .collect();

    let pattern = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    let rules: HashMap<_, _> = lines
        .map(|line| {
            let captures = pattern.captures(line).unwrap();
            (
                captures.get(1).unwrap().as_str(),
                (
                    captures.get(2).unwrap().as_str(),
                    captures.get(3).unwrap().as_str(),
                ),
            )
        })
        .collect();

    let mut position = "AAA";
    let mut steps = 0;
    for direction in directions.iter().cycle() {
        let next_move = rules.get(position).unwrap();

        match direction {
            Direction::Left => position = next_move.0,
            Direction::Right => position = next_move.1,
        };

        steps += 1;

        if position == "ZZZ" {
            return steps;
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
        ";

    static TEST_INPUT_2: &str = "
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
        ";

    #[test]
    fn test_1() {
        assert_eq!(part_1(TEST_INPUT), 2);
    }

    #[test]
    fn test_1_2() {
        assert_eq!(part_1(TEST_INPUT_2), 6);
    }
}
