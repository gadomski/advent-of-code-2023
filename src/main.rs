use anyhow::Result;
use aoc::{day_01, day_02, day_03, day_04, day_05};

fn main() -> Result<()> {
    println!("Day 01 part 1: {}", day_01::part_1()?);
    println!("       part 2: {}", day_01::part_2()?);
    println!("Day 02 part 1: {}", day_02::part_1()?);
    println!("       part 2: {}", day_02::part_2()?);
    println!("Day 03 part 1: {}", day_03::part_1()?);
    println!("       part 2: {}", day_03::part_2()?);
    println!("Day 04 part 1: {}", day_04::part_1()?);
    println!("       part 2: {}", day_04::part_2()?);
    println!("Day 05 part 1: {}", day_05::part_1()?);
    println!("       part 2: {}", day_05::part_2()?);
    Ok(())
}
