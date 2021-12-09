use std::{collections::HashSet, fs};

use anyhow::Context;
use itertools::Itertools;

type Pat = HashSet<u8>;

fn pred<F: Fn(&Pat) -> bool>(pats: &[Pat], f: F) -> Option<&Pat> {
    pats.iter().find(|&s| f(s))
}

fn solve(pats: &[Pat], output: &[Pat]) -> Option<usize> {
    let one = pred(pats, |s| s.len() == 2)?;
    let seven = pred(pats, |s| s.len() == 3)?;
    let four = pred(pats, |s| s.len() == 4)?;
    let eight = pred(pats, |s| s.len() == 7)?;
    let nine = pred(pats, |s| s.len() == 6 && s.is_superset(four))?;
    let zero = pred(pats, |s| s.len() == 6 && s.is_superset(one) && s != nine)?;
    let six = pred(pats, |s| s.len() == 6 && s != nine && s != zero)?;
    let five = pred(pats, |s| s.len() == 5 && s.is_subset(six))?;
    let three = pred(pats, |s| s.len() == 5 && s.is_subset(nine) && s != five)?;
    let two = pred(pats, |s| s.len() == 5 && s != five && s != three)?;

    let answer: [&Pat; 10] = [zero, one, two, three, four, five, six, seven, eight, nine];

    let mut result = 0;
    for digit in output {
        let d = answer.iter().position(|s| *s == digit)?;
        result *= 10;
        result += d;
    }

    Some(result)
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input/day8.txt")?;

    let parse_item = |s: &str| {
        s.trim()
            .split_ascii_whitespace()
            .map(|i| i.bytes().collect::<HashSet<_>>())
            .collect_vec()
    };
    let parse_entry = |s: &str| {
        s.split_once('|')
            .map(|(pats, output)| (parse_item(pats), parse_item(output)))
    };
    let entries = input
        .lines()
        .map(parse_entry)
        .collect::<Option<Vec<_>>>()
        .context("Invalid input")?;

    let result: usize = entries.into_iter().filter_map(|(p, o)| solve(&p, &o)).sum();

    println!("{:?}", result);

    Ok(())
}
