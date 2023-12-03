//! Day 03

use anyhow::{Error, Result};
use std::str::FromStr;

const INPUT: &str = include_str!("../input/day_03.txt");

/// Part 1
///
/// # Example
///
/// ```
/// assert_eq!(aoc::day_03::part_1().unwrap(), 554003);
/// ```
pub fn part_1() -> Result<i64> {
    sum_of_all_part_numbers(INPUT)
}

/// Part 2
///
/// # Example
///
/// ```
/// assert_eq!(aoc::day_03::part_2().unwrap(), 87263515);
/// ```
pub fn part_2() -> Result<i64> {
    sum_of_all_gear_ratios(INPUT)
}

fn sum_of_all_part_numbers(s: &str) -> Result<i64> {
    let schematic: Schematic = s.parse()?;
    Ok(schematic.part_numbers().into_iter().sum())
}

fn sum_of_all_gear_ratios(s: &str) -> Result<i64> {
    let schematic: Schematic = s.parse()?;
    Ok(schematic.gear_ratios().into_iter().sum())
}

#[derive(Debug, Default)]
struct Schematic {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

#[derive(Debug)]
struct Number {
    value: i64,
    row: i64,
    start_col: i64,
    end_col: i64,
}

#[derive(Debug)]
struct Symbol {
    symbol: char,
    row: i64,
    col: i64,
}

impl Schematic {
    fn add_number(&mut self, number: &str, row: i64, start_col: i64, end_col: i64) -> Result<()> {
        self.numbers.push(Number {
            value: number.parse()?,
            row,
            start_col,
            end_col,
        });
        Ok(())
    }

    fn add_symbol(&mut self, symbol: char, row: i64, col: i64) {
        self.symbols.push(Symbol {
            symbol: symbol,
            row,
            col,
        });
    }

    fn part_numbers(&self) -> Vec<i64> {
        let mut part_numbers = Vec::new();
        for symbol in &self.symbols {
            for number in &self.numbers {
                if number.is_adjacent(symbol) {
                    part_numbers.push(number.value);
                }
            }
        }
        part_numbers
    }

    fn gear_ratios(&self) -> Vec<i64> {
        let mut gear_ratios = Vec::new();
        'outer: for symbol in &self.symbols {
            if symbol.symbol != '*' {
                continue;
            }
            let mut numbers = Vec::new();
            for number in &self.numbers {
                if number.is_adjacent(symbol) {
                    numbers.push(number);
                    if numbers.len() > 2 {
                        continue 'outer;
                    }
                }
            }
            if numbers.len() == 2 {
                gear_ratios.push(numbers[0].value * numbers[1].value);
            }
        }
        gear_ratios
    }
}

impl Number {
    fn is_adjacent(&self, symbol: &Symbol) -> bool {
        self.row.abs_diff(symbol.row) <= 1
            && symbol.col >= (self.start_col - 1)
            && symbol.col <= (self.end_col + 1)
    }
}

impl FromStr for Schematic {
    type Err = Error;

    fn from_str(s: &str) -> Result<Schematic> {
        let mut schematic = Schematic::default();
        for (row, line) in s.lines().enumerate() {
            let row: i64 = row.try_into()?;
            let mut start_col = 0;
            let mut end_col = 0;
            let mut number = String::new();
            for (col, c) in line.chars().enumerate() {
                let col: i64 = col.try_into()?;
                if c.is_ascii_digit() {
                    if number.is_empty() {
                        start_col = col;
                    }
                    end_col = col;
                    number.push(c);
                } else {
                    if !number.is_empty() {
                        schematic.add_number(&number, row, start_col, end_col)?;
                        number.clear();
                    }
                    if c != '.' {
                        schematic.add_symbol(c, row, col);
                    }
                }
            }
            if !number.is_empty() {
                schematic.add_number(&number, row, start_col, end_col)?;
            }
        }
        Ok(schematic)
    }
}

#[test]
fn part_1_example() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(sum_of_all_part_numbers(input).unwrap(), 4361);
}

#[test]
fn is_adjacent() {
    let number = Number {
        value: 467,
        row: 0,
        start_col: 0,
        end_col: 2,
    };
    let symbol = Symbol {
        symbol: '*',
        row: 1,
        col: 3,
    };
    assert!(number.is_adjacent(&symbol));
}

#[test]
fn back_to_back() {
    let input = "998*973";
    let schematic: Schematic = input.parse().unwrap();
    assert_eq!(schematic.part_numbers().len(), 2);
}

#[test]
fn part_2_example() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(sum_of_all_gear_ratios(input).unwrap(), 467835);
}
