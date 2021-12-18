use advent_of_code_2021::day13::{Points, Instruction};
use anyhow::Context;

use scan_fmt::scan_fmt;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/day13.txt")?;

    let (coords, inst) = input.split_once("\n\n").context("Invalid input")?;

    let mut points = Points::new();
    for line in coords.lines() {
        let (x, y) = scan_fmt!(line, "{d},{d}", i32, i32)?;
        points.insert((x, y));
    }

    let mut instructions: Vec<Instruction> = Vec::new();
    for line in inst.lines() {
        instructions.push(line.parse()?);
    }

    let folded = instructions[0].fold_points(&points);
    println!("{}", folded.len());

    Ok(())
}
