use std::fs;

use anyhow::Context;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input/day7.txt")?;

    let pos: Vec<u32> = input.trim().split(',').map(|x| x.parse()).try_collect()?;

    let fuel = |n| n * (n + 1) / 2;
    let abs_diff = |x, y| if x > y { x - y } else { y - x };
    let distance = |x| pos.iter().map(|&y| fuel(abs_diff(x, y))).sum();

    let mean = pos.iter().sum::<u32>() / pos.len() as u32;

    let result: u32 = (mean - 1..=mean + 1)
        .map(distance)
        .min()
        .context("No enough elements")?;

    println!("{:?}", result);

    Ok(())
}
