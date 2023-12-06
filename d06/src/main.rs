fn main() {
    let input = include_str!("../input.txt");

    println!("part 1: {}", run(parse_races_1(input).iter().copied()));
    println!("part 2: {}", run(parse_races_2(input).iter().copied()));
}

fn calc_distance(charge: u64, race_len: u64) -> u64 {
    let race_for = race_len - charge;
    charge * race_for
}

fn parse_races_1(input: &str) -> Vec<(u64, u64)> {
    let mut lines = input.lines().map(|l| l.trim()).filter(|l| !l.is_empty());

    let (_, times) = lines.next().unwrap().split_once(':').unwrap();
    let (_, distances) = lines.next().unwrap().split_once(':').unwrap();

    times
        .split_whitespace()
        .zip(distances.split_whitespace())
        .map(|(t, d)| (t.parse().unwrap(), d.parse().unwrap()))
        .collect()
}

fn parse_races_2(input: &str) -> Vec<(u64, u64)> {
    let mut lines = input.lines().map(|l| l.trim()).filter(|l| !l.is_empty());

    let (_, times) = lines.next().unwrap().split_once(':').unwrap();
    let (_, distances) = lines.next().unwrap().split_once(':').unwrap();

    vec![(
        times.replace(' ', "").parse().unwrap(),
        distances.replace(' ', "").parse().unwrap(),
    )]
}

fn run(races: impl Iterator<Item = (u64, u64)>) -> u64 {
    let mut sum = 1;

    for (race_len, record_distance) in races {
        let mut wins = 0;
        for try_charge in 1..=race_len {
            let distance = calc_distance(try_charge, race_len);
            if distance > record_distance {
                wins += 1;
            }
        }
        if wins > 0 {
            sum *= wins;
        }
    }
    // races
    //     .map(|(race_len, record_distance)| {
    //         (1..=race_len)
    //             .filter(|&try_charge| calc_distance(try_charge, race_len) > record_distance)
    //             .count() as u64
    //     })
    //     .fold(0u64, |acc, x| if x > 0 { acc * x } else { acc })
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    static TEST_INPUT: &str = "
        Time:      7  15   30
        Distance:  9  40  200
        ";

    #[test_case(0, 0)]
    #[test_case(1, 6)]
    #[test_case(2, 10)]
    #[test_case(3, 12)]
    #[test_case(4, 12)]
    #[test_case(7, 0)]
    fn test_calc_distance(charge: u64, expected: u64) {
        assert_eq!(calc_distance(charge, 7), expected);
    }

    #[test]
    fn test_1() {
        assert_eq!(run(parse_races_1(TEST_INPUT).iter().copied()), 288);
    }

    #[test]
    fn test_2() {
        assert_eq!(run(parse_races_2(TEST_INPUT).iter().copied()), 71503);
    }
}
