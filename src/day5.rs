use crate::utils::{read_file, IterExt, ParseExt};

fn read_day_5_data() -> Data {
    let lines = read_file("./test_data/day_5");
    Data::read_data(lines)
}

pub fn day_5_part_1() {
    let smallest = read_day_5_data().map_seeds().into_iter().min().unwrap();
    println!("part 1: {smallest}");
}

pub fn day_5_part_2() {
    let smallest = read_day_5_data()
        .map_seed_ranges()
        .into_iter()
        .min()
        .unwrap();
    println!("part 2: {smallest}");
}

struct Map {
    // incoming: String,
    // outgoing: String,
    maps: Vec<MappedRange>,
}

#[derive(Debug)]
struct MappedRange {
    start: u64,
    dest_value: u64,
    length: u64,
}

impl MappedRange {
    fn try_map(&self, value: u64) -> Option<u64> {
        let range = self.start..(self.start + self.length);
        range.contains(&value).then(|| {
            // println!("{}-{} {:?}", self.start, value, range);
            (value - self.start) + self.dest_value
        })
    }
}

impl Map {
    fn read<I>(mut data: I) -> (I, Self)
    where
        I: Iterator<Item = String>,
    {
        let header = data.next().unwrap();
        let (prefix, postfix) = header.trim().split_once(" ").unwrap();
        assert_eq!(postfix, "map:");
        let mut header_parts = prefix.split("-");
        let _incoming = header_parts.next().unwrap().to_owned();
        let _to = header_parts.next();
        let _outgoing = header_parts.next().unwrap().to_owned();
        let mut maps = Vec::new();
        while let Some(data) = data.next() {
            if data.trim().is_empty() {
                break;
            }
            let mut numbers = data.read_delimited(" ");
            maps.push(MappedRange {
                dest_value: numbers.next().unwrap(),
                start: numbers.next().unwrap(),
                length: numbers.next().unwrap(),
            });
        }
        (
            data,
            Map {
                // incoming,
                // outgoing,
                maps,
            },
        )
    }

    fn map_value(&self, value: u64) -> u64 {
        let outgoing = self
            .maps
            .iter()
            .find_map(|v| v.try_map(value))
            .unwrap_or(value);
        // println!("{}->{}: {}->{}", self.incoming, self.outgoing, value, outgoing);
        outgoing
    }
}

struct Data {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}
impl Data {
    fn read_data(mut data: impl Iterator<Item = String>) -> Data {
        let seeds = data.next().unwrap();
        let (header, seeds) = seeds.split_once(":").unwrap();
        assert_eq!(header, "seeds");
        let seeds = seeds.read_delimited(" ");
        let _ = data.next().unwrap();
        let mut maps = vec![];
        for _ in 0..7 {
            let (rest, map) = Map::read(data);
            maps.push(map);
            data = rest;
        }
        Data {
            seeds: seeds.collect(),
            maps,
        }
    }

    fn map_seed(&self, seed: u64) -> u64 {
        self.maps.iter().fold(seed, |seed, map| map.map_value(seed))
    }

    fn map_seeds(&self) -> impl Iterator<Item = u64> + '_ {
        self.seeds.iter().copied().map(|seed| self.map_seed(seed))
    }

    fn map_seed_ranges(&self) -> impl Iterator<Item = u64> + '_ {
        self.seeds
            .iter()
            .copied()
            .tuple_pairs()
            .flat_map(|(range_start, seed_length)| {
                (range_start..=(range_start + seed_length))
                    .into_iter()
                    .map(|seed| self.map_seed(seed))
            })
    }
}

#[cfg(test)]
mod test {
    use std::io::{BufRead, BufReader, Cursor};

    use super::Data;

    #[test]
    fn sample_data() {
        let sample_data = "seeds: 79 14 55 13

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
        let reader = BufReader::new(Cursor::new(sample_data));
        let data = Data::read_data(reader.lines().map(|l| l.unwrap()));
        let mapped = data.map_seeds().collect::<Vec<_>>();
        assert_eq!(mapped, vec![82, 43, 86, 35]);
        let smallest_pt_2 = data.map_seed_ranges().min().unwrap();
        assert_eq!(smallest_pt_2, 46);
    }
}
