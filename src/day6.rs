use std::{ops::Neg, time::Instant};

use crate::utils::*;

fn read_races(mut iter: impl Iterator<Item = String>) -> Vec<RaceData> {
    let time = iter.next().unwrap();
    let distance = iter.next().unwrap();
    let time = time.expect_tag("Time");
    let times = time.read_delimited::<u64>(" ");
    let distances = distance.expect_tag("Distance");
    let distances = distances.read_delimited::<u64>(" ");
    times
        .zip(distances)
        .map(|(time, distance)| RaceData {
            race_duration: time,
            distance_to_beat: distance,
        })
        .collect()
}

fn read_big_race(mut iter: impl Iterator<Item = String>) -> RaceData {
    let time = iter.next().unwrap();
    let distance = iter.next().unwrap();
    let time = time.expect_tag("Time");
    let time = time.replace(" ", "");
    let distances = distance.expect_tag("Distance");
    let distance = distances.replace(" ", "");
    RaceData {
        race_duration: time.parse().unwrap(),
        distance_to_beat: distance.parse().unwrap(),
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct RaceData {
    race_duration: u64,
    distance_to_beat: u64,
}

fn quad_formula(a: f64, b: f64, c: f64) -> (f64, f64) {
    let pos = (b.neg() + f64::sqrt(b.powi(2) - (4.0 * a * c))) / (2.0 * a);
    let neg = (b.neg() - f64::sqrt(b.powi(2) - (4.0 * a * c))) / (2.0 * a);
    (neg, pos)
}

impl RaceData {
    // returns the edges of where holding the button causes you to beat the record
    // fn find_wins(&self) -> impl Iterator<Item = u64> + '_ {
    //     (1..self.race_duration)
    //         .into_iter()
    //         .flat_map(|time_pressed| self.is_win(time_pressed))
    // }

    fn find_num_wins(&self) -> u64 {
        let (start, end) = quad_formula(
            1.0,
            (self.race_duration as f64).neg(),
            self.distance_to_beat as f64,
        );

        let offset = if start.fract() == 0.0 { 1 } else { 0 };
        (end - start).round() as u64 - offset
    }

    // returns the distance traveled if you press the button for the given time
    // returns None if you wouldn't win the race
    // fn is_win(&self, time_pressed: u64) -> Option<u64> {
    //     let Self {
    //         race_duration,
    //         distance_to_beat,
    //     } = *self;
    //     let travel_time = race_duration.checked_sub(time_pressed)?;
    //     let distance_traveled = time_pressed * travel_time;
    //     let win = distance_traveled > distance_to_beat;
    //     println!("{win} {time_pressed}");
    //     win.then(|| distance_traveled)
    // }
}

pub fn day_6_part_1() {
    let lines = read_file("./test_data/day_6");
    let data = read_races(lines);
    let product_of_wins = data
        .into_iter()
        .map(|race| race.find_num_wins() as u64)
        .product::<u64>();
    println!("{}", product_of_wins);
}

pub fn day_6_part_2() {
    let lines = read_file("./test_data/day_6");
    let data = read_big_race(lines);
    let time = Instant::now();
    let ways = data.find_num_wins();
    println!("{:?}", time.elapsed());
    println!("{ways}");
}

#[cfg(test)]
mod test {
    use crate::day6::{read_races, RaceData};

    #[test]
    fn sample_data() {
        let data = "Time:      7  15   30
        Distance:  9  40  200";
        let races = read_races(data.lines().map(|l| l.to_string()));
        assert_eq!(
            races,
            vec![
                RaceData {
                    race_duration: 7,
                    distance_to_beat: 9
                },
                RaceData {
                    race_duration: 15,
                    distance_to_beat: 40
                },
                RaceData {
                    race_duration: 30,
                    distance_to_beat: 200
                }
            ]
        );
        assert_eq!(
            races
                .into_iter()
                .map(|race| race.find_num_wins())
                .inspect(|count| println!("{count}"))
                .product::<u64>(),
            288
        );
    }
}
