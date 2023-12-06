mod data;

use data::Card;
use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("../input.txt");

    println!("part 1: {}", part_1(input));
    println!("part 2: {}", part_2(input));
}

fn part_1(input: &str) -> u64 {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let card: Card = l.parse().unwrap();

            if card.wins() > 0 {
                2u64.pow(card.wins() as u32 - 1)
            } else {
                0
            }
        })
        .sum()
}

fn part_2(input: &str) -> u64 {
    let mut cards: VecDeque<Card> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.parse().unwrap())
        .collect();

    let cards_by_number: HashMap<u64, Card> =
        HashMap::from_iter(cards.iter().cloned().map(|c| (c.number, c)));

    let mut cards_count = cards.len();

    loop {
        while let Some(card) = cards.pop_front() {
            for num in (card.number + 1)..=(card.number + card.wins()) {
                if let Some(copied_card) = cards_by_number.get(&num) {
                    cards_count += 1;
                    cards.push_back(copied_card.clone());
                }
            }
        }
        if cards.is_empty() {
            break;
        }
    }

    cards_count as u64
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

    #[test]
    fn test_2() {
        assert_eq!(part_2(TEST_INPUT), 30);
    }
}
