//! String utilities.

use anyhow::{anyhow, Result};

/// Split a string at a character.
///
/// A wrapper around `std::str::split_once` that returns an error instead of an option.
///
/// # Examples
///
/// ```
/// let (a, b) = aoc::str::split_once("foo=bar", '=').unwrap();
/// assert_eq!(a, "foo");
/// assert_eq!(b, "bar");
/// ```
pub fn split_once(s: &str, delimiter: char) -> Result<(&str, &str)> {
    s.split_once(delimiter)
        .ok_or_else(|| anyhow!("could not find '{}' in {}", delimiter, s))
}
