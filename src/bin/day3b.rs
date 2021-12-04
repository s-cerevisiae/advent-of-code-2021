use std::fs;

use advent_of_code_2021::day3::{
    Count,
    Which::{self, Least as CO2, Most as Oxygen},
};

fn rating(mut diag: Vec<u32>, which: Which) -> u32 {
    for digit in (0..12).rev() {
        if diag.len() == 1 {
            break;
        }

        let count = Count::at_digit(&diag, digit);
        diag.retain(|x| (x >> digit) & 1 == count.get_bit(which));
    }
    diag[0]
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input/day3.txt")?;

    let diagnostic = input
        .lines()
        .map(|l| u32::from_str_radix(l, 2))
        .collect::<Result<Vec<u32>, _>>()?;

    let oxygen = rating(diagnostic.clone(), Oxygen);
    let co2 = rating(diagnostic, CO2);

    println!("{}", oxygen * co2);
    Ok(())
}
