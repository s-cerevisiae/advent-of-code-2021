use std::fs;

use advent_of_code_2021::utils::abs_diff;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input/day7.txt")?;

    let mut pos: Vec<u32> = input.trim().split(',').map(|x| x.parse()).try_collect()?;
    pos.sort_unstable();

    let distance = |x| pos.iter().map(|&y| abs_diff(x, y)).sum();

    let median = pos[pos.len() / 2];
    let result: u32 = distance(median);

    println!("{:?}", result);

    Ok(())
}
