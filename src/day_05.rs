//! Day 05

use anyhow::{anyhow, Error, Result};
use std::str::FromStr;

const INPUT: &str = include_str!("../input/day_05.txt");

/// Part 1
///
/// # Examples
///
/// ```
/// assert_eq!(aoc::day_05::part_1().unwrap(), 318728750);
/// ```
pub fn part_1() -> Result<i64> {
    lowest_location_number(INPUT)
}

/// Part 2
pub fn part_2() -> Result<i64> {
    lowest_location_number_with_ranges(INPUT)
}

fn lowest_location_number(s: &str) -> Result<i64> {
    let almanac: Almanac = s.parse()?;
    let mut lowest_location_number = i64::MAX;
    for seed in &almanac.seeds {
        let location_number = almanac.location_for_seed(*seed);
        if location_number < lowest_location_number {
            lowest_location_number = location_number;
        }
    }
    Ok(lowest_location_number)
}

fn lowest_location_number_with_ranges(s: &str) -> Result<i64> {
    let almanac: Almanac = s.parse()?;
    for i in 0.. {
        let seed = almanac.seed_for_location(i);
        for chunk in almanac.seeds.chunks_exact(2) {
            if seed >= chunk[0] && seed < chunk[0] + chunk[1] {
                return Ok(i);
            }
        }
    }
    unreachable!()
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Map>,
}

#[derive(Debug)]
struct Map(Vec<Range>);

#[derive(Debug)]
struct Range {
    source_start: i64,
    destination_start: i64,
    length: i64,
}

impl Almanac {
    fn location_for_seed(&self, seed: i64) -> i64 {
        let mut value = seed;
        for map in &self.maps {
            value = map.transform(value);
        }
        value
    }

    fn seed_for_location(&self, location: i64) -> i64 {
        let mut value = location;
        for map in self.maps.iter().rev() {
            value = map.reverse_transform(value);
        }
        value
    }
}

impl Map {
    fn transform(&self, value: i64) -> i64 {
        for range in &self.0 {
            if let Some(value) = range.transform(value) {
                return value;
            }
        }
        value
    }

    fn reverse_transform(&self, value: i64) -> i64 {
        for range in &self.0 {
            if let Some(value) = range.reverse_transform(value) {
                return value;
            }
        }
        value
    }
}

impl Range {
    fn transform(&self, value: i64) -> Option<i64> {
        let diff = value - self.source_start;
        if diff >= 0 && diff < self.length {
            Some(self.destination_start + diff)
        } else {
            None
        }
    }

    fn reverse_transform(&self, value: i64) -> Option<i64> {
        let diff = value - self.destination_start;
        if diff >= 0 && diff < self.length {
            Some(self.source_start + diff)
        } else {
            None
        }
    }
}

impl FromStr for Almanac {
    type Err = Error;

    fn from_str(s: &str) -> Result<Almanac> {
        let mut iter = s.split("\n\n");
        let mut seeds = crate::iter::next(&mut iter)?.split_whitespace();
        if crate::iter::next(&mut seeds)? != "seeds:" {
            return Err(anyhow!("expected 'seeds:' at start of first line: {}", s));
        }
        let seeds = seeds
            .map(|seed| seed.parse::<i64>().map_err(Error::from))
            .collect::<Result<Vec<_>>>()?;
        let maps = iter
            .map(|map| map.parse::<Map>())
            .collect::<Result<Vec<_>>>()?;
        Ok(Almanac { seeds, maps })
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Map> {
        let mut lines = s.lines();
        if !crate::iter::next(&mut lines)?.ends_with("map:") {
            return Err(anyhow!(
                "expected first line of map to end with 'map:': {}",
                s
            ));
        }
        let ranges = lines
            .map(|line| line.parse::<Range>())
            .collect::<Result<Vec<_>>>()?;
        Ok(Map(ranges))
    }
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Range> {
        let mut iter = s.split_whitespace();
        let destination_start = crate::iter::next(&mut iter)?.parse()?;
        let source_start = crate::iter::next(&mut iter)?.parse()?;
        let length = crate::iter::next(&mut iter)?.parse()?;
        if iter.next().is_some() {
            Err(anyhow!("invalid range: {}", s))
        } else {
            Ok(Range {
                destination_start,
                source_start,
                length,
            })
        }
    }
}

#[test]
fn part_1_example() {
    let input = "seeds: 79 14 55 13

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
    let almanac: Almanac = input.parse().unwrap();
    assert_eq!(almanac.location_for_seed(79), 82);
    assert_eq!(almanac.location_for_seed(14), 43);
    assert_eq!(almanac.location_for_seed(55), 86);
    assert_eq!(almanac.location_for_seed(13), 35);
    assert_eq!(lowest_location_number(input).unwrap(), 35);
}

#[test]
fn part_2_example() {
    let input = "seeds: 79 14 55 13

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
    assert_eq!(lowest_location_number_with_ranges(input).unwrap(), 46);
}
