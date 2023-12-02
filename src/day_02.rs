use anyhow::{anyhow, Error, Result};
use std::{collections::HashMap, str::FromStr};

const INPUT: &str = include_str!("../input/day_02.txt");

pub fn part_1() -> Result<i64> {
    sum_of_ids_of_possible_games(INPUT)
}

pub fn part_2() -> Result<i64> {
    sum_of_the_power(INPUT)
}

fn sum_of_ids_of_possible_games(input: &str) -> Result<i64> {
    let mut sum = 0;
    for line in input.lines() {
        let game: Game = line.parse()?;
        if game.is_possible() {
            sum += game.id;
        }
    }
    Ok(sum)
}

fn sum_of_the_power(input: &str) -> Result<i64> {
    let mut sum = 0;
    for line in input.lines() {
        let game: Game = line.parse()?;
        sum += game.power();
    }
    Ok(sum)
}

#[derive(Debug)]
struct Game {
    id: i64,
    reveals: Vec<HashMap<Color, i64>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
    Blue,
    Red,
    Green,
}

impl Game {
    fn is_possible(&self) -> bool {
        self.reveals
            .iter()
            .all(|reveal| self.is_reveal_possible(reveal))
    }

    fn is_reveal_possible(&self, reveal: &HashMap<Color, i64>) -> bool {
        reveal
            .iter()
            .all(|(color, count)| self.is_color_and_count_possible(*color, *count))
    }

    fn is_color_and_count_possible(&self, color: Color, count: i64) -> bool {
        use Color::*;
        match color {
            Red => count <= 12,
            Green => count <= 13,
            Blue => count <= 14,
        }
    }

    fn power(&self) -> i64 {
        use Color::*;

        let mut colors = HashMap::with_capacity(3);
        for reveal in &self.reveals {
            for (color, count) in reveal {
                let previous_count = colors.entry(color).or_default();
                if count > previous_count {
                    *previous_count = *count;
                }
            }
        }
        colors.get(&Red).cloned().unwrap_or_default()
            * colors.get(&Blue).cloned().unwrap_or_default()
            * colors.get(&Green).cloned().unwrap_or_default()
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Game> {
        let (front, back) = split_once(s, ':')?;
        let (game_prefix, id) = split_once(front, ' ')?;
        if game_prefix != "Game" {
            return Err(anyhow!("string does not start with 'Game': {}", s));
        }
        let mut reveals = Vec::new();
        for s in back.split(';') {
            let mut reveal = HashMap::new();
            for s in s.split(',').map(|s| s.trim()) {
                let (count, color) = split_once(s, ' ')?;
                reveal.insert(color.parse()?, count.parse()?);
            }
            reveals.push(reveal);
        }
        Ok(Game {
            id: id.parse()?,
            reveals,
        })
    }
}

impl FromStr for Color {
    type Err = Error;

    fn from_str(s: &str) -> Result<Color> {
        use Color::*;
        match s {
            "green" => Ok(Green),
            "blue" => Ok(Blue),
            "red" => Ok(Red),
            _ => Err(anyhow!("invalid color: {}", s)),
        }
    }
}

fn split_once(s: &str, delimiter: char) -> Result<(&str, &str)> {
    s.split_once(delimiter)
        .ok_or_else(|| anyhow!("could not find '{}' in {}", delimiter, s))
}

#[test]
fn game_from_str() {
    let game: Game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        .parse()
        .unwrap();
    assert_eq!(game.id, 1);
    assert_eq!(game.reveals.len(), 3);
    assert_eq!(*game.reveals[0].get(&Color::Blue).unwrap(), 3);
}

#[test]
fn part_1_example() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(sum_of_ids_of_possible_games(input).unwrap(), 8);
}

#[test]
fn part_1_check() {
    assert_eq!(part_1().unwrap(), 2512);
}

#[test]
fn part_2_example() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(sum_of_the_power(input).unwrap(), 2286);
}
