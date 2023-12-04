fn main() {
    let input = include_str!("../input.txt");

    println!("part 1: {}", part_1(input));
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

#[cfg(test)]
mod tests {
    use super::*;

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

    // #[test]
    // fn test_2() {
    //     assert_eq!(run_on_input(TEST_INPUT, part_2), 2286);
    // }
}
