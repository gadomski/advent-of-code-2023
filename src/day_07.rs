//! Day 07

use anyhow::{anyhow, Result};
use std::{cmp::Ordering, collections::HashMap};

const INPUT: &str = include_str!("../input/day_07.txt");

/// Part 1
///
/// # Examples
///
/// ```
/// assert_eq!(aoc::day_07::part_1().unwrap(), 250232501);
/// ```
pub fn part_1() -> Result<i64> {
    total_winnings(INPUT, false)
}

/// Part 2
///
/// # Examples
///
/// ```
/// assert_eq!(aoc::day_07::part_2().unwrap(), 249138943);
/// ```
pub fn part_2() -> Result<i64> {
    total_winnings(INPUT, true)
}

fn total_winnings(s: &str, use_jokers: bool) -> Result<i64> {
    let mut hands = s
        .lines()
        .map(|line| Hand::new(line, use_jokers))
        .collect::<Result<Vec<_>>>()?;
    hands.sort();
    let mut total_winnings = 0;
    for (i, hand) in hands.into_iter().enumerate() {
        total_winnings += (i64::try_from(i)? + 1) * hand.bid;
    }
    Ok(total_winnings)
}

#[derive(Debug)]
struct Hand {
    cards: [u32; 5],
    hand_type: HandType,
    bid: i64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl Hand {
    fn new(s: &str, use_jokers: bool) -> Result<Hand> {
        let (card_names, bid) = crate::str::split_once(s, ' ')?;
        let mut cards = [0; 5];
        for (name, card) in card_names.chars().zip(cards.iter_mut()) {
            *card = match name {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => {
                    if use_jokers {
                        1
                    } else {
                        11
                    }
                }
                'T' => 10,
                _ => name
                    .to_digit(10)
                    .ok_or_else(|| anyhow!("could not make digit: {}", name))?,
            };
        }
        Ok(Hand {
            hand_type: HandType::from(cards),
            cards,
            bid: bid.parse()?,
        })
    }
}

impl From<[u32; 5]> for HandType {
    fn from(cards: [u32; 5]) -> HandType {
        use HandType::*;

        let mut map: HashMap<u32, usize> = HashMap::new();
        for card in cards {
            if card != 1 {
                // Jokers aren't counted, they just make the counts work (it's magic!)
                let entry = map.entry(card).or_default();
                *entry += 1;
            }
        }
        let mut counts: Vec<_> = map.values().cloned().collect();
        counts.sort();
        counts.reverse();
        match counts.len() {
            0 | 1 => FiveOfAKind,
            2 => {
                if counts[1] == 2 {
                    FullHouse
                } else {
                    FourOfAKind
                }
            }
            3 => {
                if counts[1] == 2 {
                    TwoPair
                } else {
                    ThreeOfAKind
                }
            }
            4 => OnePair,
            5 => HighCard,
            _ => unreachable!(),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        use Ordering::*;

        let ordering = self.hand_type.cmp(&other.hand_type);
        if ordering == Equal {
            for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                let ordering = a.cmp(b);
                if ordering != Equal {
                    return ordering;
                }
            }
            Equal
        } else {
            ordering
        }
    }
}

#[test]
fn part_1_example() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    assert_eq!(total_winnings(input, false).unwrap(), 6440);
}

#[test]
fn enum_ordering() {
    assert!(HandType::FiveOfAKind > HandType::FourOfAKind);
}

#[test]
fn part_2_example() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    assert_eq!(total_winnings(input, true).unwrap(), 5905);
}
