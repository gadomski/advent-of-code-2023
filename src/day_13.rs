//! Day 13

use anyhow::{anyhow, Error, Result};
use std::{collections::HashSet, str::FromStr};

const INPUT: &str = include_str!("../input/day_13.txt");

/// Part 1
///
/// # Examples
///
/// ```
/// assert_eq!(aoc::day_13::part_1().unwrap(), 32723);
/// ```
pub fn part_1() -> Result<i64> {
    summarize(INPUT, false)
}

/// Part 2
///
/// # Examples
///
/// ```
/// assert_eq!(aoc::day_13::part_2().unwrap(), 34536);
/// ```
pub fn part_2() -> Result<i64> {
    summarize(INPUT, true)
}

fn summarize(s: &str, smudge: bool) -> Result<i64> {
    let mut sum = 0;
    for (i, pattern) in s.split("\n\n").enumerate() {
        let pattern = pattern.parse::<Pattern>()?.smudge(smudge);
        if let Some(col) = pattern.find_vertical_reflection() {
            sum += col + 1;
        } else if let Some(row) = pattern.find_horizontal_reflection() {
            sum += 100 * (row + 1);
        } else {
            return Err(anyhow!("no reflection found: {}", i));
        }
    }
    Ok(sum)
}

#[derive(Debug)]
struct Pattern {
    rocks: HashSet<Location>,
    min: Location,
    max: Location,
    smudge: bool,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Location {
    row: i64,
    col: i64,
}

impl Pattern {
    fn smudge(mut self, smudge: bool) -> Pattern {
        self.smudge = smudge;
        self
    }

    fn find_vertical_reflection(&self) -> Option<i64> {
        for col in 0..self.max.col {
            if self.has_vertical_reflection(col) {
                return Some(col);
            }
        }
        None
    }

    fn has_vertical_reflection(&self, col: i64) -> bool {
        let mut left = col;
        let mut right = col + 1;
        let mut one_smudge = false;
        while left >= self.min.col && right <= self.max.col {
            for row in self.min.row..=self.max.row {
                if !self.equal(Location { row, col: left }, Location { row, col: right }) {
                    if self.smudge {
                        if one_smudge {
                            return false;
                        } else {
                            one_smudge = true;
                        }
                    } else {
                        return false;
                    }
                }
            }
            left -= 1;
            right += 1;
        }
        if self.smudge {
            one_smudge
        } else {
            true
        }
    }

    fn find_horizontal_reflection(&self) -> Option<i64> {
        for row in 0..self.max.row {
            if self.has_horizontal_reflection(row) {
                return Some(row);
            }
        }
        None
    }

    fn has_horizontal_reflection(&self, row: i64) -> bool {
        let mut top = row;
        let mut bottom = row + 1;
        let mut one_smudge = false;
        while top >= self.min.row && bottom <= self.max.row {
            for col in self.min.col..=self.max.col {
                if !self.equal(Location { row: top, col }, Location { row: bottom, col }) {
                    if self.smudge {
                        if one_smudge {
                            return false;
                        } else {
                            one_smudge = true;
                        }
                    } else {
                        return false;
                    }
                }
            }
            top -= 1;
            bottom += 1;
        }
        if self.smudge {
            one_smudge
        } else {
            true
        }
    }

    fn equal(&self, a: Location, b: Location) -> bool {
        self.rocks.contains(&a) == self.rocks.contains(&b)
    }
}

impl FromStr for Pattern {
    type Err = Error;

    fn from_str(s: &str) -> Result<Pattern> {
        let mut rocks = HashSet::new();
        let mut min = Location {
            row: i64::MAX,
            col: i64::MAX,
        };
        let mut max = Location {
            row: i64::MIN,
            col: i64::MIN,
        };
        for (row, line) in s.lines().enumerate() {
            let row = row.try_into()?;
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    let col = col.try_into()?;
                    let _ = rocks.insert(Location { row, col });
                    if row < min.row {
                        min.row = row;
                    }
                    if col < min.col {
                        min.col = col;
                    }
                    if row > max.row {
                        max.row = row;
                    }
                    if col > max.col {
                        max.col = col;
                    }
                }
            }
        }
        Ok(Pattern {
            rocks,
            min,
            max,
            smudge: false,
        })
    }
}

#[test]
fn part_1_example() {
    let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    assert_eq!(summarize(input, false).unwrap(), 405);
}

#[test]
fn part_2_example() {
    let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    assert_eq!(summarize(input, true).unwrap(), 400);
}
