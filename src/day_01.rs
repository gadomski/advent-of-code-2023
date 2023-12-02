//! Day 01

use anyhow::{anyhow, Error, Result};

const INPUT: &str = include_str!("../input/day_01.txt");
const DIGITS: [(&str, &str); 9] = [
    ("1", "one"),
    ("2", "two"),
    ("3", "three"),
    ("4", "four"),
    ("5", "five"),
    ("6", "six"),
    ("7", "seven"),
    ("8", "eight"),
    ("9", "nine"),
];

/// Part 1
///
/// # Examples
///
/// ```
/// assert_eq!(aoc::day_01::part_1().unwrap(), 57346);
/// ```
pub fn part_1() -> Result<i64> {
    let mut sum = 0;
    for line in INPUT.lines() {
        sum += calibration_value(line)?;
    }
    Ok(sum)
}

/// Part 2
///
/// # Examples
///
/// ```
/// assert_eq!(aoc::day_01::part_2().unwrap(), 57345);
/// ```
pub fn part_2() -> Result<i64> {
    let mut sum = 0;
    for line in INPUT.lines() {
        sum += calibration_value_with_words(line)?;
    }
    Ok(sum)
}

fn calibration_value(s: &str) -> Result<i64> {
    let first = first_digit(s).ok_or_else(|| anyhow!("no first digit: {}", s))?;
    let last = last_digit(s).ok_or_else(|| anyhow!("no last digit: {}", s))?;
    format!("{}{}", first, last).parse().map_err(Error::from)
}

fn calibration_value_with_words(s: &str) -> Result<i64> {
    let first = first_digit_with_words(s).ok_or_else(|| anyhow!("no first digit: {}", s))?;
    let last = last_digit_with_words(s).ok_or_else(|| anyhow!("no last digit: {}", s))?;
    format!("{}{}", first, last).parse().map_err(Error::from)
}

fn first_digit(s: &str) -> Option<char> {
    s.chars().find(|c| c.is_digit(10))
}

fn last_digit(s: &str) -> Option<char> {
    s.chars().rev().find(|c| c.is_digit(10))
}

fn first_digit_with_words(s: &str) -> Option<&'static str> {
    for (i, _) in s.char_indices() {
        for (a, b) in DIGITS {
            if s[i..].starts_with(a) || s[i..].starts_with(b) {
                return Some(a);
            }
        }
    }
    None
}

fn last_digit_with_words(s: &str) -> Option<&'static str> {
    for (i, _) in s.char_indices().rev() {
        for (a, b) in DIGITS {
            if s[i..].starts_with(a) || s[i..].starts_with(b) {
                return Some(a);
            }
        }
    }
    None
}

#[test]
fn part_1_example() {
    assert_eq!(calibration_value("1abc2").unwrap(), 12);
    assert_eq!(calibration_value("pqr3stu8vwx").unwrap(), 38);
    assert_eq!(calibration_value("a1b2c3d4e5f").unwrap(), 15);
    assert_eq!(calibration_value("treb7uchet").unwrap(), 77);
}

#[test]
fn part_1_check() {}

#[test]
fn part_2_example() {
    assert_eq!(calibration_value_with_words("two1nine").unwrap(), 29);
    assert_eq!(calibration_value_with_words("eightwothree").unwrap(), 83);
    assert_eq!(calibration_value_with_words("abcone2threexyz").unwrap(), 13);
    assert_eq!(calibration_value_with_words("xtwone3four").unwrap(), 24);
    assert_eq!(
        calibration_value_with_words("4nineeightseven2").unwrap(),
        42
    );
    assert_eq!(calibration_value_with_words("zoneight234").unwrap(), 14);
    assert_eq!(calibration_value_with_words("7pqrstsixteen").unwrap(), 76);
}
