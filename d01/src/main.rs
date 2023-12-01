fn main() {
    let lines: Vec<String> = std::fs::read_to_string(std::env::args().nth(1).unwrap())
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

    println!("part 1: {}", run(lines.iter()));
    println!("part 2: {}", run(lines.iter()));
}

fn find_digit(line: &[u8], range: impl IntoIterator<Item = usize>) -> Option<u64> {
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

    for start in range {
        if line[start].is_ascii_digit() {
            return Some(line[start] as u64 - b'0' as u64);
        }

        for (value, digit) in DIGITS.iter() {
            if line[start..].starts_with(value) {
                return Some(*digit);
            }
        }
    }
    None
}

fn run<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> u64 {
    lines
        .filter_map(|line| {
            let line = line.as_ref().trim().as_bytes();

            let first_digit = find_digit(line, 0usize..line.len());
            let last_digit = find_digit(line, (0usize..line.len()).rev());

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
            run("1abc2
                 pqr3stu8vwx
                 a1b2c3d4e5f
                 treb7uchet"
                .lines()),
            142
        )
    }

    #[test]
    fn test_2() {
        assert_eq!(
            run("
                two1nine
                eightwothree
                abcone2threexyz
                xtwone3four
                4nineeightseven2
                zoneight234
                7pqrstsixteen"
                .lines()),
            281
        );
    }
}
