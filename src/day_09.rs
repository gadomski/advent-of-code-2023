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
    sum_of_extrapolated_values(INPUT)
}

fn sum_of_extrapolated_values(s: &str) -> Result<i64> {
    let mut sum = 0;
    for line in s.lines() {
        sum += extrapolate_line(line)?;
    }
    Ok(sum)
}

fn extrapolate_line(s: &str) -> Result<i64> {
    let values = s
        .split_whitespace()
        .map(|s| s.parse::<i64>().map_err(Error::from))
        .collect::<Result<Vec<_>>>()?;
    Ok(extrapolate(&values))
}

fn extrapolate(values: &[i64]) -> i64 {
    if values.len() < 2 {
        return 0;
    }
    let last_value = *values
        .last()
        .expect("we already checked that the values slice is long enough to make one delta");

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
        last_value
    } else {
        last_value + extrapolate(&deltas)
    }
}

#[test]
fn part_1_example() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    assert_eq!(sum_of_extrapolated_values(input).unwrap(), 114);
}
