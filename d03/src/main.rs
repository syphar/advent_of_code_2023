fn main() {
    let input = include_str!("../input.txt");

    println!("part 1: {}", part_1(input));
    println!("part 2: {}", part_2(input));
}

fn contains_symbol(line: &str, from: i64, to: i64) -> bool {
    let from = from.max(0) as usize;
    let to = to.min(line.len() as i64) as usize;

    for c in line.chars().skip(from).take(to - from) {
        if !(c.is_whitespace() || c.is_ascii_digit() || c == '.') {
            return true;
        }
    }
    false
}

fn part_1(input: &str) -> u64 {
    let lines: Vec<_> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(str::trim)
        .collect();

    let mut sum = 0;

    for y in 0..lines.len() {
        let mut line = lines[y].to_owned();
        line.push('.');

        let mut number = String::new();
        let mut number_start: Option<usize> = None;
        for x in 0..line.len() {
            let c = line.chars().nth(x).unwrap();

            if c.is_ascii_digit() {
                number.push(c);
                if number_start.is_none() {
                    number_start = Some(x);
                }
            } else if !number.is_empty() {
                if (y > 0
                    && contains_symbol(
                        lines[y - 1],
                        number_start.unwrap() as i64 - 1,
                        (x + 1) as i64,
                    ))
                    || (y < lines.len() - 1
                        && contains_symbol(
                            lines[y + 1],
                            number_start.unwrap() as i64 - 1,
                            (x + 1) as i64,
                        ))
                    || contains_symbol(&line, number_start.unwrap() as i64 - 1, (x + 1) as i64)
                {
                    sum += number.parse::<u64>().unwrap();
                }

                number.clear();
                number_start = None;
            }
        }
    }

    sum
}

fn fetch_number(line: &str, start: i64) -> Option<u64> {
    let start = start.max(0) as usize;

    let Some(start_ch) = line.chars().nth(start) else {
        return None;
    };
    if !start_ch.is_ascii_digit() {
        return None;
    }

    let mut number = String::from(start_ch);

    for idx in (0..start).rev() {
        let ch = line.chars().nth(idx).unwrap();
        if ch.is_ascii_digit() {
            number.insert(0, ch);
        } else {
            break;
        }
    }

    for idx in (start + 1)..line.len() {
        if let Some(ch) = line.chars().nth(idx) {
            if ch.is_ascii_digit() {
                number.push(ch)
            } else {
                break;
            }
        }
    }

    Some(number.parse::<u64>().unwrap())
}

fn part_2(input: &str) -> u64 {
    let lines: Vec<_> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(str::trim)
        .map(|l| format!(".{}.", l))
        .collect();

    let mut sum = 0;

    for y in 0..lines.len() {
        let line = lines[y].to_owned();

        for x in 0..line.len() {
            let c = line.chars().nth(x).unwrap();

            if c == '*' {
                let mut gears = Vec::new();
                if let Some(number) = fetch_number(&line, x as i64 - 1) {
                    gears.push(number);
                }

                if let Some(number) = fetch_number(&line, x as i64 + 1) {
                    gears.push(number);
                }

                if let Some(number) = fetch_number(&lines[y - 1], x as i64) {
                    gears.push(number);
                } else {
                    if let Some(number) = fetch_number(&lines[y - 1], x as i64 - 1) {
                        gears.push(number);
                    }
                    if let Some(number) = fetch_number(&lines[y - 1], x as i64 + 1) {
                        gears.push(number);
                    }
                }

                if let Some(number) = fetch_number(&lines[y + 1], x as i64) {
                    gears.push(number);
                } else {
                    if let Some(number) = fetch_number(&lines[y + 1], x as i64 - 1) {
                        gears.push(number);
                    }
                    if let Some(number) = fetch_number(&lines[y + 1], x as i64 + 1) {
                        gears.push(number);
                    }
                }

                if gears.len() == 2 {
                    let ratio = gears[0] * gears[1];
                    sum += ratio;
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    static TEST_INPUT: &str = "
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        ";

    #[test]
    fn test_1() {
        assert_eq!(part_1(TEST_INPUT), 4361);
    }

    #[test]
    fn test_2() {
        assert_eq!(part_2(TEST_INPUT), 467835);
    }

    #[test_case("..456..", -1, None; "test 1")]
    #[test_case("..456..", 0, None)]
    #[test_case("..456..", 1, None)]
    #[test_case("..456..", 2, Some(456))]
    #[test_case("..456..", 3, Some(456))]
    #[test_case("..456..", 4, Some(456))]
    #[test_case("..456..", 5, None)]
    #[test_case("789..", -2, Some(789); "test2")]
    #[test_case("789..", 0, Some(789))]
    #[test_case("789..", 1, Some(789))]
    #[test_case("789..", 2, Some(789))]
    #[test_case("789..", 3, None)]
    #[test_case("789..", 4, None)]
    #[test_case("789..", 5, None)]
    #[test_case("789..", 6, None)]
    fn test_fetch_number(line: &str, start: i64, expected: Option<u64>) {
        assert_eq!(fetch_number(line, start), expected);
    }
}
