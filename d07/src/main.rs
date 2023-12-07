mod data;
use data::Hand;

fn main() {
    let input = include_str!("../input.txt");

    println!("part 1: {}", part_1(parse(input)));
    // println!("part 2: {}", run(parse_races_2(input).iter().copied()));
}

fn parse(input: &str) -> Vec<(Hand, u64)> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (card, bid) = line.split_once(' ').unwrap();
            (card.parse().unwrap(), bid.parse().unwrap())
        })
        .collect()
}

fn part_1(mut hands: Vec<(Hand, u64)>) -> u64 {
    hands.sort();

    hands
        .iter()
        .map(|(_, bid)| bid)
        .enumerate()
        .map(|(rank, bid)| (rank + 1) as u64 * bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
        ";

    #[test]
    fn test_1() {
        let data = parse(TEST_INPUT);
        assert_eq!(part_1(data), 6440);
    }

    // #[test]
    // fn test_2() {
    //     assert_eq!(run(parse_races_2(TEST_INPUT).iter().copied()), 71503);
    // }
}
