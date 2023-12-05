use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Mapping {
    destination_start: u64,
    source_start: u64,
    len: u64,
}

impl FromStr for Mapping {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pieces: Vec<_> = s.split_whitespace().collect();
        assert_eq!(pieces.len(), 3);

        Ok(Mapping {
            destination_start: pieces[0].parse().unwrap(),
            source_start: pieces[1].parse().unwrap(),
            len: pieces[2].parse().unwrap(),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Map(Vec<Mapping>);

impl Map {
    pub(crate) fn map(&self, number: u64) -> u64 {
        for mapping in &self.0 {
            if number >= mapping.source_start && number <= mapping.source_start + mapping.len {
                return mapping.destination_start + (number - mapping.source_start);
            }
        }
        number
    }
}

impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Map(s
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.parse().unwrap())
            .collect()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn parse() {
        let map: Map = "
            50 98 2
            52 50 48
            "
        .parse()
        .unwrap();

        assert_eq!(
            map.0,
            vec![
                Mapping {
                    destination_start: 50,
                    source_start: 98,
                    len: 2,
                },
                Mapping {
                    destination_start: 52,
                    source_start: 50,
                    len: 48,
                },
            ],
        )
    }

    #[test_case(98, 50)]
    #[test_case(99, 51)]
    #[test_case(50, 52)]
    #[test_case(53, 55)]
    #[test_case(97, 99)]
    #[test_case(0, 0)]
    #[test_case(1, 1)]
    fn map(input: u64, output: u64) {
        let map = Map(vec![
            Mapping {
                destination_start: 50,
                source_start: 98,
                len: 2,
            },
            Mapping {
                destination_start: 52,
                source_start: 50,
                len: 48,
            },
        ]);
        assert_eq!(map.map(input), output);
    }
}
