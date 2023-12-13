//! Day 12

use anyhow::{anyhow, Error, Result};

const INPUT: &str = include_str!("../input/day_12.txt");

/// Part 1
///
/// # Examples
///
/// ```
/// assert_eq!(aoc::day_12::part_1().unwrap(), 7169);
/// ```
pub fn part_1() -> Result<usize> {
    sum_of_number_of_arrangements(INPUT)
}

fn sum_of_number_of_arrangements(s: &str) -> Result<usize> {
    let mut sum = 0;
    for line in s.lines() {
        sum += number_of_arrangements_for_line(line)?;
    }
    Ok(sum)
}

fn number_of_arrangements_for_line(line: &str) -> Result<usize> {
    let (conditions, groups) = crate::str::split_once(line, ' ')?;
    let conditions = conditions
        .chars()
        .map(Condition::from_char)
        .collect::<Result<Vec<_>>>()?;
    let groups = groups
        .split(',')
        .map(|s| s.parse().map_err(Error::from))
        .collect::<Result<Vec<usize>>>()?;
    Ok(number_of_arrangements(conditions, 0, &groups))
}

fn number_of_arrangements(mut conditions: Vec<Condition>, start: usize, groups: &[usize]) -> usize {
    use Condition::*;

    if let Some(condition) = conditions.get(start).cloned() {
        let mut sum = 0;
        if condition.is_not_damaged() {
            let mut conditions = conditions.clone();
            conditions[start] = Operational;
            sum += number_of_arrangements(conditions, start + 1, groups);
        }
        if let Some(&group) = groups.first() {
            let stop = start + group;
            if condition.is_not_operational() {
                if stop <= conditions.len()
                    && conditions[start..stop]
                        .iter()
                        .all(|&condition| condition.is_not_operational())
                {
                    for condition in &mut conditions[start..stop] {
                        *condition = Damaged;
                    }
                    if stop == conditions.len() {
                        if groups.len() == 1 {
                            sum += 1;
                        }
                    } else if conditions[stop].is_not_damaged() {
                        conditions[stop] = Operational;
                        sum += number_of_arrangements(conditions, stop + 1, &groups[1..]);
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
    assert_eq!(number_of_arrangements_for_line("???.### 1,1,3").unwrap(), 1);
    assert_eq!(
        number_of_arrangements_for_line(".??..??...?##. 1,1,3").unwrap(),
        4
    );
    assert_eq!(
        number_of_arrangements_for_line("?#?#?#?#?#?#?#? 1,3,1,6").unwrap(),
        1
    );
    assert_eq!(
        number_of_arrangements_for_line("????.#...#... 4,1,1").unwrap(),
        1
    );
    assert_eq!(
        number_of_arrangements_for_line("????.######..#####. 1,6,5").unwrap(),
        4
    );
    assert_eq!(
        number_of_arrangements_for_line("?###???????? 3,2,1").unwrap(),
        10
    );
    assert_eq!(
        sum_of_number_of_arrangements(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
        )
        .unwrap(),
        21
    );
}
