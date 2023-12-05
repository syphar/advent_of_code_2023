use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    println!("part 1: {}", part_1(input));
    // println!("part 2: {}", run_on_input(input, part_2));
}

fn part_1(input: &str) -> u64 {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let (_, numbers) = l.split_once(':').unwrap();
            let (winners, mine) = numbers.split_once('|').unwrap();

            let winners: HashSet<u64> = winners
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            let mine: HashSet<u64> = mine
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            let won = winners.intersection(&mine).count();

            if won > 0 {
                2u64.pow(won as u32 - 1)
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ";

    #[test]
    fn test_1() {
        assert_eq!(part_1(TEST_INPUT), 13);
    }

    // #[test]
    // fn test_2() {
    //     assert_eq!(run_on_input(TEST_INPUT, part_2), 2286);
    // }
}
