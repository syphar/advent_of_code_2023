use itertools::Itertools;
use std::hash::Hash;
use std::{cmp::Ordering, str::FromStr};

// static CARDS: &[char] = &[
//     'A', 'K', 'Q', 'J', T', '9', '8', '7', '6', '5', '4', '3', '2',
// ];
static CARDS: &[char] = &[
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

#[derive(Debug, Eq, Clone, Copy)]
pub(crate) struct Card(char);

impl Hash for Card {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<char> for Card {
    fn from(ch: char) -> Self {
        assert!(CARDS.iter().any(|&c| c == ch));
        Self(ch)
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.chars().next().expect("empty string").into())
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        let this_idx = CARDS
            .iter()
            .position(|&c| c == self.0)
            .expect("unknown char");
        let other_idx = CARDS
            .iter()
            .position(|&c| c == other.0)
            .expect("unknown char");

        other_idx.cmp(&this_idx)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) struct Hand([Card; 5], [Card; 5]);

impl Hand {
    fn counts(&self) -> Vec<(Card, usize)> {
        self.0
            .iter()
            .copied()
            .counts()
            .iter()
            .sorted_by_key(|(_, &count)| count)
            .rev()
            .map(|(&card, &count)| (card, count))
            .collect()
    }

    fn is_five_of_a_kind(&self) -> bool {
        self.counts()[0].1 == 5
    }

    fn is_four_of_a_kind(&self) -> bool {
        self.counts()[0].1 == 4
    }
    fn is_three_of_a_kind(&self) -> bool {
        self.counts()[0].1 == 3
    }

    fn is_full_house(&self) -> bool {
        let counts = self.counts();

        counts[0].1 == 3 && counts[1].1 == 2
    }

    fn is_two_pairs(&self) -> bool {
        let counts = self.counts();

        counts[0].1 == 2 && counts[1].1 == 2
    }

    fn is_one_pair(&self) -> bool {
        let counts = self.counts();
        counts[0].1 == 2
    }

    fn order_value(&self) -> u8 {
        if self.is_five_of_a_kind() {
            6
        } else if self.is_four_of_a_kind() {
            5
        } else if self.is_full_house() {
            4
        } else if self.is_three_of_a_kind() {
            3
        } else if self.is_two_pairs() {
            2
        } else if self.is_one_pair() {
            1
        } else {
            0
        }
    }

    fn compare_cards(&self, other: &Self) -> Ordering {
        if self.1 == other.1 {
            return Ordering::Equal;
        }
        for (lhs, rhs) in self.1.iter().zip(other.1.iter()) {
            if lhs != rhs {
                return lhs.cmp(rhs);
            }
        }
        unreachable!();
    }

    pub(crate) fn upgrade(&self) -> Self {
        let mut new_hand = self.clone();

        if let Some(pos) = new_hand.0.iter().position(|ch| ch.0 == 'J') {
            for new_ch in CARDS.iter().copied() {
                let mut potential_new_hand = self.clone();
                potential_new_hand.0[pos].0 = new_ch;
                if potential_new_hand.order_value() > new_hand.order_value() {
                    new_hand = potential_new_hand.upgrade();
                }
            }
        }

        new_hand
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s.chars().map(Card::from).collect_vec().try_into().unwrap();
        Ok(Self(cards, cards))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.order_value().cmp(&other.order_value()) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.compare_cards(other),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn card_ordering() {
        for (&lhs, &rhs) in CARDS.iter().tuple_windows() {
            let a: Card = lhs.into();
            let b: Card = rhs.into();
            println!("{} {}", a.0, b.0);
            assert!(a > b);
            assert!(b < a);
        }
    }

    #[test_case("2AAAA")]
    #[test_case("33332")]
    fn is_four_of_a_kind(s: &str) {
        let hand: Hand = s.parse().unwrap();
        assert!(hand.is_four_of_a_kind());
    }

    #[test_case("2AAAA", "33332")]
    #[test_case("77788", "77888")]
    #[test_case("32T3K", "KK677")]
    #[test_case("32T3K", "KTJJT")]
    #[test_case("32T3K", "T55J5")]
    #[test_case("32T3K", "QQQJA")]
    #[test_case("KTJJT", "KK677")]
    #[test_case("KTJJT", "T55J5")]
    #[test_case("KTJJT", "QQQJA")]
    #[test_case("KK677", "T55J5")]
    #[test_case("KK677", "QQQJA")]
    #[test_case("T55J5", "QQQJA")]
    #[test_case("JKKK2", "QQQQ2")]
    fn hand_ordering(lhs: &str, rhs: &str) {
        let lhs: Hand = lhs.parse().unwrap();
        let rhs: Hand = rhs.parse().unwrap();
        println!("{:?} {:?}", lhs.order_value(), rhs.order_value());
        assert!(lhs < rhs);
    }

    #[test_case("32T3K", "32T3K")]
    #[test_case("KK677", "KK677")]
    #[test_case("T55J5", "T5555")]
    #[test_case("KTJJT", "KTTTT")]
    #[test_case("QQQJA", "QQQQA")]
    fn test_upgrade(lhs: &str, rhs: &str) {
        let lhs: Hand = lhs.parse().unwrap();
        let rhs: Hand = rhs.parse().unwrap();
        assert_eq!(lhs.upgrade().0, rhs.0);
    }
}
