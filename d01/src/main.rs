fn main() {
    let lines: Vec<String> = std::fs::read_to_string(std::env::args().nth(1).unwrap())
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

    println!("day 1: {}", part_1(lines.iter()));
    // println!("day 2: {}", part_2(lines.iter()));
}

fn part_1<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> u64 {
    lines
        .filter_map(|line| {
            let line = line.as_ref().trim();
            let digits: Vec<u16> = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c as u16 - '0' as u16)
                .collect();

            (!digits.is_empty())
                .then(|| *digits.first().unwrap() as u64 * 10 + *digits.last().unwrap() as u64)
        })
        .sum()
}

fn part_2<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> u64 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
        ";

    #[test]
    fn test_1() {
        assert_eq!(part_1(TEST_INPUT.lines()), 142)
    }

    // #[test]
    // fn test_2() {
    //     assert_eq!(part_2(TEST_INPUT.lines()), 45000)
    // }
}
