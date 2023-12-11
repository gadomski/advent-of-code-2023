//! Day 10

use anyhow::{anyhow, Error, Result};
use std::{collections::HashMap, fmt::Display, str::FromStr};

const INPUT: &str = include_str!("../input/day_10.txt");

/// Part 1
///
/// # Examples
///
/// ```
/// assert_eq!(aoc::day_10::part_1().unwrap(), 6701);
/// ```
pub fn part_1() -> Result<usize> {
    max_distance_from_start(INPUT)
}

fn max_distance_from_start(s: &str) -> Result<usize> {
    let map: Map = s.parse()?;
    map.max_distance_from_start()
}

#[derive(Debug, Default)]
struct Map {
    starting_position: Option<Location>,
    pipes: HashMap<Location, Pipe>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Location {
    row: i64,
    col: i64,
}

#[derive(Debug)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    East,
    West,
    North,
    South,
}

impl Map {
    fn set_starting_position(&mut self, row: usize, col: usize) -> Result<()> {
        self.starting_position = Some((row.try_into()?, col.try_into()?).into());
        Ok(())
    }

    fn add_pipe(&mut self, row: usize, col: usize, pipe: Pipe) -> Result<()> {
        let _ = self
            .pipes
            .insert((row.try_into()?, col.try_into()?).into(), pipe);
        Ok(())
    }

    fn max_distance_from_start(&self) -> Result<usize> {
        let starting_position = self
            .starting_position
            .ok_or_else(|| anyhow!("no starting position"))?;
        let mut seen = vec![starting_position];
        while seen.len() == 1 || seen.first().unwrap() != seen.last().unwrap() {
            let location = self.next_location(&seen)?;
            seen.push(location);
        }
        Ok((seen.len() - 1) / 2)
    }

    fn next_location(&self, seen: &[Location]) -> Result<Location> {
        let location = *seen.last().ok_or_else(|| anyhow!("empty seen array"))?;
        if seen.len() == 1 {
            for (neighbor, direction) in location.neighbors() {
                if let Some(pipe) = self.pipes.get(&neighbor) {
                    if pipe.connects_to(direction.inverse()) {
                        return Ok(neighbor);
                    }
                }
            }
            Err(anyhow!("no connecting neighbor: {}", location,))
        } else {
            let from = seen[seen.len() - 2];
            let from_direction = Direction::to_neighbor(location, from);
            let pipe = self
                .pipes
                .get(&location)
                .ok_or_else(|| anyhow!("no pipe at location: {}", location))?;
            let to_direction = pipe.other_side(from_direction).ok_or_else(|| {
                anyhow!(
                    "pipe does not connect to direction: {:?}, {:?}",
                    pipe,
                    from_direction
                )
            })?;
            let to = to_direction.location_from(location);
            Ok(to)
        }
    }
}

impl Pipe {
    fn connects_to(&self, direction: Direction) -> bool {
        use Direction::*;
        use Pipe::*;

        match self {
            Vertical => match direction {
                North | South => true,
                East | West => false,
            },
            Horizontal => match direction {
                East | West => true,
                North | South => false,
            },
            NorthEast => match direction {
                North | East => true,
                _ => false,
            },
            NorthWest => match direction {
                North | West => true,
                _ => false,
            },
            SouthWest => match direction {
                South | West => true,
                _ => false,
            },
            SouthEast => match direction {
                South | East => true,
                _ => false,
            },
        }
    }

    fn other_side(&self, direction: Direction) -> Option<Direction> {
        use Direction::*;
        use Pipe::*;

        match self {
            Vertical => match direction {
                North => Some(South),
                South => Some(North),
                _ => None,
            },
            Horizontal => match direction {
                East => Some(West),
                West => Some(East),
                _ => None,
            },
            NorthEast => match direction {
                North => Some(East),
                East => Some(North),
                _ => None,
            },
            NorthWest => match direction {
                North => Some(West),
                West => Some(North),
                _ => None,
            },
            SouthEast => match direction {
                South => Some(East),
                East => Some(South),
                _ => None,
            },
            SouthWest => match direction {
                South => Some(West),
                West => Some(South),
                _ => None,
            },
        }
    }
}

impl Direction {
    fn inverse(&self) -> Direction {
        use Direction::*;

        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }

    fn to_neighbor(from: Location, to: Location) -> Direction {
        use Direction::*;

        if from.row < to.row {
            assert!(from.col == to.col);
            South
        } else if from.row > to.row {
            assert!(from.col == to.col);
            North
        } else if from.col < to.col {
            assert!(from.row == to.row);
            East
        } else if from.col > to.col {
            assert!(from.row == to.row);
            West
        } else {
            panic!("should not be checking neighbors on diagonals")
        }
    }

    fn location_from(&self, from: Location) -> Location {
        use Direction::*;

        match self {
            North => (from.row - 1, from.col).into(),
            South => (from.row + 1, from.col).into(),
            East => (from.row, from.col + 1).into(),
            West => (from.row, from.col - 1).into(),
        }
    }
}

impl Location {
    fn neighbors(&self) -> [(Location, Direction); 4] {
        use Direction::*;

        [
            ((self.row, self.col - 1).into(), West),
            ((self.row - 1, self.col).into(), North),
            ((self.row + 1, self.col).into(), South),
            ((self.row, self.col + 1).into(), East),
        ]
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Map> {
        let mut map = Map::default();
        for (row, line) in s.lines().enumerate() {
            for (col, c) in line.char_indices() {
                if c == 'S' {
                    map.set_starting_position(row, col)?;
                } else if c == '.' {
                    // ground
                } else {
                    map.add_pipe(row, col, c.try_into()?)?;
                }
            }
        }
        if map.starting_position.is_none() {
            Err(anyhow!("no starting position found"))
        } else {
            Ok(map)
        }
    }
}

impl TryFrom<char> for Pipe {
    type Error = Error;

    fn try_from(value: char) -> Result<Pipe> {
        use Pipe::*;

        match value {
            '|' => Ok(Vertical),
            '-' => Ok(Horizontal),
            'L' => Ok(NorthEast),
            'J' => Ok(NorthWest),
            '7' => Ok(SouthWest),
            'F' => Ok(SouthEast),
            _ => Err(anyhow!("invalid pipe character: {}", value)),
        }
    }
}

impl From<(i64, i64)> for Location {
    fn from(value: (i64, i64)) -> Location {
        Location {
            row: value.0,
            col: value.1,
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[test]
fn part_1_examples() {
    let input = ".....
.S-7.
.|.|.
.L-J.
.....";
    assert_eq!(max_distance_from_start(input).unwrap(), 4);

    let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
    assert_eq!(max_distance_from_start(input).unwrap(), 8);
}
