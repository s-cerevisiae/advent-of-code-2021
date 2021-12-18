use advent_of_code_2021::day14::{step, Polymer, Rule};
use anyhow::Context;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/day14.txt")?;

    let (template, rules) = input.split_once("\n\n").context("Invalid Input")?;

    let mut poly: Polymer = template.into();

    let rules: Vec<Rule> = rules.lines().map(|s| s.trim().parse()).try_collect()?;

    for _ in 0..10 {
        step(&mut poly, &rules);
    }

    let result = poly
        .elements
        .values()
        .minmax()
        .into_option()
        .map(|(min, max)| max - min);
    println!("{:?}", result);

    Ok(())
}
