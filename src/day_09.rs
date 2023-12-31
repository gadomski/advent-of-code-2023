//! Day 09

use anyhow::{Error, Result};

const INPUT: &str = include_str!("../input/day_09.txt");

/// Part 1
///
/// # Examples
///
/// ```
/// assert_eq!(aoc::day_09::part_1().unwrap(), 1884768153);
/// ```
pub fn part_1() -> Result<i64> {
    sum_of_extrapolated_values(INPUT, false)
}

/// Part 2
///
/// # Examples
///
/// ```
/// assert_eq!(aoc::day_09::part_2().unwrap(), 1031);
/// ```
pub fn part_2() -> Result<i64> {
    sum_of_extrapolated_values(INPUT, true)
}

fn sum_of_extrapolated_values(s: &str, before: bool) -> Result<i64> {
    let mut sum = 0;
    for line in s.lines() {
        sum += extrapolate_line(line, before)?;
    }
    Ok(sum)
}

fn extrapolate_line(s: &str, before: bool) -> Result<i64> {
    let values = s
        .split_whitespace()
        .map(|s| s.parse::<i64>().map_err(Error::from))
        .collect::<Result<Vec<_>>>()?;
    Ok(extrapolate(&values, before))
}

fn extrapolate(values: &[i64], before: bool) -> i64 {
    if values.len() < 2 {
        return 0;
    }
    let value = if before {
        *values.first().unwrap()
    } else {
        *values.last().unwrap()
    };

    let mut all_zeros = true;
    let mut deltas = Vec::new();
    for (a, b) in values.iter().zip(values.iter().skip(1)) {
        let delta = b - a;
        if delta != 0 {
            all_zeros = false;
        }
        deltas.push(delta);
    }
    if all_zeros {
        value
    } else if before {
        value - extrapolate(&deltas, before)
    } else {
        value + extrapolate(&deltas, before)
    }
}

#[test]
fn part_1_example() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    assert_eq!(sum_of_extrapolated_values(input, false).unwrap(), 114);
}

#[test]
fn part_2_example() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    assert_eq!(sum_of_extrapolated_values(input, true).unwrap(), 2);
}
