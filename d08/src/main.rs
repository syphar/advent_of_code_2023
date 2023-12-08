mod data;
use std::collections::HashMap;

use data::Direction;
use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");

    let (directions, instructions) = parse(input);

    println!("part 1: {}", part_1(&directions, &instructions));
    println!("part 2: {}", part_2(&directions, &instructions));
}

fn parse(input: &str) -> (Vec<Direction>, HashMap<String, (String, String)>) {
    let mut lines = input.lines().map(str::trim).filter(|line| !line.is_empty());

    let directions: Vec<_> = lines
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(Direction::from)
        .collect();

    let pattern = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    let instructions: HashMap<_, _> = lines
        .map(|line| {
            let captures = pattern.captures(line).unwrap();
            (
                captures.get(1).unwrap().as_str().to_owned(),
                (
                    captures.get(2).unwrap().as_str().to_owned(),
                    captures.get(3).unwrap().as_str().to_owned(),
                ),
            )
        })
        .collect();

    (directions, instructions)
}

fn part_1(directions: &[Direction], instructions: &HashMap<String, (String, String)>) -> u64 {
    let mut position = "AAA";
    let mut steps = 0;
    for direction in directions.iter().cycle() {
        let next_move = instructions.get(position).unwrap();

        match direction {
            Direction::Left => position = &next_move.0,
            Direction::Right => position = &next_move.1,
        };

        steps += 1;

        if position == "ZZZ" {
            return steps;
        }
    }
    unreachable!();
}

fn part_2(directions: &[Direction], instructions: &HashMap<String, (String, String)>) -> u64 {
    let mut positions: Vec<_> = instructions
        .keys()
        .filter(|key| key.ends_with('A'))
        .collect();

    let mut steps = 0;
    for direction in directions.iter().cycle() {
        for position in &mut positions {
            let next_move = instructions.get(*position).unwrap();

            match direction {
                Direction::Left => *position = &next_move.0,
                Direction::Right => *position = &next_move.1,
            };
        }

        steps += 1;

        if positions.iter().all(|position| position.ends_with('Z')) {
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

    static TEST_INPUT_3: &str = "
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
        ";

    #[test]
    fn test_1() {
        let (directions, instructions) = parse(TEST_INPUT);
        assert_eq!(part_1(&directions, &instructions), 2);
    }

    #[test]
    fn test_1_2() {
        let (directions, instructions) = parse(TEST_INPUT_2);
        assert_eq!(part_1(&directions, &instructions), 6);
    }

    #[test]
    fn test_2() {
        let (directions, instructions) = parse(TEST_INPUT_3);
        assert_eq!(part_2(&directions, &instructions), 6);
    }
}
