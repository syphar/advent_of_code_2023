pub(crate) enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(ch: char) -> Self {
        match ch {
            'L' => Direction::Left,
            'R' => Direction::Right,
            ch => panic!("unknown direction: {}", ch),
        }
    }
}
