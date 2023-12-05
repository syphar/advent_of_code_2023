use std::{collections::HashSet, str::FromStr};

#[derive(Clone, Debug)]
pub(crate) struct Card {
    pub(crate) number: u64,
    pub(crate) winners: HashSet<u64>,
    pub(crate) mine: HashSet<u64>,
    pub(crate) handled: bool,
}

impl Card {
    pub(crate) fn wins(&self) -> u64 {
        self.winners.intersection(&self.mine).count() as u64
    }
}

impl FromStr for Card {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card, numbers) = s.trim().split_once(':').unwrap();
        let (_, number) = card.trim().split_once(' ').unwrap();
        let (winners, mine) = numbers.split_once('|').unwrap();

        Ok(Card {
            number: number.trim().parse().unwrap(),
            winners: winners
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
            mine: mine
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
            handled: false,
        })
    }
}
