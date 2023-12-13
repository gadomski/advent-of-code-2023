//! Day 12

use anyhow::{anyhow, Error, Result};
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day_12.txt");

/// Part 1
///
/// # Examples
///
/// ```
/// assert_eq!(aoc::day_12::part_1().unwrap(), 7169);
/// ```
pub fn part_1() -> Result<usize> {
    sum_of_number_of_arrangements(INPUT, false)
}

/// Part 2
pub fn part_2() -> Result<usize> {
    sum_of_number_of_arrangements(INPUT, true)
}

fn sum_of_number_of_arrangements(s: &str, unfold: bool) -> Result<usize> {
    let mut sum = 0;
    for line in s.lines() {
        sum += number_of_arrangements_for_line(line, unfold)?;
    }
    Ok(sum)
}

fn number_of_arrangements_for_line(line: &str, unfold: bool) -> Result<usize> {
    let (conditions, groups) = crate::str::split_once(line, ' ')?;
    let mut conditions = conditions
        .chars()
        .map(Condition::from_char)
        .collect::<Result<Vec<_>>>()?;
    let mut groups = groups
        .split(',')
        .map(|s| s.parse().map_err(Error::from))
        .collect::<Result<Vec<usize>>>()?;
    if unfold {
        let original_conditions = conditions.clone();
        let original_groups = groups.clone();
        for _ in 0..4 {
            conditions.push(Condition::Unknown);
            conditions.extend(original_conditions.iter());
            groups.extend(original_groups.iter());
        }
    }
    let mut cache = Cache::new();
    Ok(number_of_arrangements(&conditions, &groups, &mut cache))
}

fn number_of_arrangements(conditions: &[Condition], groups: &[usize], cache: &mut Cache) -> usize {
    if let Some(count) = cache.get(conditions, groups) {
        return count;
    }
    if let Some(condition) = conditions.first() {
        let mut sum = 0;
        if condition.is_not_damaged() {
            let count = number_of_arrangements(&conditions[1..], groups, cache);
            cache.insert(&conditions[1..], groups, count);
            sum += count;
        }
        if let Some(&group) = groups.first() {
            if condition.is_not_operational() {
                if group <= conditions.len()
                    && conditions[0..group]
                        .iter()
                        .all(|&condition| condition.is_not_operational())
                {
                    if group == conditions.len() {
                        if groups.len() == 1 {
                            sum += 1;
                        }
                    } else if conditions[group].is_not_damaged() {
                        let count =
                            number_of_arrangements(&conditions[group + 1..], &groups[1..], cache);
                        cache.insert(&conditions[group + 1..], &groups[1..], count);
                        sum += count;
                    }
                }
            }
        }
        sum
    } else if groups.is_empty() {
        1
    } else {
        0
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct Cache(HashMap<String, usize>);

impl Condition {
    fn from_char(c: char) -> Result<Condition> {
        use Condition::*;

        match c {
            '.' => Ok(Operational),
            '#' => Ok(Damaged),
            '?' => Ok(Unknown),
            _ => Err(anyhow!("unknown condition char: {}", c)),
        }
    }

    fn is_not_operational(&self) -> bool {
        *self != Condition::Operational
    }

    fn is_not_damaged(&self) -> bool {
        *self != Condition::Damaged
    }
}

impl Cache {
    fn new() -> Cache {
        Cache(HashMap::new())
    }

    fn insert(&mut self, conditions: &[Condition], groups: &[usize], count: usize) {
        let key = key(conditions, groups);
        let _ = self.0.insert(key, count);
    }

    fn get(&self, conditions: &[Condition], groups: &[usize]) -> Option<usize> {
        let key = key(conditions, groups);
        self.0.get(&key).cloned()
    }
}

fn key(conditions: &[Condition], groups: &[usize]) -> String {
    use Condition::*;

    let mut key = String::new();
    for condition in conditions {
        match condition {
            Operational => key.push('.'),
            Damaged => key.push('#'),
            Unknown => key.push('?'),
        }
    }
    if !groups.is_empty() {
        key.push(' ');
        for group in &groups[0..groups.len() - 1] {
            key.push_str(&group.to_string());
            key.push(',');
        }
        key.push_str(&groups[groups.len() - 1].to_string());
    }
    key
}

#[allow(unused)]
fn print_conditions(conditions: &[Condition]) {
    use Condition::*;

    print!("[");
    for condition in conditions {
        print!(
            "{}",
            match condition {
                Operational => '.',
                Damaged => '#',
                Unknown => '?',
            }
        )
    }
    println!("]");
}

#[test]
fn part_1_examples() {
    assert_eq!(
        number_of_arrangements_for_line("???.### 1,1,3", false).unwrap(),
        1
    );
    assert_eq!(
        number_of_arrangements_for_line(".??..??...?##. 1,1,3", false).unwrap(),
        4
    );
    assert_eq!(
        number_of_arrangements_for_line("?#?#?#?#?#?#?#? 1,3,1,6", false).unwrap(),
        1
    );
    assert_eq!(
        number_of_arrangements_for_line("????.#...#... 4,1,1", false).unwrap(),
        1
    );
    assert_eq!(
        number_of_arrangements_for_line("????.######..#####. 1,6,5", false).unwrap(),
        4
    );
    assert_eq!(
        number_of_arrangements_for_line("?###???????? 3,2,1", false).unwrap(),
        10
    );
    assert_eq!(
        sum_of_number_of_arrangements(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
            false
        )
        .unwrap(),
        21
    );
}

#[test]
fn part_2_example() {
    assert_eq!(
        sum_of_number_of_arrangements(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
            true
        )
        .unwrap(),
        525152
    );
}
