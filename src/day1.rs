use advent::advent_of_code;
use advent_utils::*;

const POSSIBLE_DIGITS: &[(&str, u32)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

struct Reversed(String);

impl Reversed {
    fn new(str: &str) -> Self {
        Self(str.chars().rev().collect())
    }
}

fn find_digit_reverse(input: Reversed) -> u32 {
    if let Some(char) = input.0.chars().next() {
        if let Some(digit) = char.to_digit(10) {
            return digit;
        }
    }
    for (digit, value) in POSSIBLE_DIGITS {
        if let Some(cmp_str) = input.0.get(0..digit.len()) {
            if Reversed::new(digit).0 == cmp_str {
                return *value;
            }
        }
    }
    if let Some(remainder) = input.0.get(1..) {
        find_digit_reverse(Reversed(remainder.to_string()))
    } else {
        panic!("unable to find number")
    }
}

fn find_digit(input: &str) -> u32 {
    if let Some(char) = input.chars().next() {
        if let Some(digit) = char.to_digit(10) {
            return digit;
        }
    }
    for (digit, value) in POSSIBLE_DIGITS {
        if let Some(cmp_str) = input.get(0..digit.len()) {
            if *digit == cmp_str {
                return *value;
            }
        }
    }
    if let Some(remainder) = input.get(1..) {
        find_digit(remainder)
    } else {
        panic!("unable to find number")
    }
}

#[advent_of_code(day = 1, part = 1)]
pub fn day_1_part_1(lines: impl Iterator<Item = String>) -> String {
    let value = lines
        .map(|line| find_calibration_value_simple(line.as_str()))
        .sum::<u32>();
    value.to_string()
}

#[advent_of_code(day = 1, part = 2)]
fn day_1_part_2(lines: impl Iterator<Item = String>) -> String {
    let value = lines
        .map(|line| find_calibration_value_text(line.as_str()))
        .sum::<u32>();
    value.to_string()
}

fn find_calibration_value_simple(line: &str) -> u32 {
    let first = line
        .chars()
        .flat_map(|digit| digit.to_digit(10))
        .next()
        .unwrap();
    let last = line
        .chars()
        .flat_map(|digit| digit.to_digit(10))
        .last()
        .unwrap();
    first * 10 + last
}

fn find_calibration_value_text(line: &str) -> u32 {
    let first = find_digit(line);
    let last = find_digit_reverse(Reversed::new(line));
    first * 10 + last
}

#[cfg(test)]
mod test {
    use super::{find_calibration_value_simple, find_calibration_value_text};

    #[test]
    fn part_1_sample() {
        let data = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let result = data
            .into_iter()
            .map(find_calibration_value_simple)
            .sum::<u32>();
        assert_eq!(result, 142);
    }

    #[test]
    fn part_2_sample() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        let values = input
            .lines()
            .map(find_calibration_value_text)
            .collect::<Vec<_>>();
        assert_eq!(values, vec![29, 83, 13, 24, 42, 14, 76]);
        assert_eq!(values.iter().sum::<u32>(), 281);
    }
}
