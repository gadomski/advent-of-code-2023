use anyhow::{anyhow, Result};
use aoc::{
    day_01, day_02, day_03, day_04, day_05, day_06, day_07, day_08, day_09, day_10, day_11, day_12,
    day_13, day_14, day_15, day_16,
};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let all = match args.len() {
        1 => false,
        2 => {
            if args[1] == "--all" {
                true
            } else {
                return Err(anyhow!("invalid usage"));
            }
        }
        _ => return Err(anyhow!("invalid usage")),
    };
    println!("Day 01 part 1: {}", day_01::part_1()?);
    println!("       part 2: {}", day_01::part_2()?);
    println!("Day 02 part 1: {}", day_02::part_1()?);
    println!("       part 2: {}", day_02::part_2()?);
    println!("Day 03 part 1: {}", day_03::part_1()?);
    println!("       part 2: {}", day_03::part_2()?);
    println!("Day 04 part 1: {}", day_04::part_1()?);
    println!("       part 2: {}", day_04::part_2()?);
    println!("Day 05 part 1: {}", day_05::part_1()?);
    if all {
        println!("       part 2: {}", day_05::part_2()?);
    } else {
        println!("       part 2: skipped");
    }
    println!("Day 06 part 1: {}", day_06::part_1()?);
    println!("       part 2: {}", day_06::part_2()?);
    println!("Day 07 part 1: {}", day_07::part_1()?);
    println!("       part 2: {}", day_07::part_2()?);
    println!("Day 08 part 1: {}", day_08::part_1()?);
    println!("       part 2: {}", day_08::part_2()?);
    println!("Day 09 part 1: {}", day_09::part_1()?);
    println!("       part 2: {}", day_09::part_2()?);
    println!("Day 10 part 1: {}", day_10::part_1()?);
    println!("Day 11 part 1: {}", day_11::part_1()?);
    println!("       part 2: {}", day_11::part_2()?);
    println!("Day 12 part 1: {}", day_12::part_1()?);
    if all {
        println!("       part 2: {}", day_12::part_2()?);
    } else {
        println!("       part 2: skipped");
    }
    println!("Day 13 part 1: {}", day_13::part_1()?);
    println!("       part 2: {}", day_13::part_2()?);
    println!("Day 14 part 1: {}", day_14::part_1()?);
    println!("Day 15 part 1: {}", day_15::part_1()?);
    println!("       part 2: {}", day_15::part_2()?);
    println!("Day 16 part 1: {}", day_16::part_1()?);
    if all {
        println!("       part 2: {}", day_16::part_2()?);
    } else {
        println!("       part 2: skipped");
    }
    Ok(())
}
