//! Day 04

use anyhow::{anyhow, Error, Result};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    str::FromStr,
};

const INPUT: &str = include_str!("../input/day_04.txt");

/// Part 1
///
/// # Examples
///
/// ```
/// assert_eq!(aoc::day_04::part_1().unwrap(), 22488);
/// ```
pub fn part_1() -> Result<i64> {
    points(INPUT)
}

/// Part 2
///
/// # Examples
///
/// ```
/// assert_eq!(aoc::day_04::part_2().unwrap(), 7013204);
/// ```
pub fn part_2() -> Result<i64> {
    number_of_scratchcards(INPUT)
}

fn points(s: &str) -> Result<i64> {
    let mut points = 0;
    for line in s.lines() {
        let card: Card = line.parse()?;
        points += card.points()?;
    }
    Ok(points)
}

fn number_of_scratchcards(s: &str) -> Result<i64> {
    let mut cards = BTreeMap::new();
    let mut counts = HashMap::new();
    for line in s.lines() {
        let card: Card = line.parse()?;
        let _ = counts.insert(card.id, 1);
        let _unused = cards.insert(card.id, card);
    }
    for (id, card) in cards {
        let count = counts.get(&id).unwrap().clone();
        let number_of_matches: i64 = card.number_of_matches().try_into()?;
        for i in 0..number_of_matches {
            let future_count = counts.get_mut(&(i + id + 1)).unwrap();
            *future_count += count;
        }
    }
    Ok(counts.values().sum())
}

#[derive(Debug)]
struct Card {
    id: i64,
    winning_numbers: HashSet<i64>,
    numbers: HashSet<i64>,
}

impl Card {
    fn number_of_matches(&self) -> usize {
        self.numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count()
    }

    fn points(&self) -> Result<i64> {
        let number_of_matches = self.number_of_matches();
        if number_of_matches > 0 {
            Ok(2_i64.pow((number_of_matches - 1).try_into()?))
        } else {
            Ok(0)
        }
    }
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Card> {
        let (a, b) = crate::str::split_once(s, ':')?;
        let mut iter = a.split_whitespace();
        let card = iter
            .next()
            .ok_or_else(|| anyhow!("invalid string: {}", s))?;
        let number = iter
            .next()
            .ok_or_else(|| anyhow!("invalid string: {}", s))?;
        if iter.next().is_some() {
            return Err(anyhow!("invalid string: {}", s));
        }
        if card != "Card" {
            return Err(anyhow!("string does not start with 'Card': {}", s));
        }
        let (a, b) = crate::str::split_once(b, '|')?;
        let winning_numbers = a
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i64>().map_err(Error::from))
            .collect::<Result<HashSet<_>>>()?;
        let numbers = b
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i64>().map_err(Error::from))
            .collect::<Result<HashSet<_>>>()?;
        Ok(Card {
            id: number.parse()?,
            winning_numbers,
            numbers,
        })
    }
}

#[test]
fn card_points() {
    assert_eq!(
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
            .parse::<Card>()
            .unwrap()
            .points()
            .unwrap(),
        8
    );
    assert_eq!(
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"
            .parse::<Card>()
            .unwrap()
            .points()
            .unwrap(),
        2
    );
    assert_eq!(
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"
            .parse::<Card>()
            .unwrap()
            .points()
            .unwrap(),
        2
    );
    assert_eq!(
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"
            .parse::<Card>()
            .unwrap()
            .points()
            .unwrap(),
        1
    );
    assert_eq!(
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"
            .parse::<Card>()
            .unwrap()
            .points()
            .unwrap(),
        0
    );
    assert_eq!(
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .parse::<Card>()
            .unwrap()
            .points()
            .unwrap(),
        0
    );
}

#[test]
fn part_1_example() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(points(input).unwrap(), 13);
}

#[test]
fn part_2_example() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(number_of_scratchcards(input).unwrap(), 30);
}
