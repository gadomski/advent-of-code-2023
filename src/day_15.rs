//! Day 15

const INPUT: &str = include_str!("../input/day_15.txt");

/// Part 1
///
/// # Examples
///
/// ```
/// assert_eq!(aoc::day_15::part_1(), 520500);
/// ```
pub fn part_1() -> u64 {
    sum_of_hashes(INPUT)
}

fn hash(s: &str) -> u64 {
    let mut value = 0;
    for c in s.chars() {
        value += u64::from(c);
        value *= 17;
        value = value % 256;
    }
    value
}

fn sum_of_hashes(s: &str) -> u64 {
    s.trim().split(',').map(|s| hash(s)).sum()
}

#[test]
fn part_1_example() {
    assert_eq!(hash("HASH"), 52);
    assert_eq!(
        sum_of_hashes("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
        1320
    );
}
