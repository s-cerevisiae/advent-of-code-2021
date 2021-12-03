use std::fs;

use advent_of_code_2021::day3::{
    Count,
    Which::{self, Least as CO2, Most as Oxygen},
};
use itertools::{FoldWhile::*, Itertools};

fn rating(diag: Vec<u32>, which: Which) -> u32 {
    (0..12)
        .rev()
        .fold_while(diag, |acc, digit| {
            if acc.len() == 1 {
                Done(acc)
            } else {
                let count = Count::at_digit(&acc, digit);
                Continue(
                    acc.into_iter()
                        .filter(|x| (x >> digit) & 1 == count.get_bit(which))
                        .collect(),
                )
            }
        })
        .into_inner()[0]
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input/day3.txt")?;

    let diagnostic = input
        .lines()
        .map(|l| u32::from_str_radix(l, 2))
        .collect::<Result<Vec<u32>, _>>()?;

    let oxygen = rating(diagnostic.clone(), Oxygen);
    let co2 = rating(diagnostic.clone(), CO2);

    println!("{}", oxygen * co2);
    Ok(())
}
