//! Day 14

use anyhow::{anyhow, Error, Result};
use std::{collections::HashMap, fmt::Display, str::FromStr};

const INPUT: &str = include_str!("../input/day_14.txt");

/// Part 1
///
/// # Examples
///
/// ```
/// assert_eq!(aoc::day_14::part_1().unwrap(), 109345);
/// ```
pub fn part_1() -> Result<i64> {
    total_load_after_tilting_north(INPUT)
}

fn total_load_after_tilting_north(s: &str) -> Result<i64> {
    use Direction::*;

    let mut platform: Platform = s.parse()?;
    platform.tilt(North);
    platform.run();
    Ok(platform.total_load())
}

#[derive(Debug)]
struct Platform {
    rocks: HashMap<Location, Rock>,
    tilt: Option<Direction>,
    min: Location,
    max: Location,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Location {
    row: i64,
    col: i64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Rock {
    Round,
    Cube,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    _South,
    _East,
    _West,
}

impl Platform {
    fn tilt(&mut self, direction: Direction) {
        self.tilt = Some(direction);
    }

    fn run(&mut self) {
        if let Some(tilt) = self.tilt {
            while self.run_one(tilt) {}
        }
    }

    fn run_one(&mut self, tilt: Direction) -> bool {
        use Rock::*;

        let mut new_rocks = HashMap::new();
        let mut moved = false;
        for (&location, &rock) in self.rocks.iter() {
            match rock {
                Round => {
                    let destination = location.neighbor(tilt);
                    if self.rocks.contains_key(&destination) || self.is_off_platform(destination) {
                        let _ = new_rocks.insert(location, rock);
                    } else {
                        let _ = new_rocks.insert(destination, rock);
                        moved = true;
                    }
                }
                Cube => {
                    let _ = new_rocks.insert(location, rock);
                }
            }
        }
        let _unused = std::mem::replace(&mut self.rocks, new_rocks);
        moved
    }

    fn is_off_platform(&self, location: Location) -> bool {
        location.col < self.min.col
            || location.row < self.min.row
            || location.col > self.max.col
            || location.row > self.max.row
    }

    fn total_load(&self) -> i64 {
        let mut sum = 0;
        for location in self.rocks.iter().filter_map(|(location, rock)| {
            if *rock == Rock::Round {
                Some(location)
            } else {
                None
            }
        }) {
            sum += self.max.row - location.row + 1;
        }
        sum
    }
}

impl Location {
    fn new(row: i64, col: i64) -> Location {
        Location { row, col }
    }

    fn neighbor(&self, direction: Direction) -> Location {
        use Direction::*;

        match direction {
            North => Location::new(self.row - 1, self.col),
            _South => Location::new(self.row + 1, self.col),
            _East => Location::new(self.row, self.col + 1),
            _West => Location::new(self.row, self.col - 1),
        }
    }
}

impl FromStr for Platform {
    type Err = Error;

    fn from_str(s: &str) -> Result<Platform> {
        use Rock::*;

        let mut rocks = HashMap::new();
        let mut max = Location::new(0, 0);
        for (row, line) in s.lines().enumerate() {
            let row: i64 = row.try_into()?;
            for (col, c) in line.chars().enumerate() {
                let col: i64 = col.try_into()?;
                let location = Location::new(row, col);
                match c {
                    'O' => {
                        let _ = rocks.insert(location, Round);
                    }
                    '#' => {
                        let _ = rocks.insert(location, Cube);
                    }
                    '.' => {} // empty
                    _ => return Err(anyhow!("unexpected character: {}", c)),
                }
                if row > max.row {
                    max.row = row;
                }
                if col > max.col {
                    max.col = col;
                }
            }
        }
        Ok(Platform {
            rocks,
            tilt: None,
            min: Location::new(0, 0),
            max,
        })
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.min.row..=self.max.col {
            for col in self.min.col..=self.max.col {
                let c = if let Some(rock) = self.rocks.get(&Location::new(row, col)) {
                    match rock {
                        Rock::Cube => '#',
                        Rock::Round => 'O',
                    }
                } else {
                    '.'
                };
                write!(f, "{}", c)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[test]
fn part_1_example() {
    let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    assert_eq!(total_load_after_tilting_north(input).unwrap(), 136);
}
