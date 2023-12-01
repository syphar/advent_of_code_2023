fn main() {
    let lines: Vec<String> = std::fs::read_to_string(std::env::args().nth(1).unwrap())
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

    println!("part 1: {}", part_1(lines.iter()));
    println!("part 2: {}", part_2(lines.iter()));
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
    static DIGITS: [(&[u8], u64); 10] = [
        (b"one", 1),
        (b"two", 2),
        (b"three", 3),
        (b"four", 4),
        (b"five", 5),
        (b"six", 6),
        (b"seven", 7),
        (b"eight", 8),
        (b"nine", 9),
        (b"zero", 0),
    ];

    lines
        .filter_map(|line| {
            let line = line.as_ref().trim();
            let line = line.as_bytes();

            let mut first_digit: Option<u64> = None;
            'outer: for start in 0usize..line.len() {
                if line[start].is_ascii_digit() {
                    first_digit = Some(line[start] as u64 - b'0' as u64);
                    break 'outer;
                }

                for (value, digit) in DIGITS.iter() {
                    if line[start..].starts_with(value) {
                        first_digit = Some(*digit);
                        break 'outer;
                    }
                }
            }

            let mut last_digit: Option<u64> = None;
            'outer: for start in (0usize..line.len()).rev() {
                if line[start].is_ascii_digit() {
                    last_digit = Some(line[start] as u64 - b'0' as u64);
                    break 'outer;
                }

                for (value, digit) in DIGITS.iter() {
                    if line[start..].starts_with(value) {
                        last_digit = Some(*digit);
                        break 'outer;
                    }
                }
            }

            Some(first_digit? * 10 + last_digit?)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            part_1(
                "1abc2
                 pqr3stu8vwx
                 a1b2c3d4e5f
                 treb7uchet"
                    .lines()
            ),
            142
        )
    }

    #[test]
    fn test_2() {
        assert_eq!(
            part_2(
                "
                two1nine
                eightwothree
                abcone2threexyz
                xtwone3four
                4nineeightseven2
                zoneight234
                7pqrstsixteen"
                    .lines()
            ),
            281
        );
    }
}
