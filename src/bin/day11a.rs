use std::iter;

use advent_of_code_2021::day11::step;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/day11.txt")?;

    let mut grid = input
        .lines()
        .map(|l| l.chars().filter_map(|b| b.to_digit(10)).collect_vec())
        .collect_vec();

    let result: u32 = iter::repeat_with(|| step(&mut grid)).take(100).sum();

    println!("{:?}", result);

    Ok(())
}
