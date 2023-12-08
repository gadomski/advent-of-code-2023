//! Day 08

use anyhow::{anyhow, Result};
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day_08.txt");

/// Part 1
///
/// # Examples
///
/// ```
/// assert_eq!(aoc::day_08::part_1().unwrap(), 20569);
/// ```
pub fn part_1() -> Result<usize> {
    steps_to_reach_zzz(INPUT)
}

/// Part 2
///
/// # Examples
///
/// ```
/// assert_eq!(aoc::day_08::part_2().unwrap(), 21366921060721);
/// ```
pub fn part_2() -> Result<usize> {
    steps_to_reach_all_zs(INPUT)
}

fn steps_to_reach_zzz(s: &str) -> Result<usize> {
    let (instructions, map) = parse(s)?;
    simulate(instructions, &map, "AAA", |location| location == "ZZZ")
}

fn simulate(
    instructions: &str,
    map: &HashMap<&str, (&str, &str)>,
    start: &str,
    test: fn(&str) -> bool,
) -> Result<usize> {
    let mut location = start;
    let mut steps = 0;
    for instruction in instructions.chars().cycle() {
        let (left, right) = map
            .get(location)
            .ok_or_else(|| anyhow!("invalid location: {}", location))?;
        match instruction {
            'L' => location = left,
            'R' => location = right,
            _ => return Err(anyhow!("invalid instruction: {}", instruction)),
        }
        steps += 1;
        if test(location) {
            break;
        }
    }
    Ok(steps)
}

fn steps_to_reach_all_zs(s: &str) -> Result<usize> {
    let (instructions, map) = parse(s)?;
    let mut steps = Vec::new();
    for location in map.keys().filter(|key| key.ends_with('A')) {
        steps.push(simulate(instructions, &map, location, |location| {
            location.ends_with('Z')
        })?);
    }
    Ok(steps.into_iter().fold(1, num_integer::lcm))
}

fn parse(s: &str) -> Result<(&str, HashMap<&str, (&str, &str)>)> {
    let (instructions, s) = s
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("invalid string, no double newline: {}", s))?;
    let mut map = HashMap::new();
    for line in s.lines() {
        let (key, values) = line
            .split_once(" = ")
            .ok_or_else(|| anyhow!("invalid line: {}", line))?;
        if !(values.starts_with('(') && values.ends_with(')')) {
            return Err(anyhow!("invalid values: {}", values));
        }
        let (left, right) = values[1..values.len() - 1]
            .split_once(", ")
            .ok_or_else(|| anyhow!("invalid values: {}", values))?;
        let _ = map.insert(key, (left, right));
    }
    Ok((instructions, map))
}

#[test]
fn part_1_examples() {
    let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    assert_eq!(steps_to_reach_zzz(input).unwrap(), 2);

    let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    assert_eq!(steps_to_reach_zzz(input).unwrap(), 6);
}

#[test]
fn part_2_example() {
    let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    assert_eq!(steps_to_reach_all_zs(input).unwrap(), 6);
}
