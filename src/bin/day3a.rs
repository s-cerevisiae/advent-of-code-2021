use std::fs;

use advent_of_code_2021::day3::Count;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input/day3.txt")?;

    let diagnostic = input
        .lines()
        .map(|l| u32::from_str_radix(l, 2))
        .collect::<Result<Vec<u32>, _>>()?;

    let digit_counts = (0..12)
        .map(|i| Count::at_digit(&diagnostic, i))
        .collect_vec();

    let gamma = digit_counts
        .iter()
        .enumerate()
        .map(|(i, count)| count.most_common() << i)
        .fold(0, |acc, d| acc | d);

    let epsilon = digit_counts
        .iter()
        .enumerate()
        .map(|(i, count)| count.least_common() << i)
        .fold(0, |acc, d| acc | d);

    println!("{}", gamma * epsilon);
    Ok(())
}
