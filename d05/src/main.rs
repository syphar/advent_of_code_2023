mod data;
use data::Map;
use itertools::Itertools;
use rayon::prelude::*;

fn main() {
    let (seeds, maps) = parse_input(include_str!("../input.txt"));

    println!("part 1: {}", part_1(&seeds, maps.clone()));
    println!("part 2: {}", part_2(&seeds, maps));
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<Map>) {
    let mut lines = input.lines().map(str::trim).filter(|l| !l.is_empty());

    let seed_line = lines.next().unwrap();
    assert!(seed_line.starts_with("seeds: "));
    let seeds: Vec<u64> = seed_line
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();

    let mut maps: Vec<Map> = Vec::new();
    let mut map_inputs = Vec::new();
    for map_line in lines {
        if map_line.chars().next().unwrap().is_ascii_digit() {
            map_inputs.push(map_line);
        } else {
            maps.push(map_inputs.join("\n").parse().unwrap());
            map_inputs.clear();
        }
    }
    if !map_inputs.is_empty() {
        maps.push(map_inputs.join("\n").parse().unwrap());
    }

    (seeds, maps)
}

fn part_1(seeds: &[u64], maps: Vec<Map>) -> u64 {
    seeds
        .iter()
        .map(|seed| {
            let mut value = *seed;
            for map in &maps {
                value = map.map(value);
            }
            value
        })
        .min()
        .unwrap()
}

fn part_2(seeds: &[u64], maps: Vec<Map>) -> u64 {
    let seed_ranges: Vec<(_, _)> = seeds.iter().tuples::<(_, _)>().collect();
    seed_ranges
        .par_iter()
        .map(|(&start, &len)| {
            (start..=(start + len))
                .map(|seed| {
                    let mut value = seed;
                    for map in &maps {
                        value = map.map(value);
                    }
                    value
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4";

    #[test]
    fn test_1() {
        let (seeds, maps) = parse_input(TEST_INPUT);
        assert_eq!(part_1(&seeds, maps), 35);
    }

    #[test]
    fn test_2() {
        let (seeds, maps) = parse_input(TEST_INPUT);
        assert_eq!(part_2(&seeds, maps), 46);
    }
}
