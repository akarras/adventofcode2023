use std::{
    collections::HashMap,
    str::FromStr,
};

use advent::advent_of_code;
use advent_utils::*;

#[advent_of_code(day = 2, part = 1)]
pub fn day_2_part_1(lines: impl Iterator<Item = String>) -> String {
    let mut possibilities = HashMap::new();
    possibilities.insert(Color::Red, 12);
    possibilities.insert(Color::Blue, 14);
    possibilities.insert(Color::Green, 13);
    let day_2_data = lines.map(|game| Game::from_line(game.as_str()));
    let game_id_sum = possible_games(day_2_data, &possibilities)
        .map(|game| game.game_number)
        .sum::<u32>();
    game_id_sum.to_string()
}

#[advent_of_code(day = 2, part = 2)]
pub fn day_2_part_2(lines: impl Iterator<Item = String>) -> String {
    let day_2_data = lines.map(|game| Game::from_line(game.as_str()));
    let sum = day_2_data.map(min_round).map(power).sum::<u32>();
    sum.to_string()
}

fn power(round: Round) -> u32 {
    round
        .round_data
        .into_iter()
        .map(|(_, count)| count)
        .product()
}

fn min_round(game: Game) -> Round {
    let mut min_data = HashMap::new();
    for round in game.rounds {
        for (color, count) in round.round_data {
            let entry = min_data.entry(color).or_insert(count);
            let value = *entry;
            *entry = value.max(count);
        }
    }
    Round {
        round_data: min_data.into_iter().collect(),
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
enum Color {
    Blue,
    Red,
    Green,
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blue" => Ok(Color::Blue),
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            s => Err(s.to_string()),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Round {
    round_data: Vec<(Color, u32)>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Game {
    rounds: Vec<Round>,
    game_number: u32,
}

impl Game {
    fn from_line(line: &str) -> Self {
        // Game 1: [round]; [round];
        let (game_title, rounds) = line.split_once(':').unwrap();
        let game_number = game_title
            .trim()
            .split_once(' ')
            .unwrap()
            .1
            .parse::<u32>()
            .unwrap();
        Self {
            rounds: rounds
                .split(';')
                .map(|round| Round {
                    round_data: round
                        .split(',')
                        .map(|cube| {
                            cube.trim()
                                .split_once(' ')
                                .map(|(count, color)| {
                                    (color.parse().unwrap(), count.parse().unwrap())
                                })
                                .unwrap()
                        })
                        .collect(),
                })
                .collect(),
            game_number,
        }
    }
}

fn possible_games<'a>(
    games: impl Iterator<Item = Game> + 'a,
    possible: &'a HashMap<Color, u32>,
) -> impl Iterator<Item = Game> + 'a {
    games.filter(|game| {
        game.rounds.iter().all(|item| {
            item.round_data.iter().all(|(color, count)| {
                possible
                    .get(color)
                    .map(|max_count| *count <= *max_count)
                    .unwrap_or_default()
            })
        })
    })
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::day2::{possible_games, Game, Round};

    use super::Color;

    fn read_games<'a>(lines_iter: impl Iterator<Item = &'a str>) -> Vec<Game> {
        lines_iter.map(Game::from_line).collect()
    }

    #[test]
    fn parse_game_data() {
        let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let games = read_games(data.lines());
        let sample_game = vec![Game {
            rounds: vec![Round {
                round_data: vec![(Color::Blue, 3), (Color::Red, 4)],
            }],
            game_number: 1,
        }];
        assert_eq!(games[0].rounds[0], sample_game[0].rounds[0]);
        let mut possibilities = HashMap::new();
        possibilities.insert(Color::Red, 12);
        possibilities.insert(Color::Blue, 14);
        possibilities.insert(Color::Green, 13);
        assert_eq!(
            possible_games(games.into_iter(), &possibilities)
                .map(|game| game.game_number)
                .sum::<u32>(),
            8
        );
    }
}
