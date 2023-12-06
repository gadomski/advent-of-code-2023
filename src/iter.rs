//! Iterator utilities.

use anyhow::{anyhow, Result};

/// Advance an iterator, but return an error instead of `None`.
///
/// # Examples
///
/// ```
/// let v = vec![0, 1, 2];
/// let mut iter = v.into_iter();
/// assert_eq!(aoc::iter::next(&mut iter).unwrap(), 0);
/// assert_eq!(aoc::iter::next(&mut iter).unwrap(), 1);
/// assert_eq!(aoc::iter::next(&mut iter).unwrap(), 2);
/// aoc::iter::next(iter).unwrap_err();
/// ```
pub fn next<T>(mut iter: impl Iterator<Item = T>) -> Result<T> {
    iter.next()
        .ok_or_else(|| anyhow!("could not advance iterator"))
}
