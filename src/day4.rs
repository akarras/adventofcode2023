use std::collections::HashSet;

use advent::advent_of_code;
use advent_utils::*;

#[advent_of_code(day = 4, part = 1)]
pub fn day_4_part_1(lines: impl Iterator<Item = String>) -> String {
    let sum = lines
        .into_iter()
        .map(|line| Card::parse_line(&line))
        .map(|card| card.score())
        .sum::<u32>();
    sum.to_string()
}

#[advent_of_code(day = 4, part = 2)]
pub fn day_4_part_2(lines: impl Iterator<Item = String>) -> String {
    let cards = lines
        .into_iter()
        .map(|line| Card::parse_line(&line))
        .collect::<Vec<_>>();
    let sum = process_pile(&cards);
    sum.to_string()
}

fn process_pile(cards: &Vec<Card>) -> u32 {
    let mut counts = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let matches = card.num_matching();
        for score in 1..=matches {
            counts[i + score as usize] += counts[i];
        }
    }
    counts.iter().sum()
}

struct Card {
    // number: i32,
    winners: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

fn parse_numbers(nums: &str) -> HashSet<u32> {
    nums.split(' ')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect()
}

impl Card {
    fn parse_line(line: &str) -> Card {
        let (_card, rest) = line.split_once(':').unwrap();
        // let number = card.split_once(" ").unwrap().1.parse().unwrap();
        let (my_numbers, winners) = rest.split_once('|').unwrap();
        Self {
            // number,
            winners: parse_numbers(my_numbers),
            my_numbers: parse_numbers(winners),
        }
    }

    fn num_matching(&self) -> u32 {
        self.my_numbers
            .iter()
            .filter(|my_number| self.winners.contains(my_number))
            .count() as u32
    }

    fn score(&self) -> u32 {
        let num_cards = self.num_matching();
        if num_cards >= 1 {
            u32::pow(2, num_cards - 1)
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day4::process_pile;

    use super::Card;

    #[test]
    fn sample() {
        let sample = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let cards = sample
            .lines()
            .map(|line| Card::parse_line(line))
            .collect::<Vec<_>>();
        assert_eq!(cards[1].score(), 2);
        let sum = cards.iter().map(|card| card.score()).sum::<u32>();
        assert_eq!(sum, 13);
        let score = process_pile(&cards);
        assert_eq!(score, 30)
    }
}
