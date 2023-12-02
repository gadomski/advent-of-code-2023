use anyhow::Result;
use aoc::{day_01, day_02};

fn main() -> Result<()> {
    println!("Day 01, part 1: {}", day_01::part_1()?);
    println!("Day 01, part 2: {}", day_01::part_2()?);
    println!("Day 02, part 1: {}", day_02::part_1()?);
    println!("Day 02, part 2: {}", day_02::part_2()?);
    Ok(())
}
