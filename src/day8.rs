use std::collections::HashMap;

use advent::advent_of_code;
use advent_utils::*;

#[advent_of_code(day = 8, part = 1)]
fn part1(mut lines: impl Iterator<Item = String>) -> String {
    let key = lines.next().unwrap();
    lines.next();
    let map: HashMap<String, (String, String)> = lines
        .map(|line| {
            let (key, values) = line.split_once("=").unwrap();
            let (left, right) = values
                .trim()
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split_once(',')
                .unwrap();
            (
                key.trim().to_string(),
                (left.trim().to_string(), right.trim().to_string()),
            )
        })
        .collect();

    let mut loops = 0;
    let mut current_node = "AAA";
    loop {
        for c in key.bytes() {
            let (left, right) = map.get(current_node).unwrap();
            if c == b'L' {
                current_node = left;
            } else if c == b'R' {
                current_node = right;
            } else {
                panic!("unexpected character {c}");
            }
            loops += 1;
            if current_node == "ZZZ" {
                return loops.to_string();
            }
        }
    }
}

#[advent_of_code(day = 8, part = 2)]
fn part2(mut lines: impl Iterator<Item = String>) -> String {
    let key = lines.next().unwrap();
    lines.next();
    let map: HashMap<String, (String, String)> = lines
        .map(|line| {
            let (key, values) = line.split_once("=").unwrap();
            let (left, right) = values
                .trim()
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split_once(',')
                .unwrap();
            (
                key.trim().to_string(),
                (left.trim().to_string(), right.trim().to_string()),
            )
        })
        .collect();
    let nodes: Vec<&str> = map
        .keys()
        .filter(|key| key.ends_with('A'))
        .map(|s| s.as_str())
        .collect();
    let mut node_cycle = nodes.iter().map(|node| {
        let mut node = *node;
        let mut loops = 0usize;
        'outer: loop {
            for c in key.bytes() {
                let (left, right) = map.get(node).unwrap();
                if c == b'L' {
                    node = left.as_str();
                } else if c == b'R' {
                    node = right.as_str();
                } else {
                    panic!("unexpected character {c}");
                }

                loops += 1;
                // println!("{loops}");
                if node.ends_with('Z') {
                    break 'outer;
                }
            }
        }
        loops
    });
    let mut l = node_cycle.next().unwrap();
    for value in node_cycle {
        l = lcm(value, l);
    }
    return l.to_string();
}

fn lcm(l: usize, r: usize) -> usize {
    let mut x = l.max(r);
    let mut y = l.min(r);
    let mut rem = x % y;
    while rem != 0 {
        x = y;
        y = rem;
        rem = x % y;
    }
    l * r / y
}

#[cfg(test)]
mod test {
    #[test]
    fn sample_data() {
        let sample = "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)";
        assert_eq!(super::part1(sample.lines().map(|l| l.to_string())), "2");

        let sample_2 = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";
        assert_eq!(super::part1(sample_2.lines().map(|l| l.to_string())), "6");
    }

    #[test]
    fn two() {
        let two = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";
        assert_eq!(super::part2(two.lines().map(|l| l.to_string())), "6");
    }

    #[test]
    fn lcm() {
        assert_eq!(super::lcm(6, 36), 36);
    }
}
