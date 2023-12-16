//! Day 16

use anyhow::{anyhow, Error, Result};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

const INPUT: &str = include_str!("../input/day_16.txt");

/// Part 1
///
/// # Examples
///
/// ```
/// assert_eq!(aoc::day_16::part_1().unwrap(), 7498);
/// ```
pub fn part_1() -> Result<usize> {
    energized_tiles(INPUT)
}

fn energized_tiles(s: &str) -> Result<usize> {
    let contraption: Contraption = s.parse()?;
    Ok(contraption.energized_tiles())
}

#[derive(Debug)]
struct Contraption {
    map: HashMap<Location, Device>,
    min: Location,
    max: Location,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Location {
    row: i64,
    col: i64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Beam {
    location: Location,
    direction: Direction,
}

#[derive(Debug)]
enum Device {
    VerticalSplitter,
    HorizontalSplitter,
    BackslashMirror,
    SlashMirror,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Contraption {
    fn energized_tiles(&self) -> usize {
        use Device::*;
        use Direction::*;

        let mut active_beams = vec![Beam::new(0, -1, East)];
        let mut beams = HashSet::new();
        while let Some(beam) = active_beams.pop() {
            if beam.location != Location::new(0, -1) && !beams.insert(beam) {
                continue;
            }
            let beam = beam.advance();
            if !self.contains(beam.location) {
                continue;
            }
            if let Some(device) = self.map.get(&beam.location) {
                match device {
                    VerticalSplitter => match beam.direction {
                        East | West => {
                            active_beams.push(beam.north());
                            active_beams.push(beam.south());
                        }
                        North | South => active_beams.push(beam),
                    },
                    HorizontalSplitter => match beam.direction {
                        North | South => {
                            active_beams.push(beam.east());
                            active_beams.push(beam.west());
                        }
                        East | West => active_beams.push(beam),
                    },
                    BackslashMirror => match beam.direction {
                        North => active_beams.push(beam.west()),
                        West => active_beams.push(beam.north()),
                        South => active_beams.push(beam.east()),
                        East => active_beams.push(beam.south()),
                    },
                    SlashMirror => match beam.direction {
                        North => active_beams.push(beam.east()),
                        East => active_beams.push(beam.north()),
                        South => active_beams.push(beam.west()),
                        West => active_beams.push(beam.south()),
                    },
                }
            } else {
                active_beams.push(beam);
            }
        }
        let locations: HashSet<Location> = beams.iter().map(|beam| beam.location).collect();
        locations.len()
    }

    fn contains(&self, location: Location) -> bool {
        self.min.row <= location.row
            && self.min.col <= location.col
            && self.max.row >= location.row
            && self.max.col >= location.col
    }

    #[allow(unused)]
    fn print_beams(&self, beams: &HashSet<Beam>) {
        use Device::*;
        use Direction::*;
        for row in 0..=self.max.row {
            for col in 0..=self.max.col {
                let location = Location::new(row, col);
                if let Some(device) = self.map.get(&location) {
                    print!(
                        "{}",
                        match device {
                            BackslashMirror => '\\',
                            SlashMirror => '/',
                            VerticalSplitter => '|',
                            HorizontalSplitter => '-',
                        }
                    );
                } else {
                    let beams: Vec<_> = beams
                        .iter()
                        .filter(|beam| beam.location == location)
                        .collect();
                    print!(
                        "{}",
                        if beams.is_empty() {
                            '.'
                        } else if beams.len() == 1 {
                            match beams[0].direction {
                                North => '^',
                                South => 'v',
                                East => '>',
                                West => '<',
                            }
                        } else if beams.len() == 2 {
                            '2'
                        } else if beams.len() == 3 {
                            '3'
                        } else if beams.len() == 4 {
                            '4'
                        } else {
                            panic!()
                        },
                    );
                }
            }
            println!();
        }
    }
}

impl Location {
    fn new(row: i64, col: i64) -> Location {
        Location { row, col }
    }
}

impl Beam {
    fn new(row: i64, col: i64, direction: Direction) -> Beam {
        Beam {
            location: Location::new(row, col),
            direction,
        }
    }

    fn advance(self) -> Beam {
        use Direction::*;

        let location = match self.direction {
            North => Location::new(self.location.row - 1, self.location.col),
            South => Location::new(self.location.row + 1, self.location.col),
            East => Location::new(self.location.row, self.location.col + 1),
            West => Location::new(self.location.row, self.location.col - 1),
        };
        Beam {
            location,
            direction: self.direction,
        }
    }

    fn north(self) -> Beam {
        Beam {
            location: self.location,
            direction: Direction::North,
        }
    }

    fn south(self) -> Beam {
        Beam {
            location: self.location,
            direction: Direction::South,
        }
    }

    fn east(self) -> Beam {
        Beam {
            location: self.location,
            direction: Direction::East,
        }
    }

    fn west(self) -> Beam {
        Beam {
            location: self.location,
            direction: Direction::West,
        }
    }
}

impl FromStr for Contraption {
    type Err = Error;

    fn from_str(s: &str) -> Result<Contraption> {
        let mut map = HashMap::new();
        let mut max = Location::new(i64::MIN, i64::MIN);
        for (row, line) in s.lines().enumerate() {
            let row = row.try_into()?;
            if row > max.row {
                max.row = row;
            }
            for (col, c) in line.chars().enumerate() {
                let col = col.try_into()?;
                if col > max.col {
                    max.col = col;
                }
                if let Some(device) = match c {
                    '\\' => Some(Device::BackslashMirror),
                    '/' => Some(Device::SlashMirror),
                    '|' => Some(Device::VerticalSplitter),
                    '-' => Some(Device::HorizontalSplitter),
                    '.' => None,
                    _ => return Err(anyhow!("invalid character: {}", c)),
                } {
                    let _ = map.insert(Location::new(row, col), device);
                }
            }
        }
        Ok(Contraption {
            map,
            min: Location::new(0, 0),
            max,
        })
    }
}

#[test]
fn part_1_example() {
    let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
    assert_eq!(energized_tiles(input).unwrap(), 46);
}
