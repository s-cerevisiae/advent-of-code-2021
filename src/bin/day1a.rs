use std::fs;

use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("input/day1.txt")?;

    let result = input
        .lines()
        .filter_map(|l| l.parse::<u32>().ok())
        .tuple_windows()
        .filter(|(x, y)| x < y)
        .count();

    println!("{}", result);
    Ok(())
}
