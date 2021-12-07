use std::fs;

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input/day7.txt")?;

    let mut pos: Vec<u32> = input.trim().split(',').map(|x| x.parse()).try_collect()?;
    pos.sort_unstable();

    let abs_diff = |x, y| if x > y { x - y } else { y - x };
    let distance = |x| pos.iter().map(|&y| abs_diff(x, y)).sum();

    let median = pos[pos.len() / 2];
    let result: u32 = distance(median);

    println!("{:?}", result);

    Ok(())
}
