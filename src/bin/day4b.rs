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

    let mut boards: Vec<Board> = lines.map(|b| b.parse()).try_collect()?;

    let mut score = None;
    let mut drawn = HashSet::new();
    for &d in &numbers_to_draw {
        drawn.insert(d);
        if boards.len() == 1 {
            let b = &boards[0];
            if b.is_winning(&drawn) {
                score = Some(b.score(&drawn, d));
                break;
            }
        } else {
            boards.retain(|b| !b.is_winning(&drawn));
        }
    }

    let result = score.context("No single last board")?;
    println!("{}", result);

    Ok(())
}
