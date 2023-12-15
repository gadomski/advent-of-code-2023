//! Day 15

use anyhow::{Error, Result};
use std::{collections::HashMap, str::FromStr};

const INPUT: &str = include_str!("../input/day_15.txt");

/// Part 1
///
/// # Examples
///
/// ```
/// assert_eq!(aoc::day_15::part_1().unwrap(), 520500);
/// ```
pub fn part_1() -> Result<u64> {
    sum_of_hashes(INPUT)
}

/// Part 2
///
/// # Examples
///
/// ```
/// assert_eq!(aoc::day_15::part_2().unwrap(), 213097);
/// ```
pub fn part_2() -> Result<u64> {
    focusing_power(INPUT)
}

fn hash(s: &str) -> Result<u8> {
    let mut value = 0;
    for c in s.chars() {
        value += u64::from(c);
        value *= 17;
        value = value % 256;
    }
    u8::try_from(value).map_err(Error::from)
}

fn sum_of_hashes(s: &str) -> Result<u64> {
    let mut sum = 0;
    for value in s.trim().split(',') {
        sum += u64::from(hash(value)?);
    }
    Ok(sum)
}

fn focusing_power(s: &str) -> Result<u64> {
    let mut boxes: HashMap<u8, Box> = HashMap::new();
    for step in s.trim().split(',') {
        if step.ends_with('-') {
            let label = &step[0..step.len() - 1];
            let box_number = hash(label)?;
            if let Some(box_) = boxes.get_mut(&box_number) {
                box_.remove_lens(label);
            }
        } else {
            let lens: Lens = step.parse()?;
            let box_ = boxes.entry(lens.box_number()?).or_default();
            box_.add_lens(lens);
        }
    }
    let mut focusing_power = 0;
    for (&box_number, box_) in boxes.iter() {
        focusing_power += box_.focusing_power(box_number)?;
    }
    Ok(focusing_power)
}

#[derive(Debug, Default)]
struct Box {
    lenses: Vec<Lens>,
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: u64,
}

impl Box {
    fn add_lens(&mut self, lens: Lens) {
        if let Some(index) = self
            .lenses
            .iter()
            .position(|other| other.label == lens.label)
        {
            let _unused = self.lenses.remove(index);
            self.lenses.insert(index, lens);
        } else {
            self.lenses.push(lens);
        }
    }

    fn remove_lens(&mut self, label: &str) {
        if let Some(index) = self.lenses.iter().position(|other| other.label == label) {
            let _unused = self.lenses.remove(index);
        }
    }

    fn focusing_power(&self, box_number: u8) -> Result<u64> {
        let mut focusing_power = 0;
        for (i, lens) in self.lenses.iter().enumerate() {
            focusing_power +=
                (u64::from(box_number) + 1) * (u64::try_from(i)? + 1) * lens.focal_length;
        }
        Ok(focusing_power)
    }
}

impl Lens {
    fn box_number(&self) -> Result<u8> {
        hash(&self.label)
    }
}

impl FromStr for Lens {
    type Err = Error;

    fn from_str(s: &str) -> Result<Lens> {
        let (label, focal_length) = crate::str::split_once(s, '=')?;
        Ok(Lens {
            label: label.to_string(),
            focal_length: focal_length.parse()?,
        })
    }
}

#[test]
fn part_1_example() {
    assert_eq!(hash("HASH").unwrap(), 52);
    assert_eq!(
        sum_of_hashes("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7").unwrap(),
        1320
    );
}

#[test]
fn part_2_example() {
    assert_eq!(
        focusing_power("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7").unwrap(),
        145
    );
}
