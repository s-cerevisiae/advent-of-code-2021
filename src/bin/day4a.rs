use std::{collections::HashSet, fs};

use advent_of_code_2021::day4::Board;
use anyhow::Context;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input/day4.txt")?;

    let mut lines = input.split("\n\n");

    let numbers_to_draw: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse())
        .try_collect()?;

    let boards: Vec<Board> = lines.map(|b| b.parse()).try_collect()?;

    let mut score = None;
    let mut drawn = HashSet::new();
    'outer: for d in numbers_to_draw {
        drawn.insert(d);
        for b in &boards {
            if b.is_winning(&drawn) {
                score = Some(b.score(&drawn, d));
                break 'outer;
            }
        }
    }

    let result = score.context("No one is ever winning")?;
    println!("{}", result);

    Ok(())
}
