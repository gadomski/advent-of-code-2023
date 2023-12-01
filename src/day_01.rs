use std::collections::HashMap;

use anyhow::{anyhow, Error, Result};

const INPUT: &str = include_str!("../input/day_01.txt");

pub fn part_1() -> Result<i64> {
    let mut sum = 0;
    for line in INPUT.lines() {
        sum += calibration_value(line)?;
    }
    Ok(sum)
}

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
    let mut lookup = HashMap::with_capacity(18);
    lookup.insert("1", "1");
    lookup.insert("one", "1");
    lookup.insert("2", "2");
    lookup.insert("two", "2");
    lookup.insert("3", "3");
    lookup.insert("three", "3");
    lookup.insert("4", "4");
    lookup.insert("four", "4");
    lookup.insert("5", "5");
    lookup.insert("five", "5");
    lookup.insert("6", "6");
    lookup.insert("six", "6");
    lookup.insert("7", "7");
    lookup.insert("seven", "7");
    lookup.insert("8", "8");
    lookup.insert("eight", "8");
    lookup.insert("9", "9");
    lookup.insert("nine", "9");
    let first =
        first_digit_with_words(s, &lookup).ok_or_else(|| anyhow!("no first digit: {}", s))?;
    let last = last_digit_with_words(s, &lookup).ok_or_else(|| anyhow!("no last digit: {}", s))?;
    format!("{}{}", first, last).parse().map_err(Error::from)
}

fn first_digit(s: &str) -> Option<char> {
    s.chars().find(|c| c.is_digit(10))
}

fn last_digit(s: &str) -> Option<char> {
    s.chars().rev().find(|c| c.is_digit(10))
}

fn first_digit_with_words(
    s: &str,
    lookup: &HashMap<&'static str, &'static str>,
) -> Option<&'static str> {
    for (i, _) in s.char_indices() {
        for (key, value) in lookup {
            if s[i..].starts_with(key) {
                return Some(value);
            }
        }
    }
    None
}

fn last_digit_with_words(
    s: &str,
    lookup: &HashMap<&'static str, &'static str>,
) -> Option<&'static str> {
    for (i, _) in s.char_indices().rev() {
        for (key, value) in lookup {
            if s[i..].starts_with(key) {
                return Some(value);
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
fn part_1_check() {
    assert_eq!(part_1().unwrap(), 57346);
}

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

#[test]
fn part_2_check() {
    assert_eq!(part_2().unwrap(), 57345);
}
