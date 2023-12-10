use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
    str::FromStr, cmp::{Reverse, Ordering},
};

use advent::advent_of_code;
use advent_utils::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Joker,
    Queen,
    King,
    Ace,
}

impl From<u8> for Card {
    fn from(value: u8) -> Self {
        match value {
            b'A' => Card::Ace,
            b'K' => Card::King,
            b'Q' => Card::Queen,
            b'J' => Card::Joker,
            b'T' => Card::Ten,
            b'9' => Card::Nine,
            b'8' => Card::Eight,
            b'7' => Card::Seven,
            b'6' => Card::Six,
            b'5' => Card::Five,
            b'4' => Card::Four,
            b'3' => Card::Three,
            b'2' => Card::Two,
            e => panic!("unexpected value {e}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Copy, Clone, Debug)]
struct Hand {
    cards: [Card; 5],
    bet: u32,
}

impl Hand {
    fn kind(&self) -> HandKind {
        let mut cards = self.cards.clone();
        cards.sort();

        let mut matches = cards.iter().count_distinct().collect::<Vec<_>>();
        matches.sort_by_key(|(i, _)| Reverse(*i));
        let mut matches = matches.iter();
        let (count, _card) = *matches.next().unwrap();
        let second = matches.next();
        match count {
            5 => HandKind::FiveOfAKind,
            4 => HandKind::FourOfAKind,
            3 => {
                if second.unwrap().0 == 2 {
                    HandKind::FullHouse
                } else {
                    HandKind::ThreeOfAKind
                }
            }
            2 => HandKind::TwoPair,
            1 => HandKind::HighCard,
            _ => unreachable!("???"),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.bet == other.bet
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind().cmp(&other.kind()) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            o => o
        }
    }
}

impl FromStr for Hand {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bet) = s.split_once(' ').unwrap();
        assert_eq!(hand.as_bytes().len(), 5);
        let bytes: [u8; 5] = hand.as_bytes().try_into().unwrap();
        let cards = bytes.map(|c| Card::from(c));
        Ok(Self {
            cards,
            bet: bet.parse()?,
        })
    }
}

fn from_lines(lines: impl Iterator<Item = String>) -> impl Iterator<Item = Hand> {
    lines.map(|line| Hand::from_str(line.trim()).unwrap())
}

#[advent_of_code(day = 7, part = 1)]
fn part_1(lines: impl Iterator<Item = String>) -> String {
    let mut hands = from_lines(lines).collect::<Vec<_>>();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, value)| value.bet * (i + 1) as u32)
        .sum::<u32>()
        .to_string()
}

// #[advent_of_code(day = 7, part = 2)]
// fn part_2(lines: impl Iterator<Item = String>) -> String {

// }

#[cfg(test)]
mod test {
    use std::{cmp::Reverse, str::FromStr};

    use super::{Hand, Card};

    #[test]
    fn part_1() {
        let sample_data = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";
        assert_eq!(
            super::part_1(sample_data.lines().map(|l| l.to_string())),
            "6440".to_string()
        );
    }

    #[test]
    fn test_order() {
        let mut order = vec![Hand {
            cards: b"33332".map(|h| Card::from(h)),
            bet: 2,
        },
        Hand {
            cards: b"2AAAA".map(|h| Card::from(h)),
            bet: 1,
        }
        ];
        order.sort();
        assert_eq!(order[0].bet, 1);
    }

    #[test]
    fn blah() {
        let mut hand = vec![Hand::from_str("2AAAA 430").unwrap(), Hand::from_str("A2222 40").unwrap(), Hand::from_str("5555A 41").unwrap(), Hand::from_str("8888A 41").unwrap()];
        hand.sort();
        assert_eq!(hand[3].bet, 40);
    }
}
