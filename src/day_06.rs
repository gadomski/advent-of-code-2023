//! Day 06

use anyhow::{anyhow, Result};

const INPUT: &str = include_str!("../input/day_06.txt");

/// Part 1
///
/// # Examples
///
/// ```
/// assert_eq!(aoc::day_06::part_1().unwrap(), 293046);
/// ```
pub fn part_1() -> Result<i64> {
    number_of_ways_to_beat_the_records(INPUT)
}

/// Part 1
///
/// # Examples
///
/// ```
///  assert_eq!(aoc::day_06::part_1().unwrap(), 35150181);
/// ```
pub fn part_2() -> Result<i64> {
    Ok(number_of_ways_to_beat_the_record(61709066, 643118413621041))
}

fn number_of_ways_to_beat_the_records(s: &str) -> Result<i64> {
    let mut lines = s.lines();
    let mut times = crate::iter::next(&mut lines)?.split_whitespace();
    if crate::iter::next(&mut times)? != "Time:" {
        return Err(anyhow!("invalid time string: {}", s));
    }
    let mut distances = crate::iter::next(&mut lines)?.split_whitespace();
    if crate::iter::next(&mut distances)? != "Distance:" {
        return Err(anyhow!("invalid distance string: {}", s));
    }
    if lines.next().is_some() {
        return Err(anyhow!("invalid string: {}", s));
    }
    let mut product = 1;
    for (time, distance) in times.zip(distances) {
        let time: i64 = time.parse()?;
        let distance: i64 = distance.parse()?;
        product *= number_of_ways_to_beat_the_record(time, distance);
    }
    Ok(product)
}

fn number_of_ways_to_beat_the_record(time: i64, distance: i64) -> i64 {
    let mut start = 0;
    let mut end = 0;
    for button_press_duration in 1..time {
        if button_press_duration * (time - button_press_duration) > distance {
            start = button_press_duration;
            break;
        }
    }
    assert_ne!(start, 0);
    for button_press_duration in (1..time).rev() {
        if button_press_duration * (time - button_press_duration) > distance {
            end = button_press_duration;
            break;
        }
    }
    assert_ne!(end, 0);
    end - start + 1
}

#[test]
fn part_1_example() {
    let input = "Time:      7  15   30
Distance:  9  40  200";
    assert_eq!(number_of_ways_to_beat_the_records(input).unwrap(), 288);
}

#[test]
fn part_2_example() {
    assert_eq!(number_of_ways_to_beat_the_record(71530, 940200), 71503);
}
