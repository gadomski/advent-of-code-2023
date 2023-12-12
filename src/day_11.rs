//! Day 11

use anyhow::{Error, Result};
use std::str::FromStr;

const INPUT: &str = include_str!("../input/day_11.txt");

/// Part 1
///
/// # Examples
///
/// ```
/// assert_eq!(aoc::day_11::part_1().unwrap(), 9550717);
/// ```
pub fn part_1() -> Result<i64> {
    sum_of_the_shortest_paths(INPUT, 2)
}

/// Part 2
///
/// # Examples
///
/// ```
/// assert_eq!(aoc::day_11::part_2().unwrap(), 648458253817);
/// ```
pub fn part_2() -> Result<i64> {
    sum_of_the_shortest_paths(INPUT, 1_000_000)
}

fn sum_of_the_shortest_paths(s: &str, expansion: i64) -> Result<i64> {
    let mut image: Image = s.parse()?;
    image.expand(expansion);
    let mut sum = 0;
    for (i, a) in image.locations.iter().enumerate() {
        for b in image.locations.iter().skip(i + 1) {
            sum += (a.row - b.row).abs() + (a.col - b.col).abs();
        }
    }
    Ok(sum)
}

#[derive(Debug)]
struct Image {
    locations: Vec<Location>,
    min: Location,
    max: Location,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Location {
    row: i64,
    col: i64,
}

impl Image {
    fn expand(&mut self, expansion: i64) {
        let mut row = self.min.row;
        while row <= self.max.row {
            if !self.locations.iter().any(|location| location.row == row) {
                for location in &mut self.locations {
                    if location.row > row {
                        location.row += expansion - 1;
                    }
                }
                self.max.row += expansion - 1;
                row += expansion;
            } else {
                row += 1;
            }
        }
        let mut col = self.min.col;
        while col <= self.max.col {
            if !self.locations.iter().any(|location| location.col == col) {
                for location in &mut self.locations {
                    if location.col > col {
                        location.col += expansion - 1;
                    }
                }
                self.max.col += expansion - 1;
                col += expansion;
            } else {
                col += 1;
            }
        }
    }
}

impl FromStr for Image {
    type Err = Error;

    fn from_str(s: &str) -> Result<Image> {
        let mut locations = Vec::new();
        let mut min_row = i64::MAX;
        let mut max_row = i64::MIN;
        let mut min_col = i64::MAX;
        let mut max_col = i64::MIN;
        for (row, line) in s.lines().enumerate() {
            let row = row.try_into()?;
            for (col, c) in line.char_indices() {
                let col = col.try_into()?;
                if c == '#' {
                    if row < min_row {
                        min_row = row;
                    }
                    if row > max_row {
                        max_row = row;
                    }
                    if col < min_col {
                        min_col = col;
                    }
                    if col > max_col {
                        max_col = col;
                    }
                    locations.push(Location { row, col });
                }
            }
        }
        Ok(Image {
            locations,
            min: Location {
                row: min_row,
                col: min_col,
            },
            max: Location {
                row: max_row,
                col: max_col,
            },
        })
    }
}

#[test]
fn examples() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    assert_eq!(sum_of_the_shortest_paths(input, 2).unwrap(), 374);
    assert_eq!(sum_of_the_shortest_paths(input, 10).unwrap(), 1030);
    assert_eq!(sum_of_the_shortest_paths(input, 100).unwrap(), 8410);
}
