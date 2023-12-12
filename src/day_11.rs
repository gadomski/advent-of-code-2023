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
    sum_of_the_shortest_paths(INPUT)
}

fn sum_of_the_shortest_paths(s: &str) -> Result<i64> {
    let image: Image = s.parse()?;
    let mut sum = 0;
    for (i, a) in image.0.iter().enumerate() {
        for b in image.0.iter().skip(i + 1) {
            sum += (a.row - b.row).abs() + (a.col - b.col).abs();
        }
    }
    Ok(sum)
}

#[derive(Debug)]
struct Image(Vec<Location>);

#[derive(Debug, Hash, PartialEq, Eq)]
struct Location {
    row: i64,
    col: i64,
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
        let mut row = min_row;
        while row <= max_row {
            if !locations.iter().any(|location| location.row == row) {
                for location in &mut locations {
                    if location.row > row {
                        location.row += 1;
                    }
                }
                max_row += 1;
                row += 2;
            } else {
                row += 1;
            }
        }
        let mut col = min_col;
        while col <= max_col {
            if !locations.iter().any(|location| location.col == col) {
                for location in &mut locations {
                    if location.col > col {
                        location.col += 1;
                    }
                }
                max_col += 1;
                col += 2;
            } else {
                col += 1;
            }
        }
        Ok(Image(locations))
    }
}

#[test]
fn part_1_example() {
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
    assert_eq!(sum_of_the_shortest_paths(input).unwrap(), 374);
}
