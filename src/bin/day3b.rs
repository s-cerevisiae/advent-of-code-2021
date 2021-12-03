use std::fs;

use advent_of_code_2021::day3::Count;
use itertools::{FoldWhile::*, Itertools};

fn oxygen_rating(diag: Vec<u32>) -> u32 {
    (0..12)
        .rev()
        .fold_while(diag, |acc, digit| {
            if acc.len() == 1 {
                Done(acc)
            } else {
                let count = Count::at_digit(&acc, digit);
                Continue(
                    acc.into_iter()
                        .filter(|x| (x >> digit) & 1 == count.most_common())
                        .collect(),
                )
            }
        })
        .into_inner()[0]
}

fn co2_rating(diag: Vec<u32>) -> u32 {
    (0..12)
        .rev()
        .fold_while(diag, |acc, digit| {
            if acc.len() == 1 {
                Done(acc)
            } else {
                let count = Count::at_digit(&acc, digit);
                Continue(
                    acc.into_iter()
                        .filter(|x| (x >> digit) & 1 == count.least_common())
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

    let oxygen = oxygen_rating(diagnostic.clone());
    let co2 = co2_rating(diagnostic.clone());

    println!("{:?}, {:?}", oxygen, co2);
    println!("{:?}", oxygen * co2);
    Ok(())
}
