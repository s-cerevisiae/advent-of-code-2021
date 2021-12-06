use std::{fs, iter};

use itertools::Itertools;

// left this inefficient solution here to warn myself or to entertain others
fn fish_counts(day: u8) -> Vec<usize> {
    let mut fishes = vec![0];
    let mut counts = vec![1];
    for _ in 0..day {
        let mut new_fish_count = 0;
        for f in &mut fishes {
            if *f == 0 {
                *f = 6;
                new_fish_count += 1;
            } else {
                *f -= 1;
            }
        }
        fishes.extend(iter::repeat(8).take(new_fish_count));
        counts.push(fishes.len());
    }

    counts
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input/day6.txt")?;

    let fish_table = fish_counts(80);

    let fishes: Vec<u8> = input.trim().split(',').map(|x| x.parse()).try_collect()?;

    let result: usize = fishes.into_iter().map(|x| fish_table[80 - x as usize]).sum();

    println!("{:?}", result);

    Ok(())
}
